<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { commands, MOD_TYPES, type Mod, type ModType } from '../bindings';
import { getLabelByTipo, getIconaByTipo, getColoreByTipo, getSfondoByTipo, getAccentoByTipo, getAccentoOppostoByTipo } from '../Functions/manage'

const activeFilters = ref<boolean[]>(MOD_TYPES.map(() => false))
const selectedTypes = computed<ModType[]>(() =>
  MOD_TYPES.filter((_, i) => activeFilters.value[i])
)

const mods = ref<Mod[]>([])
onMounted(async () => {
  mods.value = await commands.listModsCommand()
})

const orderedMods = computed(() => {
  const selected = selectedTypes.value
  const sortedMods: Mod[] = []
  let lastPushedIndex = 0

  for (const mod of mods.value) {
    if (selected.includes(mod.mod_type)) {
      sortedMods.splice(lastPushedIndex, 0, mod)
      lastPushedIndex++
    } else {
      sortedMods.push(mod)
    }
  }

  return sortedMods
})
</script>



<template>
<div id="title" class="flex-column align-center">
    <h1 class="ata-h1 ata-font-text">Manage</h1>
</div>

<div id="content" class="flex-row gh-40 ph-40">
    <div id="mods-filters" class="flex-column gv-20">
        <h2 class="ata-h2 ata-font-text">Filters</h2>
        <div id="filters" class="flex-column gv-20">
            <input type="text" class="ata-input-text-small-long ata-h3 ata-font-text accent-border main pl-10" placeholder="Search">
            <div id="filters-buttons" class="flex-column gv-20">
                <button v-for="(modType, i) in MOD_TYPES" :key="modType"
                class="flex-row ata-btn-small-long accent-border gh-10 filter"
                @click="activeFilters[i]=!activeFilters[i]"
                :class="{ 'ml-40': activeFilters[i] }">
                    <h3 class="ata-h3 ata-font-icons" :class="getColoreByTipo(modType)">{{ getIconaByTipo(modType) }}</h3>
                    <h3 class="ata-h3 ata-font-text main">{{ getLabelByTipo(modType) }}</h3>
                </button>
            </div>
        </div>
    </div>
    
    <div id="mods-list" class="ata-border-small ata-border-solid">
        <div id="mods-scroll" class="flex-column align-center ata-scrollbar gv-20 pb-20">
            <div v-for="mod in orderedMods" :key="mod.uid">
                <div class="flex-column ata-option-big align-center justify-space-between pv-5 option"
                :class="[mod.enabled ? 'ata-border-solid' : 'ata-border-dashed', getSfondoByTipo(mod.mod_type), {'ata-shadow-big-ts': mod.enabled}]"
                :style="{borderColor: getAccentoByTipo(mod.mod_type), opacity: mod.enabled ? 1 : 0.5}">
                    <h2 class="ata-h2 ata-font-text" :style="{color: getAccentoByTipo(mod.mod_type)}">{{mod.name}}</h2>
                    <div class="flex-row gh-10">
                        <div class="ata-radius-medium ph-5 pv-2"
                        :style="{background: getAccentoByTipo(mod.mod_type), color: getAccentoOppostoByTipo(mod.mod_type)}">
                            <div class="flex-row align-center gh-5">
                                <h3 class="ata-h3 ata-font-icons">files</h3>
                                <h3 class="ata-h3 ata-font-text">{{mod.files.length}} Files</h3>
                            </div>
                        </div>
                        <div class="ata-radius-medium ph-5 pv-2"
                        :style="{background: getAccentoByTipo(mod.mod_type), color: getAccentoOppostoByTipo(mod.mod_type)}">
                            <div class="flex-row align-center gh-5">
                                <h3 class="ata-h3 ata-font-icons">download</h3>
                                <h3 class="ata-h3 ata-font-text">38 Kbs</h3>
                            </div>
                        </div>
                        <div class="ata-radius-medium ph-5 pv-2"
                        :style="{background: getAccentoByTipo(mod.mod_type), color: getAccentoOppostoByTipo(mod.mod_type)}">
                            <div class="flex-row align-center gh-5">
                                <h3 class="ata-h3 ata-font-icons">calendar_today</h3>
                                <h3 class="ata-h3 ata-font-text">{{mod.install_date.split('T')[0]}}</h3>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
</template>



<style scoped lang="scss">
.filter {
    transition: margin 200ms ease
}

#mods-list {
    width: 560px;
    height: 580px;

    border-color: $dark;
    border-radius: 48px 48px 0 0;
    border-bottom: none;
    overflow: hidden;

    display: flex;
    align-self: flex-end;
    margin-bottom: -10px;
}

#mods-scroll {
    flex: 1;
    min-height: 0;
    overflow-y: scroll;
    overflow-x: hidden;

    margin-top: 20px;

    &::-webkit-scrollbar-track {
        background: transparent;
        margin-top: 30px;
    }
}

.option {
    border-radius: 28px;
}
</style>