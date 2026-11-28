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
    <div id="title" class="flex-column align-center">
        <h1 class="ata-h1 ata-font-text">Uninstall</h1>
    </div>

    <div class="flex-row justify-center align-center">
        <Select v-model="selectedMods" :mods="mods" ></Select>
        <div id="confirm" class="ata-border-solid ata-radius-medium ata-border-small bd-dark pv-20 ph-20 justify-self-end" v-if="selectedMods.length > 0">
            <h1 class="ata-h1 ata-font-text">Uninstall {{selectedMods.length}} mod{{selectedMods.length==1 ? '' : 's'}}?</h1>

            <div>
                
            </div>
            
            <div class="flex-row gh-10">
                <button class="ata-btn-medium-big ata-shadow-accent-secondary bg-accent-secondary bd-accent-secondary-dark ata-border-solid">
                    <h1 class="ata-h1 ata-font-icons">delete</h1>
                    <h1 class="ata-h1 ata-font-text">Uninstall</h1>
                </button>
                <div class="flex-column align-center text-center justify-space-between bg-accent-secondary ata-radius-small ph-5 pv-10">
                    <p class="ata-p ata-font-text">Removal of<br>files is<br><span class="bold">permanent</span></p>
                    <p class="ata-p ata-font-text">It cannot<br>be undone</p>
                </div>
            </div>
        </div>
    </div>
</template>



<style scoped lang="scss">
#confirm {
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;

    margin-right: -13px;
}
</style>