<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';

const appWindow = getCurrentWindow();

const props = defineProps<{
  page: string
}>()

function randomTitle(): string {
  return 'aaa'
}
</script>

<template>
<div id="titlebar" class="align-items">
    <p id="page" class="ata-p ata-fonts-text">{{ props.page }}</p>

    <p id="name" class="ata-p ata-fonts-text">{{randomTitle()}}</p>
    
    <div id="buttons" class="flex-row">
        <btn id="hide" class="ata-h3 ata-font-icons ata-btn-titlebar" @click="appWindow.minimize()">arrow_drop_down</btn>
        <btn id="toggle-maximize" class="ata-h3 ata-font-icons ata-btn-titlebar" @click="appWindow.toggleMaximize()">expand_content</btn>
        <btn id="close" class="ata-h3 ata-font-icons ata-btn-titlebar" @click="appWindow.close()">close</btn>
    </div>
</div>
</template>

<style scoped lang="scss">
#titlebar {
    display: grid;
    grid-template-areas: "page name buttons";
    grid-template-columns: 1fr 1fr 1fr;
    
    width: 100%;
    height: 32px;
    
    background: floralwhite;

    margin: 0;
    padding-left: 10px;
    padding-right: 10px;
}
#page {
    justify-self: left;
}
#name {
    justify-self: center;
    font-style: italic;
}
#buttons {
    justify-self: right;
    gap: 10px;
}

#hide {
    background-color: $accent-tertiary;
}
#toggle-maximize {
    background-color: $main;
}
#close {
    background-color: $accent-secondary;
}
</style>