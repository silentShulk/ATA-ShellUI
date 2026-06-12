<script setup lang="ts">
import { nextTick, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import Console from './Components/Console.vue';
import { Mod } from './mods';
import { useRouter } from 'vue-router';



const router = useRouter();

const selectedFile = ref<string | null>(null);
const modName = ref<string | null>(null);

const installationStarted = ref<boolean>(false);
const installedMod = ref<Mod | null>(null);

async function browse() {
    selectedFile.value = await open({
        multiple: false,
        filters: [{ name: 'Monado', extensions: ['zip', '7z', 'rar']}]
    })
}

async function install() {
    installationStarted.value = true;

    await nextTick()

    installedMod.value = await invoke('install_mod_command', { compressedModFolderPath: selectedFile.value, answeredName: `${modName.value}`, forcedOverwrite: false});
}
</script>



<template>
<div id="install-page" class="ata-page">
    <header class="ata-header ata-colors">
        <h1 class="ata-title"> Install a Mod </h1>
    </header>

    <button class="ata-btn-back" @click="router.push('/')"> < </button>
    
    <main class="ata-main ata-flex-column">
        <div id="archive-selector" class="ata-flex">
            <h2> Select archive containing the mod (zip/7z/rar) </h2>
            <button class="ata-btn-small ata-border-radius ata-shadow ata-colors-accent" @click="browse()"> Browse </button>
        </div>
        
        <div id="archive-details" class="ata-flex" v-if="selectedFile">
            <div class="ata-flex-column">
                <strong> Path of mod folder: </strong>
                <p class="file-path ata-spaceless">{{ selectedFile }}</p>
            </div>
            <div class="ata-flex-column">
                <strong> Name for mod: </strong>
                <input class="ata-input-text ata-colors-accent ata-border-radius mod-name" type="text" v-model="modName"/>
            </div>
            <button class="ata-btn-small ata-colors-accent ata-border-radius ata-shadow" @click="install()" v-if="modName"> Install </button>
        </div>

        <Console v-show="installationStarted"/>
    </main>
</div>
</template>



<style scoped lang="scss">
.file-path {
    padding: 5px 5px 5px 5px;
    
    border-radius: 15px;
    
    background-color: $ata-accent-secondary;
    color: $ata-main;
}

.mod-name {
    font-weight: bold;
}
</style>