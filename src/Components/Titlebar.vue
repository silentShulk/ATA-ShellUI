<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { onMounted, ref } from 'vue';

const appWindow = getCurrentWindow();

const props = defineProps<{
  page: string
}>()

const TITLES: string[] = [
    'Glory to Mankind',
    'This cannot continue',
    'Become as gods',
    'Do you think games are silly little things?',
    'The future is not given to you...',
    '...It is something you must take for yourself',
    'Thank you... Nine... s',
    'It always ends like this',
    'I never quite realized...',
    '...How beautiful this world is',
    'Emotions are prohibited',
    'You want to **** 2B, don\'t you?',
    'The world is full of beautiful things',
    'I\'m not quite sure what it means to mourn',
    'I\'m coming, everyone... I\'m... coming...',
    'Humanity: Love. Family. Religion. War.',
    'Shut up... SHUT UP!',
    'Ha ha ha... ha ha ha hah!',
    'The possibility of a different future also exists',
    'Everything that lives is designed to end...',
    '...We are are perpetually trapped...',
    '...In a never-ending spiral of life and death...',
    '...Is this a curse? Or some kind of punishment?...',
    '...I often think about the god who blessed us with this cryptic puzzle...',
    '...And wonder if we\'ll ever have the chance to kill him',
    '...They are perpetually trapped...',
    '...However, life is all about the struggle within that cycle...',
    '...That is what \"we\" believe',

    'The future doesn\'t belong to you (neither does this phrase belong here)'
]
const title = ref("");

function randomizeTitle() {
    const randomTitle = TITLES[Math.floor(Math.random() * TITLES.length)];
    appWindow.setTitle(randomTitle)
    title.value = randomTitle;
}

onMounted(async () => {
  randomizeTitle()
})
</script>

<template>
<div id="titlebar" class="align-center">
    <h3 id="page" class="ata-h3 ata-font-text">{{ props.page }}</h3>

    <btn id="name" class="ata-p ata-fonts-text" @click=randomizeTitle()>{{title}}</btn>
    
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
    grid-template-columns: 0.9fr 1.2fr 0.9fr;
    
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
    user-select: none;
    cursor: pointer
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