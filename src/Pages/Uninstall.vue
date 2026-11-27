<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { commands, Mod } from '../bindings';
import Select from '../Components/Select.vue';

const mods = ref<Mod[]>([])
onMounted(async () => {
  mods.value = await commands.listModsCommand();
})

const selectedMods = ref<string[]>([])
</script>



<template>
    <div class="flex-row align-center justify-space-between">
        <Select v-model="selectedMods" :mods="mods" ></Select>
        <div v-if="selectedMods.length > 0">
            sigma sigma
        </div>
    </div>
</template>