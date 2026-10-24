<script setup lang="ts">
import { reactive, ref, onMounted, onUnmounted } from 'vue'
import Console from '../Components/Console.vue'
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';

// Steps
enum StepProgress { NotDone, Doing, Done }
const steps = reactive([StepProgress.Doing, StepProgress.NotDone, StepProgress.NotDone, StepProgress.NotDone])
const stepsInputs = ref<InstallationInput[]>([])
const stepsLogs = ref<InstallationLogEntry[]>([])

// Events / Inputs
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
onUnmounted(() => {
  unlisteners.forEach(un => un())
  unlisteners.length = 0
})

const updateLogs: (event: string, message: string) => void = (event: string, message: string): void => {
  if (!stepsLogs.value.some(log => log.event === event))
    stepsLogs.value.push({ event, message })
};
const updateInputs = (title: string, icon: string, input: string) => {
  const index = stepsInputs.value.findIndex(input => input.title === title);
  if (index === -1) {
    stepsInputs.value.push({title, icon, input});
  } else {
    stepsInputs.value[index] = {title, icon, input};
  }
}

// HTML related
const titles = ['Select the archive', 'Give a name', 'Confirmation', 'Success']
const isActive = (i: number) =>
  steps[i] === StepProgress.Doing || steps[i] === StepProgress.Done
const lineHeight = (i: number) => {
  if (steps.every(x => x == StepProgress.Done)) return '122px'
  else return (steps[i] === StepProgress.Doing ? '245px' : '60px')
}
const goTo = (i: number) => {
  if (!installationOnGoing.value) {
    steps.fill(StepProgress.Done, 0, i)
    steps[i] = i === steps.length - 1 ? StepProgress.Done : StepProgress.Doing
    steps.fill(StepProgress.NotDone, i + 1)
  }
}

const folderSelected = ref(false)
async function getArchive() {
  const modArchive = await open({
    multiple: false,
    directory: false,
    filters: [
      { name: 'Archives', extensions: ['zip', '7z', 'rar'] }
    ]
  });
  if (modArchive != null) {
    updateInputs('Folder:', 'folder', modArchive)
    folderSelected.value = true
  }  
}

const modName = ref('')
const nameSelected = ref(false)
function onNameInput(e: Event) {
  modName.value = (e.target as HTMLInputElement).value
  nameSelected.value = false
}
function getName() {
  const name = modName.value.trim()
  if (name !== '') {
    updateInputs('Name:', 'label', name)
    nameSelected.value = true
  }
}

const installationOnGoing = ref(false)
async function installMod() {
  await invoke('install_mod', { answeredName: modName.value })
  installationOnGoing.value = false
  goTo(3)
}
</script>



<template>
<div id="title" class="flex-column align-center">
    <h1 class="ata-h1 ata-font-text">Install</h1>
</div>

<div id="content" class="flex-row justify-space-between">
    <div id="installation-steps" class="flex-column justify-space-between">
        <template v-for="(title, i) in titles" :key="i">
            <button @click="if (steps[i] == StepProgress.Done) goTo(i);" class="unstyled-btn">
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
            </button>
            <div class="flex-row">
                <div v-if="i < titles.length - 1"
                    class="vl"
                    :style="{ height: lineHeight(i) }">
                </div>
                <Transition name="grow">
                    <div v-if="i==0 && steps[i] === StepProgress.Doing" class="flex-column align-self-center step-content">
                        <h2 class="ata-h2 ata-font-text step-instruction">Extensions: .zip/.7z/.rar</h2>
                        <div class="step-buttons flex-column">
                            <button class="ata-btn-medium-small accent-border flex-column"
                            @click="getArchive">
                                <h3 class="ata-h3 ata-font-icons main">drive_folder_upload</h3>
                                <h3 class="ata-h3 ata-font-text main">Select</h3>
                            </button>
                            <button v-if="folderSelected" class="ata-btn-small dark-tertiary" @click="goTo(i+1)">
                                <h3 class="ata-h3 ata-font-icons accent-tertiary">check</h3>
                            </button>
                        </div>
                    </div>
                    <div v-else-if="i==1 && steps[i] === StepProgress.Doing" class="flex-column align-self-center step-content">
                        <h2 class="ata-h2 ata-font-text step-instruction">Will be used for filtering and uninstalling</h2>
                        <div class="step-buttons flex-column">
                            <input type="text" :value="modName" @input="onNameInput" @change="getName"
                                    class="ata-input-text-small ata-h2 accent-border main" placeholder="Name">
                            <button v-if="nameSelected" class="ata-btn-small dark-tertiary" @click="goTo(i+1)">
                                <h3 class="ata-h3 ata-font-icons accent-tertiary">check</h3>
                            </button>
                        </div>
                    </div>
                    <div v-else-if="i==2 && steps[i] === StepProgress.Doing"
                        class="flex-column align-self-center step-content justify-center">
                        <Transition name="swap" mode="out-in">
                            <div v-if="!installationOnGoing" key="confirm">
                                <h2 class="ata-h2 ata-font-text step-instruction">Install "{{modName}}"?</h2>
                                <button class="ata-btn-small dark-tertiary" @click="installationOnGoing = true; installMod()">
                                    <h3 class="ata-h3 ata-font-icons accent-tertiary">install_desktop</h3>
                                </button>
                            </div>
                            <div v-else key="installing">
                                <h2 class="ata-h2 ata-font-text step-instruction">Installing "{{modName}}"</h2>
                                <h2 class="ata-h2 ata-font-text">Wait for the installation process to be complete</h2>
                            </div>
                        </Transition>
                    </div>
                </Transition>
            </div>
        </template>
    </div>

    <Console class="accent-border" :inputs="stepsInputs" :logs="stepsLogs"/>
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

.swap-enter-active,
.swap-leave-active {
    transition: opacity 0.25s ease, transform 0.25s ease;
}
.swap-enter-from,
.swap-leave-to {
    opacity: 0;
    transform: scale(0.85);
}
</style>