<script setup lang="ts">
import { RouterView } from 'vue-router'
import TitleBar from './TitleBar.vue';
import { useSettingsStore } from './stores/settings'
import { onMounted } from 'vue'

const settingsStore = useSettingsStore()

onMounted(async () => {
    await settingsStore.load_settings()
})

</script>
 
 
<template>
<div id="ata-app">
    <TitleBar />
    <RouterView />
</div>
</template>
 
 
 
<style lang="scss">
#ata-app {
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
}

.ata-btn {
    margin: 5px;
    padding: 15px;

    cursor: pointer;
    
    transition: background-color 0.2s ease;
    &:hover {
        background-color: var(--dynamic-border-color, transparent);
    }
}
.ata-btn-small {
    margin: 5px;
    padding: 5px;

    cursor: pointer;

    transition: background-color 0.2s ease;
    &:hover {
        background-color: var(--dynamic-border-color, transparent);
    }
}
.ata-btn-back {
    border-radius: 5px;

    position: absolute;
    top: 50px;
    left: 15px;
    
    width: 30px;
    height: 30px;

    cursor: pointer;   
}

.ata-grid {
    display: grid;
    flex-grow: 1;
    grid-auto-flow: column;

    gap: 10px;
}

.ata-input-text {
    padding: 5px 5px 5px 5px;

    border-radius: 5px;

    font-size: 1em;

    resize: none;
}

.ata-select {
    -webkit-appearance: none;
    -moz-appearance: none;
    appearance: none;

    padding: 5px 30px 5px 10px;

    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='8' viewBox='0 0 12 8'%3E%3Cpolyline points='1,1 6,7 11,1' fill='none' stroke='%23d6cab2' stroke-width='2' stroke-linecap='round'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 10px center;
    background-size: 12px;
}

.ata-list {
    list-style: none;
    overflow-y: scroll;
}
.ata-list-item {
    border: 3px solid $ata-black;
}

.ata-checkbox {
    width: 30px;
    height: 30px;
}

.ata-truncate {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    word-wrap: normal;
}
.ata-spaceless {
    margin: 0;
    padding: 0;
}
.ata-border-radius {
    border-radius: 15px;
}
.ata-flex {
    display: flex;
    align-items: center;
    gap: 10px;
}
.ata-shadow {
    box-shadow: 0 1px 2px 1px $ata-black;

    &:hover {
        box-shadow: 0 1px 2px 1px $ata-black, 0 2px 3px 2px $ata-black-light;
    }
}
.ata-shadow-critical {
    box-shadow: $ata-accent-secondary-dark 0 5px 15px 5px;
}
.ata-shadow-black {
    box-shadow: $ata-black 0 2px 5px 2px;
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
.ata-centered-content {
    display: flex;
    align-items: center;
    justify-content: center;
}

// ATA-COLORS
.ata-colors {
    background-color: $ata-main;
    color: $ata-accent;
}
.ata-colors-accent {
    background-color: $ata-accent;
    color: $ata-main;

    --dynamic-border-color: #{$ata-accent-dark};
    
    border: 5px solid var(--dynamic-border-color);
}
.ata-colors-secondary {
    background-color: $ata-accent-secondary;
    color: $ata-accent-tertiary;

    --dynamic-border-color: #{$ata-accent-secondary-dark};
    
    border: 5px solid var(--dynamic-border-color);
}
.ata-colors-tertiary {
    background-color: $ata-accent-tertiary;
    color: $ata-accent-secondary;
    
    --dynamic-border-color: #{$ata-accent-tertiary-dark};
    
    border: 5px solid var(--dynamic-border-color);
}
.ata-colors-critical {
    background-color: $ata-accent-secondary;
    color: $ata-black;
    
    --dynamic-border-color: #{$ata-accent-secondary-dark};

    border: 5px solid var(--dynamic-border-color);
}
.ata-colors-black {
    background-color: $ata-black-light;
    color: white;
    
    --dynamic-border-color: #{$ata-black};

    border : 5px solid var(--dynamic-border-color);
}
.ata-colors-enabled {
    background-color: $ata-accent-tertiary;
    color: $ata-black;
    
    --dynamic-border-color: #{$ata-accent-tertiary-dark};

    border: 5px solid var(--dynamic-border-color);
}
.ata-colors-disabled {
    background-color: $ata-accent-secondary;
    color: $ata-black;
    
    --dynamic-border-color: #{$ata-accent-secondary-dark};

    border: 5px solid var(--dynamic-border-color);
}

body, html {
    margin: 0;
    padding: 0;
}
</style>