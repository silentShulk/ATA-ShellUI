<script setup>
import { RouterView } from 'vue-router'
import Titlebar from './Components/Titlebar.vue';
</script>

<template>
<div id="root">
    <Titlebar />
    <RouterView id="router"/>
</div>
</template>

<style>
#root {
    display: flex;
    flex-direction: column;
    height: 100%;
}
#router {
    display: flex;
    flex-direction: column;
    min-height: 0
}

.centered-v {
    display: flex;
    align-items: center
}

html, body{
    margin: 0;
    padding: 0;
    height: 100%;
}
</style>