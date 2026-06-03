<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { Mod } from './mods';
import { invoke } from '@tauri-apps/api/core';
import { useRouter } from 'vue-router';

const router = useRouter();

const mods = ref<Mod[]>([]);

const isCompact = computed(() => mods.value.length >= 15);

const enableMod = async (modName: string) => {
    await invoke('enable_mod', { modName: modName });
    mods.value = await invoke('list_mods');
};
const disableMod = async (modName: string) => {
    await invoke('disable_mod', { modName: modName });
    mods.value = await invoke('list_mods');
};

onMounted(async () => {
    mods.value = await invoke('list_mods');
});
</script>



<template>
<div id="list-page" class="ata-page">
    <header class="ata-header">
        <h1 class="ata-title"> Installed Mods </h1>
        <h1 class="ata-title subtitle"> N. {{ mods?.length ?? 0 }} </h1>
    </header>

    <button class="ata-btn-back" @click="router.push('/')"> < </button>
    
    <main id="mods-list" class="ata-main ata-grid" :class="isCompact ? 'mod-grid-compact' : 'mod-grid'">
        <div :id="`mod-${ index + 1 }`" class="ata-flex" v-for="(mod, index) in mods" :key="index">
            <h3 class="mod-index">{{ index + 1 }}.</h3>
            <button class='ata-btn ata-small' :class="mod.enabled ? 'btn-enabled' : 'btn-disabled'"
            @click="mod.enabled ? disableMod(mod.name) : enableMod(mod.name)">
                <h2 class="mod-name">{{ mod.name }}</h2>
            </button> 
            <h3>{{ mod.modType }}</h3>
        </div>
    </main>
</div>
</template>



<style scoped lang="scss">
.mod-grid {
    grid-template-columns: repeat(3, 1fr);
    grid-template-rows: repeat(5, 1fr);

    padding: 10px 10px 10px 30px;
}
.mod-grid-compact {
    grid-template-columns: repeat(4, 1fr);
    grid-template-rows: repeat(7, 1fr);

    padding: 10px 10px 10px 20px;
}

.mod-name {
    margin: 5px 5px 5px 5px;
}

.btn-enabled {
    background-color: $ata-accent-tertiary;
}
.btn-disabled {
    background-color: $ata-accent-secondary;
}
</style>