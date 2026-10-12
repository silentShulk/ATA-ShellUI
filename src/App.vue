<script setup>
import { RouterView } from 'vue-router'
import Titlebar from './Components/Titlebar.vue';
</script>

<template>
<div id="root">
    <Titlebar data-tauri-drag-region page="Home"/>
    <main id="router">
        <RouterView />
    </main>
</div>
</template>

<style lang="scss">
*, *::before, *::after {
    box-sizing: border-box;
}

html, body, #app {
    margin: 0;
    padding: 0;
    height: 100%;
}

#root {
    display: flex;
    flex-direction: column;
    height: 100%;
}

#router {
    display: flex;
    flex-direction: column;
    flex: 1;
    
    min-height: 0;

    padding: 10px;
    gap: 20px;
    
    background-color: $main;
}

.flex-column {
    display: flex;
    flex-direction: column;
}
.flex-row {
    display: flex
}

.centered-h {
    display: flex;
    align-items: center;
}
.centered-v {
    display: flex;
    justify-content: center;
}
</style>