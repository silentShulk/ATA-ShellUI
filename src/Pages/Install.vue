<script setup lang="ts">
import { reactive } from 'vue'
import Console from '../Components/Console.vue'

enum StepProgress { NotDone, Doing, Done }

const steps = reactive([StepProgress.Doing, StepProgress.NotDone, StepProgress.NotDone, StepProgress.NotDone])

const icons = ['counter_1', 'counter_2', 'counter_3', 'counter_4']

const isActive = (i: number) =>
  steps[i] === StepProgress.Doing || steps[i] === StepProgress.Done

const lineHeight = (i: number) =>
  steps[i] === StepProgress.Doing ? '245px' : '60px'
</script>



<template>
<div id="title" class="flex-column align-center">
    <h1 class="ata-h1 ata-font-text">Install</h1>
</div>
<div id="content" class="flex-row justify-space-between">
    <div id="installation-steps" class="flex-column align-center justify-space-between">
        <template v-for="(icon, i) in icons" :key="i">
            <h1 class="ata-h1 ata-font-icons step"
                :class="isActive(i) ? 'accent-tertiary' : 'accent-secondary'">
                {{ icon }}
            </h1>
            <div v-if="i < icons.length - 1"
                 class="vl"
                 :style="{ height: lineHeight(i) }">
            </div>
        </template>
    </div>

    <Console class="accent-border"/>
</div>
</template>



<style scoped lang="scss">
.step { margin: 10px 0; }
.step:first-child { margin-top: 0; }
.step:last-child { margin-bottom: 0; }

.vl {
    width: 5px;
    flex-shrink: 0;              // stop flex squash
    background-color: $dark;
    border-radius: 500px;
    transition: height 0.3s ease; // smooth grow/shrink
}
</style>