<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { MOD_TYPES, type Mod, type ModType } from '../bindings'

const props = defineProps<{ mods: Mod[] }>()

// v-model = names of selected mods (names are unique, uid can collide)
const selected = defineModel<string[]>({ default: () => [] })
const selectedSet = computed(() => new Set(selected.value))

function toggle(name: string) {
  // reassign, never push/splice, so defineModel emits
  selected.value = selectedSet.value.has(name)
    ? selected.value.filter(n => n !== name)
    : [...selected.value, name]
}

/* ---------- search + sort ---------- */
type SortKey = 'name' | 'type' | 'status' | 'date' | 'files'

const SORT_OPTIONS: { key: SortKey; label: string }[] = [
  { key: 'name',   label: 'Name' },
  { key: 'type',   label: 'Type' },
  { key: 'status', label: 'Status' },
  { key: 'date',   label: 'Install Date' },
  { key: 'files',  label: 'N. of files' },
]

const comparators: Record<SortKey, (a: Mod, b: Mod) => number> = {
  name:   (a, b) => a.name.localeCompare(b.name),
  type:   (a, b) => MOD_TYPES.indexOf(a.mod_type) - MOD_TYPES.indexOf(b.mod_type),
  status: (a, b) => Number(b.enabled) - Number(a.enabled),            // enabled first
  date:   (a, b) => Date.parse(b.install_date) - Date.parse(a.install_date), // newest first
  files:  (a, b) => b.files.length - a.files.length,                  // biggest first
}

const query = ref('')
const sort = ref<SortKey>('name')

const currentLabel = computed(() => SORT_OPTIONS.find(o => o.key === sort.value)!.label)

const visibleMods = computed(() => {
  const q = query.value.trim().toLowerCase()
  const filtered = q ? props.mods.filter(m => m.name.toLowerCase().includes(q)) : props.mods

  // copy before sort: sort() mutates, props must stay untouched
  return [...filtered].sort((a, b) => comparators[sort.value](a, b) || a.name.localeCompare(b.name))
})

/* ---------- sort dropdown ---------- */
const open = ref(false)
const sortBox = ref<HTMLElement | null>(null)
const toggleBtn = ref<HTMLButtonElement | null>(null)

function close() {
  open.value = false
  toggleBtn.value?.focus()
}
function pick(key: SortKey) {
  sort.value = key
  close()
}
function onPointerDown(e: PointerEvent) {
  if (open.value && !sortBox.value?.contains(e.target as Node)) open.value = false
}
onMounted(() => document.addEventListener('pointerdown', onPointerDown))
onUnmounted(() => document.removeEventListener('pointerdown', onPointerDown))

/* ---------- display helpers ---------- */
const TYPE_LABELS: Record<ModType, string> = {
  DLL: 'DLL',
  Textures: 'Textures',
  PlayerModels: 'Pl. models',
  WeaponModels: 'Wp. models',
  WorldModels: 'Wo. models',
  CutsceneReplacements: 'Cutscenes',
  ReshadePreset: 'Reshade',
}

const formatDate = (iso: string) =>
  new Date(iso).toLocaleDateString('en', { month: 'short', day: 'numeric' })
</script>



<template>
<div class="ata-select ata-border-big ata-border-solid ata-radius-big ata-shadow-big bd-accent-dark bg-accent-dark">
    <div class="select-header flex-row align-center justify-space-between ph-20 bg-accent">
        <label class="search flex-row align-center gh-10">
            <span class="ata-h1 ata-font-icons main" aria-hidden="true">search</span>
            <input v-model="query" type="text" class="search-input ata-h1 ata-font-text"
                   placeholder="Select" aria-label="Search mods by name">
        </label>

        <div ref="sortBox" class="sort" @keydown.esc="close">
            <div class="sort-box ata-border-small ata-border-solid ata-radius-medium bg-accent bd-accent-dark"
                 :class="{ 'ata-shadow-small': open }">
                <button ref="toggleBtn" type="button" class="sort-toggle unstyled-btn flex-row align-center gh-10 main"
                        aria-haspopup="menu" :aria-expanded="open" @click="open = !open">
                    <span class="ata-h3 ata-font-icons" aria-hidden="true">filter_alt</span>
                    <span class="ata-h3 ata-font-text">{{ currentLabel }}</span>
                </button>

                <div v-if="open" class="sort-options flex-column light" role="menu">
                    <button v-for="option in SORT_OPTIONS" :key="option.key" type="button"
                            role="menuitemradio" :aria-checked="option.key === sort"
                            class="ata-radio ata-p ata-font-text" @click="pick(option.key)">
                        {{ option.label }}
                    </button>
                </div>
            </div>
        </div>
    </div>

    <ul class="mod-list ata-scrollbar flex-column">
        <li v-for="mod in visibleMods" :key="mod.name">
            <button type="button" class="option"
                    :class="{ selected: selectedSet.has(mod.name) }"
                    :aria-pressed="selectedSet.has(mod.name)"
                    @click="toggle(mod.name)">
                <span class="info flex-column">
                    <span class="ata-h2 ata-font-text">{{ mod.name }}</span>
                    <span class="meta flex-row gh-5">
                        <span class="ata-h3 ata-font-text">{{ TYPE_LABELS[mod.mod_type] }}</span>
                        <span class="ata-h3 ata-font-text" aria-hidden="true">•</span>
                        <span class="ata-p ata-font-text">{{ mod.files.length }} Files · Installed {{ formatDate(mod.install_date) }}</span>
                    </span>
                </span>

                <span class="ata-badge ata-p ata-font-text"
                      :class="mod.enabled ? 'ata-badge-success' : 'ata-badge-danger'">
                    {{ mod.enabled ? 'Enabled' : 'Disabled' }}
                </span>
            </button>
        </li>

        <li v-if="visibleMods.length === 0" class="empty ata-p ata-font-text">No mods found</li>
    </ul>
</div>
</template>



<style scoped lang="scss">
.ata-select {
    width: 560px;
    height: 610px;

    display: flex;
    flex-direction: column;
    overflow: hidden; // clips list to the rounded inner corners
}

/* ---- header ---- */
.select-header {
    height: 70px;
    flex-shrink: 0;
    margin-bottom: 20px; // dark strip between header and list (container bg shows through)
}

.search {
    flex: 1;
    min-width: 0;
    cursor: text;
}
.search-input {
    flex: 1;
    min-width: 0;

    background: none;
    border: none;
    outline: none;

    color: $main;
    &::placeholder { color: $main; opacity: 0.85; }
}

/* ---- sort dropdown ---- */
// .sort keeps the header layout fixed; .sort-box floats above the list when it grows
.sort {
    position: relative;
    width: 180px;
    height: 45px;
    flex-shrink: 0;
}
.sort-box {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    z-index: 10;
    overflow: hidden; // rounds the options area
}
.sort-toggle {
    width: 100%;
    height: 39px; // 45 - 2 * 3px border
    padding: 0 10px;

    &:focus-visible {
        outline: 2px dashed $main;
        outline-offset: -4px;
    }
}
.sort-options {
    padding: 5px 10px;

    background: rgba($light, 0.2);
    border-top: 5px solid $accent-dark;

    .ata-radio { padding: 8px 0; }
}

/* ---- list ---- */
.mod-list {
    flex: 1;
    min-height: 0; // lets a flex child shrink so it scrolls instead of growing

    margin: 0;
    padding: 0; // overrides .ata-scrollbar's padding-left
    list-style: none;

    overflow-y: auto;
    overflow-x: hidden;

    background: $main;

    &::-webkit-scrollbar-track { background: transparent; }

    li + li { margin-top: -3px; } // merge the 3px borders of neighbours
}

.option {
    appearance: none;
    width: 100%;
    height: 90px;
    padding: 0 16px 0 20px;

    display: flex;
    justify-content: space-between;
    align-items: center;

    background: $main;
    color: $dark;
    border: 3px solid $dark;
    border-radius: 28px;

    text-align: left;
    cursor: pointer;
    transition: background-color 0.15s;

    &:hover { background: rgba($main-dark, 0.5); }
    &.selected { background: $main-dark; }

    &:focus-visible {
        outline: 3px dashed $accent-secondary;
        outline-offset: -8px;
    }
}

.meta { align-items: baseline; }

.empty {
    padding: 20px;
    text-align: center;
    color: $accent;
}
</style>
