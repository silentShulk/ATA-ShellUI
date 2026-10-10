<script setup>
</script>

<template>
    <div id="title" class="flex-column centered-h">
        <h1 class="ata-h1 ata-font-text">ATA</h1>
        <h2 class="ata-h2 ata-font-text">Accord's Timeline Alterer</h2>
    </div>
    <div id="features">
        <div id="features-grid">
            <button id="install" class="ata-btn-big accent">
                <div class ="flex-column centered-h centered-v">
                    <h1 class="ata-h1 ata-font-icons">install_desktop</h1>
                    <h1 class="ata-h1 ata-font-text">Install</h1>
                </div>
            </button>
            <button id="manage" class="ata-btn-big accent">
                <div class ="flex-column centered-h centered-v">
                    <h1 class="ata-h1 ata-font-icons">database</h1>
                    <h1 class="ata-h1 ata-font-text">Manage</h1>
                </div>
            </button>
            <button id="uninstall" class="ata-btn-big accent">
                <div class ="flex-column centered-h centered-v">
                    <h1 class="ata-h1 ata-font-icons">delete</h1>
                    <h1 class="ata-h1 ata-font-text">Uninstall</h1>
                </div>
            </button>
            <button id="settings" class="ata-btn-big accent">
                <div class ="flex-column centered-h centered-v">
                    <h1 class="ata-h1 ata-font-icons">settings</h1>
                    <h1 class="ata-h1 ata-font-text">Settings</h1>
                </div>
            </button>
        </div>
        <button id="launch" class="ata-btn-medium-big dark-red">
            <div class ="flex-column centered-h centered-v">
                <h1 class="ata-h1 ata-font-icons">play_arrow</h1>
                <h1 class="ata-h1 ata-font-text">Launch</h1>
            </div>
        </button>
    </div>
</template>

<style lang="scss">
#title {
    gap: 20;
}

#features {
    
}
#features-grid {
    display: grid;

    row-gap: 20px;
    column-gap: 20px;
    
    grid-template-columns: 1fr 1fr;
    grid-template-rows: 1fr 1fr;

    grid-template-areas: 
    'install manage'
    'uninstall settings';
}
#install {grid-area: "install";}
#manage {grid-area: "manage";}
#uninstall {grid-area: "uninstall";}
#settings {grid-area: "settings";}

#launch {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -10%);
}
</style>
