<script setup lang="ts">
import { RouterView } from 'vue-router'
import Titlebar from './Components/Titlebar.vue';
import { ref, watch } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()

const page = ref()

watch(
  () => route.path,
  (path) => setNewPageName(path),
  { immediate: true }
)

function setNewPageName(path: string) {
  if (path === '/' || path === '') {
    page.value = 'Home'
  } else {
    const cleanName = path.replace('/', '')
    page.value = cleanName.charAt(0).toUpperCase() + cleanName.slice(1)
  }
}
</script>



<template>
<div id="root">
    <Titlebar data-tauri-drag-region :page="page"/>
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

#title {
    gap: 20;
}

.flex-column {
    display: flex;
    flex-direction: column;
}
.flex-row {
    display: flex;
    flex-direction: row
}

.justify-center {
    display: flex;
    justify-items: center;
}
.align-center {
    display: flex;
    align-items: center;
}

.justify-self {
    display: flex;
    justify-self: center;
}
.align-self {
    display: flex;
    align-self: center;
}
</style>