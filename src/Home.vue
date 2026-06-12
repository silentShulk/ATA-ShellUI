<script setup lang="ts">
import { useRouter } from 'vue-router';

const router = useRouter();
</script>



<template>
<title> ATA </title>
 
<div id="home-page" class="ata-page">
    <header class="ata-header ata-colors">
        <h1 class="ata-title ata-spaceless"> ATA </h1>
        <h3 class="ata-title subtitle ata-spaceless"> Accord's Timeline Alterer </h3>
    </header>
 
    <main id="page-selector" class="ata-main ata-flex-column">
        <div id="feature-selector" class="ata-grid">
            <button id="btn-install" class="ata-btn ata-colors-accent ata-border-radius ata-shadow" @click="router.push('/install')"> Install a Mod </button>
            <button id="btn-list" class="ata-btn ata-colors-accent ata-border-radius ata-shadow" @click="router.push('/list')"> Mod List </button>
            <button id="btn-uninstall" class="ata-btn ata-colors-accent ata-border-radius ata-shadow" @click="router.push('/uninstall')"> Uninstall a mod </button>
            <button id="btn-browse" class="ata-btn ata-colors-accent ata-border-radius ata-shadow" @click="router.push('/browse')"> Browse Mods Online </button>
        </div>

        <button id="btn-settings" class="ata-btn ata-centered ata-colors-accent ata-border-radius" @click="router.push('/settings')"> Settings </button>
    </main>
 
    <footer id="credits-footer" class="ata-footer">
        <p id="creators" class="creators">
            Creators: <a href="https://github.com/silentShulk">silentShulk</a> & <a href="https://github.com/Beyluta">Beyluta</a>
        </p>
        <p id="special-thanks" class="special-thanks"> Special Thanks: </p>
        <p id="special-thanks-yokotaro" class="special-thanks"> Yoko Taro: for making the best game(s) ever </p>
        <p id="special-thanks-wouefulwolf" class="special-thanks"> WouefulWolf: for making N:A modding possible </p>
    </footer>
</div>
</template>



<style scoped lang="scss">
#feature-selector {
    grid-template-columns: 1fr 1fr;
    grid-template-rows: 1fr 1fr;
    grid-template-areas:
        "install list"
        "uninstall browse";
}

#btn-install, #btn-list, #btn-uninstall, #btn-browse, #btn-settings {
    font-size: 40px;
}
#btn-install  { grid-area: install; }
#btn-list     { grid-area: list; }
#btn-uninstall{ grid-area: uninstall; }
#btn-browse     { grid-area: browse; }

#credits-footer {
    background-color: $ata-accent-secondary;
    color: $ata-black;
}
.creators {
    margin: 10px 0px 10px 0px;
}
.special-thanks {
    margin: 3px 0px 3px 0px;
}
</style>