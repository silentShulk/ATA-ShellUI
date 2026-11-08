<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { commands, Mod, ModType } from "../bindings";

const mods = ref<Mod[]>([])
onMounted(async () => {
  mods.value = await commands.listMods()
})

const switchAll = (state: boolean) => {
  mods.value.forEach(mod => mod.enabled = state)
}

const ROWS = 4
const COLS = 3
const PAGE_SIZE = ROWS * COLS

const pageIndex = ref(0)
const direction = ref(1)
watch(pageIndex, (next, prev) => {
  direction.value = next > prev ? 1 : -1
})

const pageCount = computed(() => Math.max(1, Math.ceil(mods.value.length / PAGE_SIZE)))

const pagedMods = computed(() =>
  mods.value.slice(pageIndex.value * PAGE_SIZE, (pageIndex.value + 1) * PAGE_SIZE)
)

const getIconaByTipo = (tipo: ModType) => {
  switch (tipo) {
    case "DLL": return "settings";
    case "Textures": return "texture";
    case "PlayerModels": return "person";
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
    case "PlayerModels": return "Player";
    case "WeaponModels": return "Weapons";
    case "WorldModels": return "World";
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
    <div id="buttons" class="flex-row justify-center gc-40">
        <button class="ata-btn-small dark-secondary-border ata-h3 ata-font-icons accent-secondary"
            @click="switchAll(false)">toggle_off</button>
        <button class="ata-btn-small dark-tertiary-border ata-h3 ata-font-icons accent-tertiary"
            @click="switchAll(true)">toggle_on</button>
    </div>

    <div id="pager" class="flex-row align-center">
        <button
            :style="{ visibility: pageIndex > 0 ? 'visible' : 'hidden' }"
            class="ata-h2 ata-font-icons accent-secondary flipped-h unstyled-btn"
            @click="pageIndex--">next_plan</button>

        <Transition name="slide" mode="out-in">
            <div id="mod-list" :key="pageIndex" class="mv-40 gr-100 gc-40">
                <div class="mod-cell flex-row align-center gc-10" v-for="mod in pagedMods" :key="mod.name">
                    <button class="ata-btn-small-long flex-row gc-10 mod-btn"
                        @click="mod.enabled = !mod.enabled"
                        :class="mod.enabled ? 'accent-tertiary-border' : 'accent-secondary-border'">
                        <h3 class="ata-h3 ata-font-icons">{{ getIconaByTipo(mod.mod_type) }}</h3>
                        <h3 class="ata-h3 ata-font-text">{{ getLabelByTipo(mod.mod_type) }}</h3>
                    </button>
                    <h2 class="ata-h2 ata-font-text mod-name" :title="mod.name">{{ mod.name }}</h2>
                </div>
            </div>
        </Transition>

        <button
            :style="{ visibility: pageIndex < pageCount - 1 ? 'visible' : 'hidden' }"
            class="ata-h2 ata-font-icons accent-tertiary unstyled-btn"
            @click="pageIndex++">next_plan</button>
    </div>
</div>
</template>



<style scoped lang="scss">
#pager {
    flex: 1;
    min-height: 0;
    overflow: hidden;
}

#mod-list {
    flex: 1;
    min-width: 0;
    min-height: 0;

    display: grid;
    grid-auto-flow: column;
    grid-template-rows: repeat(v-bind(ROWS), minmax(0, 1fr));
    grid-template-columns: repeat(v-bind(COLS), minmax(0, 1fr));
}

.mod-cell {
    min-width: 0;
}

.mod-btn {
    flex-shrink: 0;
}

.mod-name {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.slide-enter-active,
.slide-leave-active {
    transition: transform 0.2s ease;
}
.slide-enter-from {
    transform: translateX(calc(v-bind(direction) * 100%));
}
.slide-leave-to {
    transform: translateX(calc(v-bind(direction) * -100%));
}
</style>