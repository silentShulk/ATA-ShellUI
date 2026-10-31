<script setup lang="ts">
import { onMounted, ref } from "vue";
import { commands, Mod, ModType } from "../bindings";

// Data
let mods = ref<Mod[]>([])

onMounted(async () => {
  mods.value = await commands.listMods()
})

// HTML related
const getIconaByTipo = (tipo: ModType) => {
  switch (tipo) {
    case "DLL": return "settings";
    case "Textures": return "texture";
    case "PlayerModels": return "player";
    case "WeaponModels": return "swords";
    case "WorldModels": return "public";
    case "CutsceneReplacements": return "video_library";
    case "ReshadePreset": return "light_mode"
  }
}
const getLabelByTipo = (tipo: ModType) => {
  switch (tipo) {
    case "DLL": return "Unique";
    case "Textures": return "Textures";
    case "PlayerModels": return "Pl. Models";
    case "WeaponModels": return "We. Models";
    case "WorldModels": return "Wo. Models";
    case "CutsceneReplacements": return "Cutscenes";
    case "ReshadePreset": return "RS Preset"
  }
}
</script>



<template>
<div id="title" class="flex-column align-center">
    <h1 class="ata-h1 ata-font-text">Manage</h1>
</div>

<div id="content" class="flex-column">
    <div id="buttons" class="flex-row justify-center g-40">
        <button class="ata-btn-small dark-secondary-border ata-h3 ata-font-icons accent-secondary">toggle_off</button>
        <button class="ata-btn-small dark-tertiary-border ata-h3 ata-font-icons accent-tertiary">toggle_on</button>
    </div>
    <div id="mod-list">
        <div class="flex-row g-20" v-for="mod in mods">
            <button class="ata-btn-small-long flex-row g-5" :class="mod.enabled ? 'accent-tertiary-border' : 'accent-secondary-border'">
                <h3 class="ata-h3 ata-font-icons">{{getIconaByTipo(mod.mod_type)}}</h3>
                <h3 class="ata-h3 ata-font-text">{{getLabelByTipo(mod.mod_type)}}</h3>
            </button>
            <h2 class="ata-h2 ata-font-text">{{mod.name}}</h2>
        </div>
    </div>
</div>
</template>



<style scoped lang="scss">
#mod-list {
    display: grid;

    grid-template-rows: repeat(4, 1fr);
    grid-template-columns: repeat(3, 1fr);

    margin: 35px 70px 25px 70px;
}
</style>