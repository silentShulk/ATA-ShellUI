<script setup>
</script>

<template>
    <div id="title" class="centered-h">
        <h1 class="ata-h1 ata-font-text">ATA</h1>
        <h2 class="ata-h2 ata-font-text">Accord's Timeline Alterer</h2>
    </div>
    <div id="features">
        <button id="install" class="ata-btn-big accent"></button>
        <button id="manage" class="ata-btn-big accent"></button>
        <button id="uninstall" class="ata-btn-big accent"></button>
        <button id="settings" class="ata-btn-big accent"></button>
    </div>
</template>

<style lang="scss">
#title {
    display: flex;
    flex-direction: column;
    
    gap: 100;
}

#features {
    display: grid;

    grid-template-areas: 
    'install manage'
    'uninstall settings';
}
#install {grid-area: "install";}
#manage {grid-area: "manage";}
#uninstall {grid-area: "uninstall";}
#settings {grid-area: "settings";}
</style>
