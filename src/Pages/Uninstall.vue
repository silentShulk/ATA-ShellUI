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
<div>
    <Select v-model="selectedMods" :mods="mods" ></Select>
</div>
</template>