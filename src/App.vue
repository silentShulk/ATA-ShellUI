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
<div id="root" class="flex-column">
    <Titlebar data-tauri-drag-region :page="page"/>
    <main id="router" class="flex-column p-10 gv-20">
        <RouterView />
    </main>
</div>
</template>



<style lang="scss">
#root {
    height: 100%;

    border-radius: 24px;
    overflow: hidden;
}

#router {
    flex: 1;
    
    background-color: $main;
}

#content {
    flex: 1;
}

.flex-column {
    display: flex;
    flex-direction: column;
}
.flex-row {
    display: flex;
    flex-direction: row
}

.flipped-h {
    transform: scaleX(-1);
}

*, *::before, *::after {
    box-sizing: border-box;
}

html, body, #app {
    margin: 0;
    padding: 0;
    height: 100vh;
}
</style>