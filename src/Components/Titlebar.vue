<script setup lang="ts">
const props = defineProps<{
  page: string
}>()
</script>

<template>
<div id="titlebar" class="centered-h">
    <p class="ata-p ata-fonts-text">{{ props.page }}</p>
    <p class="ata-p ata-fonts-text">aaa</p>
    <div class="flex-row">
        <p class="ata-p ata-font-icons">arrow_drop_down</p>
        <p class="ata-p ata-font-icons">expand_content</p>
        <p class="ata-p ata-font-icons">close</p>
    </div>
</div>
</template>

<style scoped>
#titlebar {
    width: 100%;
    height: 32px;
    
    background: floralwhite;

    margin: 0;
    padding-left: 10px;
    padding-right: 20px;
}
</style>