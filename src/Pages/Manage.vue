<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { commands, type ModType, type Mod } from '../bindings';
import { getLabelByTipo } from '../Functions/manage'

const modTypes = ref<ModType[]>([]);
onMounted(async () => {
  modTypes.value = await commands.listTypes();
});

const activeFilters = ref<boolean[]>([false, false, false, false, false, false])

const mods = ref<Mod[]>([])
onMounted(async () => {
  mods.value = await commands.listMods()
})
</script>



<template>
<div id="title" class="flex-column align-center">
    <h1 class="ata-h1 ata-font-text">Manage</h1>
</div>

<div id="content" class="flex-row">
    <div id="mods-filters" class="flex-column gr-20">
        <h2 class="ata-h2 ata-font-text">Filters</h2>
        <div id="filters" class="flex-column gr-20">
            <div id="search">
                <select name="" id=""></select>
            </div>
            <div id="filters-buttons" class="flex-column gr-10">
                <button v-for="(modType, i) in modTypes" :key="modType"
                class="ata-h3 ata-font-text ata-btn-small-long" :class="activeFilters[i] ? 'accent-tertiary-border' : 'accent-secondary-border'"
                @click="activeFilters[i]=!activeFilters[i]">
                    {{ getLabelByTipo(modType) }}
                </button>
            </div>
        </div>
    </div>
    
    <div id="mods-list">
        <h2 v-for="mod in mods">{{ mod.name }}</h2>
    </div>
    
    <div id="mods-data"></div>
</div>
</template>



<style scoped lang="scss">
#mods-filters {
    width: 30vw;
}

#mods-list {
    width: 50vw;
    
    border: 2px solid $dark;
    border-radius: 24px 24px 0 0;
}

#mods-data {
    width: 20vw;
}
</style>