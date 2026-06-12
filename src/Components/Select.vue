<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { Fzf } from 'fzf';
import { Mod } from '../mods';

const mods = ref<Mod[]>([]);
const filteredMods = ref<Mod[]>([]);

const selectedMods = defineModel<Mod[]>({ required: true });

const filter = (e: Event) => {
    const fzf = new Fzf(mods.value, {
        selector: (mod: Mod) => mod.name,
        fuzzy: "v2"
    });
    filteredMods.value = fzf.find((e.target as HTMLInputElement).value).map(entry => entry.item);
};

const isSelected = (mod: Mod) => selectedMods.value.some(m => m.name === mod.name);

const toggleMod = (mod: Mod) => {
    const idx = selectedMods.value.findIndex(m => m.name === mod.name);
    if (idx !== -1) {
        selectedMods.value.splice(idx, 1);
    } else {
        selectedMods.value.push(mod);
    }
};

async function refresh() {
    mods.value = await invoke('list_mods_command');
    filteredMods.value = mods.value;
}

onMounted(refresh);

defineExpose({ refresh, mods });
</script>



<template>
<div id="selector" class="ata-flex-column">
    <input
        id="mod-filterer"
        class="ata-input-text ata-colors-accent"
        placeholder="Search mod..."
        @input="filter"
    />
    <ul id="mod-list" class="ata-list" :class="filteredMods.length > 0 ? 'gradient-border' : ''">
        <li v-for="(mod, index) in filteredMods" :key="mod.name">
            <label class="ata-btn ata-flex btn-mod" :class="index % 2 === 0 ? 'even' : 'odd'">
                <input type="checkbox" :checked="isSelected(mod)" @change="toggleMod(mod)"/>
                <span class="mod-name" :style="{fontSize: mod.name.length < 8 ? '3em' : '2em'}">{{ mod.name }}</span>
                <div class="ata-flex-column mod-details">
                    <span class="mod-type">{{ mod.modType }}</span>
                    <span>Install date: {{ mod.installDate }}</span>
                </div>
            </label>
        </li>
    </ul>
</div>
</template>



<style scoped lang="scss">
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
    border: 5px solid transparent;

    background-image:
        linear-gradient($ata-main, $ata-accent-dark),
        linear-gradient(180deg, $ata-main 0%, $ata-accent-dark 100%);

    background-clip: padding-box, border-box;
    background-origin: padding-box, border-box;
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
</style>
