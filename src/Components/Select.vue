<script setup lang="ts">
import { type Mod } from '../bindings'
import { getCortoByTipo } from '../Functions/uninstall';

const props = defineProps<{ mods: Mod[] }>()

const selected = defineModel<string[]>({ default: () => [] })

// HTML
const isSelected = (name: string) => {
  return selected.value.includes(name)
}

const toggleMod = (name: string) => {
  if (isSelected(name)) {
    selected.value.splice(selected.value.findIndex((n) => n === name))
  } else {
    selected.value.push(name)
  }
}

const formatInstallDate = (isoString: string): string => {
  const dateObj = new Date(isoString);
  if (Number.isNaN(dateObj.getTime())) return 'Invalid Date';

  const month = dateObj.toLocaleString('en-US', { month: 'short' });
  const day = String(dateObj.getDate()).padStart(2, '0');

  return `${month} ${day}`;
};

</script>



<template>
<div id="select" class="ata-select-big bg-main bd-accent-dark">
    <div class="ata-input-text-big-select flex-row bg-accent bd-accent-dark align-center gh-10">
        <h1 class="ata-h1 ata-font-icons main">search</h1>
        <input id="input-field" class="ata-h1 ata-font-text main bg-accent" type="text" placeholder="Search">
    </div>
    
    <div id="mod-list">
        <div v-for="mod in mods" :key="mod.name" @click="toggleMod(mod.name)"
        class="ata-option-big ata-radius-big flex-row align-center justify-space-between bd-dark pv-10 ph-20 gv-5"
        :class="{'bg-main-dark': isSelected(mod.name)}">
            <div class="flex-column">
                <h2 class="ata-h2 ata-font-text">{{mod.name}}</h2>
                <div class="flex-row gh-5 align-baseline">
                    <h3 class="ata-h3 ata-font-text">{{getCortoByTipo(mod.mod_type)}}</h3>
                    <h3 class="ata-h3 ata-font-text">-</h3>
                    <div class="flex-row gh-5">
                        <p class="ata-p ata-font-icons">files</p>
                        <p class="ata-p ata-font-text">{{mod.files.length}}</p>
                    </div>
                    <p class="ata-p ata-font-text">-</p>
                    <div class="flex-row gh-5">
                        <p class="ata-p ata-font-icons">download</p>
                        <p class="ata-p ata-font-text">38kb</p>
                    </div>
                    <p class="ata-p ata-font-text">-</p>
                    <div class="flex-row gh-5">
                        <p class="ata-p ata-font-icons">calendar_today</p>
                        <p class="ata-p ata-font-text">{{formatInstallDate(mod.install_date)}}</p>
                    </div>
                </div>
            </div>
            <div class="flex-row align-center ata-radius-big ata-border-smaller ata-border-solid bd-dark p-5 gh-5">
                <span class="ata-h3 ata-font-icons accent-tertiary">{{mod.enabled ? "radio_button_checked" : "radio_button_unchecked"}}</span>
                <p class="ata-p ata-font-text">{{mod.enabled ? "Enabled" : "Disabled"}}</p>
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
