<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { Mod } from './mods';
import { invoke } from '@tauri-apps/api/core';
import { useRouter } from 'vue-router';
import { Fzf } from 'fzf';


const router = useRouter();

const mods = ref<Mod[]>([]);

const filteredMods = ref<Mod[]>([]);
const selectedMods = ref<Mod[]>([]);
const currentIndex = ref(0);

const uninstalling = ref(false);

const filterMods = (e: Event) => {
    const fzf = new Fzf(
        mods.value,
        {
            selector: (mod: Mod) => mod.name,
            fuzzy: "v2"
        })

    const result = fzf.find((e.target as HTMLInputElement).value)
    filteredMods.value = result.map(entry => entry.item)
};

const isSelected = (mod: Mod) => selectedMods.value.some(m => m.name === mod.name);

const toggleMod = (mod: Mod) => {
    const idx = selectedMods.value.findIndex(m => m.name === mod.name);
    if (idx !== -1) {
        selectedMods.value.splice(idx, 1);
        if (currentIndex.value >= selectedMods.value.length) {
            currentIndex.value = Math.max(0, selectedMods.value.length - 1);
        }
    } else {
        selectedMods.value.push(mod);
    }
};

async function uninstall() {
    for (const mod of selectedMods.value) {
        await invoke('uninstall_mod', { modName: mod.name });
    }
    
    selectedMods.value = [];
    currentIndex.value = 0;
    uninstalling.value = false;
    
    mods.value = await invoke('list_mods');
    filteredMods.value = mods.value;
}

onMounted(async () => {
    mods.value = await invoke('list_mods');

	filteredMods.value = mods.value;
});
</script>



<template>
<div class="ata-page">
    <header class="ata-header">
        <h1 class="ata-title"> Uninstall a Mod </h1>
    </header>

    <button class="ata-btn-back" @click="router.push('/')"> < </button>

    <main id="mod-selection" class="ata-flex" v-if="mods.length > 0">
        <div id="selector" class="ata-flex-column">
            <input 
                id="mod-filterer"
                class="ata-input-text ata-colors"
                placeholder="Search mod..."
                @input="(e) => filterMods(e)"
            />
            <ul id="mod-list" class="ata-list" :class="filteredMods.length > 0 ? 'gradient-border' : ''">
                <li v-for="(mod, index) in filteredMods" :key="mod.name">
                    <label class="ata-btn ata-flex btn-mod" :class="index % 2 === 0 ? 'even' : 'odd'">
                        <input type="checkbox" :checked="isSelected(mod)" @change="toggleMod(mod)"/>
                        <span class="mod-name" :style="{fontSize: mod.name.length<8 ? '3em' : '2em'}">{{mod.name}}</span>
                        <div class="ata-flex-column mod-details">
                            <span class="mod-type">{{mod.modType}}</span>
                            <span>Install date: {{mod.installDate}}</span>
                        </div>
                    </label>
                </li>
            </ul>
        </div>

        <div id="summary" v-if="selectedMods.length > 0">
            <nav id="selection" class="ata-colors ata-centered-content">
                <button class="btn-change-mod" :style="{color: 'var(--ata-accent-tertiary)'}"
                @click="() => { if (currentIndex !== 0) currentIndex-- }"
                v-if="selectedMods.length >= 2">
                    ‹
                </button>
                <div class="ata-flex-column  mod-details ata-truncate">
                    <h1 class="mod-detail" :style="{color: 'black', fontSize: selectedMods[currentIndex].name.length<8 ? '2em' : '1.5em'}">{{ selectedMods[currentIndex].name }}</h1>
                    <span class="mod-detail">{{ selectedMods[currentIndex].modType }}</span>
                    <span class="mod-detail">Install date: {{ selectedMods[currentIndex].installDate }}</span>
                    <span class="mod-detail">Enabled: {{ selectedMods[currentIndex].enabled }}</span>
                    <span class="mod-detail">UID: {{ selectedMods[currentIndex].uid }}</span>
                </div>
                <button class="btn-change-mod" :style="{color: 'var(--ata-accent-secondary)'}"
                @click="() => { if (currentIndex !== selectedMods.length - 1) currentIndex++ }"
                v-if="selectedMods.length >= 2">
                    ›
                </button>
            </nav>

            <button id="btn-uninstall-first" class="ata-btn" @click="uninstalling = true"> Uninstall all selected mods? </button>
        </div>
    </main>
    
    <main v-else class="ata-main ata-centered-content ata-flex-column">
        <h1 class=""> No mods installed! </h1>
        <button class="ata-btn ata-colors btn-install" @click="router.push('/install')">
            <h1> <u>Install some mods here</u> </h1>
        </button>
    </main>
    
    <div id="uninstall-confirm" v-if="(selectedMods.length > 0) && uninstalling">
        <button class="ata-btn ata-colors-critical btn-uninstall-confirm" @click="uninstall()">
            <h2> Are you sure you want to uninstall {{ selectedMods.length }} mod(s)? </h2>
        </button>
    </div>
</div>
</template>



<style scoped lang="scss">
#mod-selection {
    max-height: 60vh;
}

#selector {
    width: 60%;

    max-height: 90%;

    align-items: center;
}

#mod-filterer {
    width: 60%;
    height: 15%;

    font-size: 2.5em;
}
#mod-list {
    width: 75%;
    max-height: 70%;
}
.gradient-border {
    background: linear-gradient(transparent) padding-box,
                linear-gradient(180deg, $ata-main 0%, $ata-main 20%, $ata-accent 100%) border-box;
}

.btn-mod {
    padding: 5px 10px 5px 5px;
    height: 75px;
}
.even {
    background-color: $ata-accent-secondary;
}
.odd {
    background-color: $ata-accent-tertiary;
}
.btn-mod:has(input:checked) {
    background-color: $ata-accent;
}

.mod-name {
    align-self: center;

    max-width: 175px;
}
.mod-details {
    justify-content: space-between;
}
.mod-type {
    font-size: 1.5em;
}

#summary {
    width: 30%;
    height: 90%;
}

#selection {
    margin: 125px 50px 0 0;
    padding: 5px;
    height: 40%;

    border: 5px solid $ata-black;
    border-radius: 15px;
}
.mod-detail {
    margin: 2px;
}
.btn-change-mod {
    background-color: transparent;
    border: none;

    font-size: 2em;
}

#btn-uninstall-first {
    height: 50px;
    margin: 25px 50px 0 0;

    background-color: $ata-accent-tertiary;
    color: $ata-black;
    font-size: 1.3em;
    border: 4px solid $ata-accent-secondary;

}

#uninstall-confirm {
    display: flex;
    justify-content: center;

    flex-grow: 1;
}
.btn-uninstall-confirm {
    width: 20%;
    margin-bottom: 25px;
}

.btn-install {
    width: 20%;
    height: 20%;
}
</style>