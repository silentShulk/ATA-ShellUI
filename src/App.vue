<script setup lang="ts">
import { RouterView } from 'vue-router'
import TitleBar from './TitleBar.vue';
import { useSettingsStore } from './stores/settings'
import { onMounted } from 'vue'

const settingsStore = useSettingsStore()

onMounted(async () => {
    await settingsStore.load()
})

</script>
 
 
<template>
<div id="app">
    <TitleBar />
    <RouterView />
</div>
</template>
 
 
 
<style lang="scss">
#app {
    display: flex;
    flex-direction: column;    
    
    height: 100vh;
    width: 100%;
    border-radius: 10px 10px 20px 20px;
    
    background-color: $ata-main;
    font-family: Jetbrains Mono;

    overflow: hidden;
}

.ata-page {
    display: flex;
    flex-direction: column;

    width: 100%;
    height: 100%;
    min-height: 0;
}

.ata-header {
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    
    text-align: center;

    color: $ata-accent;
}
.ata-title {
    font-size: 40px;
}
.ata-title.subtitle {
    font-size: 25px;
}

.ata-main {
    display: flex;
    flex-grow: 1;

    min-height: 0;
    margin: 10px;

    gap: 10px;
}

.ata-footer {
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    
    width: 100%;
    
    justify-content: center;
    align-items: center;
    
    background-color: $ata-accent-secondary;
    color: $ata-black;
}

.ata-btn {
    border-radius: 15px;

    cursor: pointer;
}
.ata-btn-back {
    border-radius: 5px;

    position: absolute;
    top: 50px;
    left: 15px;
    
    width: 30px;
    height: 30px;

    background-color: $ata-accent-tertiary;
    color: $ata-black;

    cursor: pointer;   
}

.ata-grid {
    display: grid;
    flex-grow: 1;
    grid-auto-flow: column;

    padding: 5px 5px 5px 5px;
    
    gap: 10px;
}

.ata-input-text {
    padding: 5px 5px 5px 5px;
    
    border-radius: 5px;

    resize: none;
}

.ata-select {
    -webkit-appearance: none;
    -moz-appearance: none;
    appearance: none;

    padding: 5px 30px 5px 10px;
    border-radius: 5px;

    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='8' viewBox='0 0 12 8'%3E%3Cpolyline points='1,1 6,7 11,1' fill='none' stroke='%23d6cab2' stroke-width='2' stroke-linecap='round'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 10px center;
    background-size: 12px;
}

.ata-list {
    border: 5px solid transparent;
    border-radius: 15px;
    
    padding: 0;

    list-style: none;
    overflow-y: scroll;
}
.ata-list > * > * {
    border-right: 4px solid $ata-black;
    border-left: 4px solid $ata-black;
    border-top: 2px solid $ata-black;
    border-bottom: 2px solid $ata-black;
}

.ata-checkbox {
    width: 30px;
    height: 30px;
}

.ata-truncate {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}
.ata-spaceless {
    margin: 0;
    padding: 0;
}
.ata-flex {
    display: flex;
    align-items: center;
    gap: 10px;
}
.ata-flex-column {
    display: flex;
    flex-direction: column;
    gap: 5px;
}
.ata-centered {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
}
.ata-small {
    margin: 5px 5px 5px 5px;
    padding: 5px 5px 5px 5px;
}
.ata-centered-content {
    display: flex;
    align-items: center;
    justify-content: center;
}

.ata-colors {
    background-color: $ata-accent;
    color: $ata-main;
}
.ata-colors-inverted {
    background-color: $ata-main;
    color: $ata-accent;
}
.ata-colors-critical {
    background-color: $ata-accent-secondary;
    color: $ata-black;
    
    box-shadow: red 0px 5px 15px;
}

body, html {
    margin: 0;
    padding: 0;
}
</style>