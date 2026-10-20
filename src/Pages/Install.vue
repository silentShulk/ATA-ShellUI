<script setup lang="ts">
import { reactive } from 'vue'
import Console from '../Components/Console.vue'

enum StepProgress { NotDone, Doing, Done }

const steps = reactive([StepProgress.Doing, StepProgress.NotDone, StepProgress.NotDone, StepProgress.NotDone])
const icons = ['counter_1', 'counter_2', 'counter_3', 'counter_4']

const isActive = (i: number) =>
  steps[i] === StepProgress.Doing || steps[i] === StepProgress.Done
const lineHeight = (i: number) => {
  if (steps.every(x => x == StepProgress.Done)) return '122px'
  else return (steps[i] === StepProgress.Doing ? '245px' : '60px')
}

const goTo = (i: number) => {
  steps.fill(StepProgress.Done, 0, i)
  steps[i] = i === steps.length - 1 ? StepProgress.Done : StepProgress.Doing
  steps.fill(StepProgress.NotDone, i + 1)
}
</script>



<template>
<div id="title" class="flex-column align-center">
    <h1 class="ata-h1 ata-font-text">Install</h1>
</div>
<div id="content" class="flex-row justify-space-between">
    <div id="installation-steps" class="flex-column justify-space-between">
        <template v-for="(icon, i) in icons" :key="i">
            <btn class="ata-h1 ata-font-icons"
                :class="isActive(i) ? 'accent-tertiary' : 'accent-secondary'"
                @click="goTo(i)">
                {{ icon }}
            </btn>
            <div class="flex-row">
                <div v-if="i < icons.length - 1"
                    class="vl"
                    :style="{ height: lineHeight(i) }">
                </div>
                <Transition name="grow">
                    <div v-if="steps[i] === StepProgress.Doing" class="step-content align-self-center">
                        Ciao
                    </div>
                </Transition>
            </div>
        </template>
    </div>

    <Console class="accent-border"/>
</div>
</template>



<style scoped lang="scss">
.vl {
    width: 5px;

    margin-left: 17px;
    
    background-color: $dark;
    border-radius: 16px;
    
    transition: height 0.3s ease;
}

.step-content {
    width: 491px;
    height: 225px;
    
    border: 2px solid $dark;
    border-left: none;
    border-radius: 0 16px 16px 0;
}

.grow-enter-active{
    transition: width 0.5s ease, height 0.3s ease;
    overflow: hidden;
}

.grow-enter-from,
.grow-leave-to {
    width: 0;
    height: 0;
}
</style>