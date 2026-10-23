<script setup lang="ts">
import { reactive, ref, onMounted, onUnmounted } from 'vue'
import Console from '../Components/Console.vue'
import { listen, UnlistenFn } from '@tauri-apps/api/event'

// Steps
enum StepProgress { NotDone, Doing, Done }
const steps = reactive([StepProgress.Doing, StepProgress.NotDone, StepProgress.NotDone, StepProgress.NotDone])
const stepsEvents = ref<LogEntry[]>([])

// Backend Events
const EVENTS = [
  'setup', 'decompression', 'mod-analysis', 'conflicts-check',
  'installation', 'data-update', 'success', 'error',
]

const unlisteners: UnlistenFn[] = []

onMounted(async () => {
  for (const event of EVENTS) {
    unlisteners.push(
      await listen<string>(event, (e) => updateLogs(event, e.payload))
    )
  }
})
const updateLogs = (event: string, message: string) =>
  stepsEvents.value.push({ event, message })

onUnmounted(() => {
  unlisteners.forEach(un => un())
  unlisteners.length = 0
})

// HTML related
const icons = ['counter_1', 'counter_2', 'counter_3', 'counter_4']
const titles = ['AAA', 'BBB', 'CCC', 'DDD']
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
        <template v-for="(title, i) in titles" :key="i">
            <btn @click="goTo(i)">
                    <div class="flex-row">
                        <h1 class="ata-h1 ata-font-icons"
                            :class="isActive(i) ? 'accent-tertiary' : 'accent-secondary'">
                            {{"counter_" + (i+1)}}
                        </h1>
                        <h2 v-if="steps[i] == StepProgress.Doing" class="ata-h2 ata-font-text">{{title}}</h2>
                    </div>
            </btn>
            <div class="flex-row">
                <div v-if="i < icons.length - 1"
                    class="vl"
                    :style="{ height: lineHeight(i) }">
                </div>
                <Transition name="grow">
                    <div v-if="steps[i] === StepProgress.Doing" class="step-content align-self-center flex-column">
                        <h2 class="ata-h2 ata-font-text">Extensions: .zip/.7z/.rar</h2>
                        <div class="step-buttons flex-column">
                            <btn class="ata-btn-medium-small accent-border"></btn>
                            <btn class="ata-btn-small dark-tertiary"></btn>
                        </div>
                    </div>
                </Transition>
            </div>
        </template>
    </div>

    <Console class="accent-border" :logs="stepsEvents"/>
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

    padding: 10px;
    gap: 20px;
    
    border: 2px solid $dark;
    border-left: none;
    border-radius: 0 16px 16px 0;
}
.step-buttons {
    gap: 10px;
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