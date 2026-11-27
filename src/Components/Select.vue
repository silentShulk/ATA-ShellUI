<script setup lang="ts">
import { type Mod } from '../bindings'

const props = defineProps<{ mods: Mod[] }>()

const selected = defineModel<string[]>({ default: () => [] })

</script>



<template>
<div id="select" class="ata-select-big bg-main bd-accent-dark">
    <div class="ata-input-text-big-select flex-row bg-accent bd-accent-dark align-center gh-10">
        <h1 class="ata-h1 ata-font-icons main">search</h1>
        <input id="input-field" class="ata-h1 ata-font-text main bg-accent" type="text" placeholder="Search">
    </div>
    
    <div id="mod-list">
        <div v-for="mod in mods" :key="mod.name" class="ata-option-big ata-radius-big flex-row align-center justify-space-between bd-dark pv-10 ph-20 gv-5"> 
            <div class="flex-column">
                <h2 class="ata-h2 ata-font-text">Sigma</h2>
                <div class="flex-row gh-5 align-baseline">
                    <h3 class="ata-h3 ata-font-text">Pl. Models</h3>
                    <h3 class="ata-h3 ata-font-text">-</h3>
                    <p class="ata-p ata-font-text">3 files</p>
                    <p class="ata-p ata-font-text">-</p>
                    <p class="ata-p ata-font-text">Installed Oct 2</p>
                </div>
            </div>
            <div class="flex-row align-center ata-radius-big ata-border-smaller ata-border-solid bd-dark p-5 gh-5">
                <span class="ata-h3 ata-font-icons accent-tertiary">radio_button_checked</span>
                <p class="ata-p ata-font-text">Enabled</p>
            </div>
        </div>
    </div>
</div>
</template>



<style scoped lang="scss">
#input-field {
    border: none
}
</style>
