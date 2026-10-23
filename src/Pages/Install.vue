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
const titles = ['Select the archive', 'Give a name', 'Confirmation', 'Success']
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
                        <h2 v-if="steps[i] == StepProgress.Doing || steps[steps.length-1] == StepProgress.Done"
                        class="ata-h2 ata-font-text align-self-center step-title">
                            {{title}}
                        </h2>
                    </div>
            </btn>
            <div class="flex-row">
                <div v-if="i < titles.length - 1"
                    class="vl"
                    :style="{ height: lineHeight(i) }">
                </div>
                <Transition name="grow">
                    <div v-if="steps[i] === StepProgress.Doing" class="align-self-center step-content">
                        <div v-if="i==0" class="flex-column">
                            <h2 class="ata-h2 ata-font-text step-instruction">Extensions: .zip/.7z/.rar</h2>
                            <div class="step-buttons flex-column">
                                <btn class="ata-btn-medium-small accent-border flex-column">
                                    <h3 class="ata-h3 ata-font-icons main">drive_folder_upload</h3>
                                    <h3 class="ata-h3 ata-font-text main">Select</h3>
                                </btn>
                                <btn class="ata-btn-small dark-tertiary" @click="goTo(i+1)">
                                    <h3 class="ata-h3 ata-font-icons accent-tertiary">check</h3>
                                </btn>
                            </div>
                        </div>
                        <div v-else-if="i==1" class="flex-column">
                            <h2 class="ata-h2 ata-font-text step-instruction">Will be used for filtering and uninstalling</h2>
                            <div class="step-buttons flex-column">
                                INPUT
                                <btn class="ata-btn-small dark-tertiary" @click="goTo(i+1)">
                                    <h3 class="ata-h3 ata-font-icons accent-tertiary">check</h3>
                                </btn>
                            </div>
                        </div>
                        <div v-else-if="i==2" class="flex-column">
                            <h2 class="ata-h2 ata-font-text step-instruction">Installing ""</h2>
                            <h2 class="ata-h2 ata-font-text">Wait for the installation process to be complete</h2>
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

.step-title {
    margin-left: 5px;
}
.step-content {
    width: 491px;
    height: 225px;

    padding: 10px;
    
    border: 2px solid $dark;
    border-left: none;
    border-radius: 0 16px 16px 0;
}
.step-instruction {
    margin-bottom: 20px;
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