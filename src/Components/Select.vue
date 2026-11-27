<script setup lang="ts">
import { type Mod } from '../bindings'

const props = defineProps<{ mods: Mod[] }>()

const selected = defineModel<string[]>({ default: () => [] })

</script>



<template>
<div id="select" class="ata-select-big bg-main bd-accent-dark">
    <input id="input-field" class="ata-input-text-big-select ata-h1 bg-accent bd-accent-dark" type="text">
    <div id="mod-list">
        
    </div>
</div>
</template>



<style scoped lang="scss">

</style>
