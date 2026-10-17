<script setup lang="ts">
import { reactive } from 'vue'
import Console from '../Components/Console.vue'

const steps = reactive([false, false, false, false])
</script>



<template>
<div id="title" class="flex-column align-center">
    <h1 class="ata-h1 ata-font-text">Install</h1>
</div>
<div id="content" class="flex-row justify-space-between">
    <div id="installation-steps flex-column">
        <h1 id="step-1" class="ata-h1 ata-font-icons" :class="steps[0] ? 'accent-tertiary' : 'accent-secondary'">counter_1</h1>
        <div class="vl"></div>
        <h1 id="step-2" class="ata-h1 ata-font-icons" :class="steps[1] ? 'accent-tertiary' : 'accent-secondary'">counter_2</h1>
        <div class="vl"></div>
        <h1 id="step-3" class="ata-h1 ata-font-icons" :class="steps[2] ? 'accent-tertiary' : 'accent-secondary'">counter_3</h1>
        <div class="vl"></div>
        <h1 id="step-4" class="ata-h1 ata-font-icons" :class="steps[3] ? 'accent-tertiary' : 'accent-secondary'">counter_4</h1>
    </div>
    <Console class="accent-border"/>
</div>
</template>



<style scoped lang="scss">
.vl {
    border-left: 5px solid $dark;
    flex: 1
}
</style>