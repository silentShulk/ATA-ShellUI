<script setup>
import { invoke } from '@tauri-apps/api/core';

async function launch_automata() {
  await invoke('launch_automata')
}
</script>

<template>
    <div id="title" class="flex-column align-center">
        <h1 class="ata-h1 ata-font-text">ATA</h1>
        <h2 class="ata-h2 ata-font-text">Accord's Timeline Alterer</h2>
    </div>
    <div id="content">
        <div id="features-grid">
            <button id="install" class="ata-btn-big accent-border" @click="$router.push('/install')">
                <div class="flex-column centered-h main">
                    <h1 class="ata-h1 ata-font-icons">install_desktop</h1>
                    <h1 class="ata-h1 ata-font-text">Install</h1>
                </div>
            </button>
            <button id="manage" class="ata-btn-big accent-border" @click="$router.push('/manage')">
                <div class="flex-column main">
                    <h1 class="ata-h1 ata-font-icons">database</h1>
                    <h1 class="ata-h1 ata-font-text">Manage</h1>
                </div>
            </button>
            <button id="uninstall" class="ata-btn-big accent-border" @click="$router.push('/uninstall')">
                <div class="flex-column main">
                    <h1 class="ata-h1 ata-font-icons">delete</h1>
                    <h1 class="ata-h1 ata-font-text">Uninstall</h1>
                </div>
            </button>
            <button id="settings" class="ata-btn-big accent-border" @click="$router.push('/settings')">
                <div class="flex-column main">
                    <h1 class="ata-h1 ata-font-icons">settings</h1>
                    <h1 class="ata-h1 ata-font-text">Settings</h1>
                </div>
            </button>
        </div>
        <button id="launch" class="ata-btn-medium-big dark-secondary-border" @click="launch_automata()">
            <div class="flex-column">
                <h1 class="ata-h1 ata-font-icons accent-secondary">play_arrow</h1>
                <h1 class="ata-h1 ata-font-text main">Launch</h1>
            </div>
        </button>
    </div>
</template>

<style lang="scss">
#title {
    gap: 20;
}

#features-grid {
    display: grid;

    row-gap: 40px;
    column-gap: 40px;
    
    grid-template-columns: 1fr 1fr;
    grid-template-rows: 1fr 1fr;

    grid-template-areas: 
    'install manage'
    'uninstall settings';
}
#install   { grid-area: install;   justify-self: center; }
#manage    { grid-area: manage;    justify-self: center; }
#uninstall { grid-area: uninstall; justify-self: center; }
#settings  { grid-area: settings;  justify-self: center; }

#launch {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -10%);
}
</style>
