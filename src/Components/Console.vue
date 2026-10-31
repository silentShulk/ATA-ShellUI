<script setup lang="ts">
const props = defineProps<{
  inputs: InstallationInput[]
  logs: InstallationLogEntry[]
}>()
</script>



<template>
<div id="console">
    <div id="initializing" class="flex-row">
        <h1 class="ata-h1 ata-font-icons main">list</h1>
        <h1 class="ata-h1 ata-font-text main">Initializing...</h1>
    </div>
    
    <div id="inputs" class="flex-column">
        <div v-for="input in props.inputs" class="input-entry">
            <h2 class="ata-h2 ata-font-text main">{{input.title}}</h2>
            <div class="flex-row input">
                <h3 class="ata-h3 ata-font-icons accent-tertiary">{{input.icon}}</h3>
                <h3 class="ata-h3 ata-font-text accent-tertiary">{{input.input}}</h3>
            </div>
        </div>
    </div>
    
    <div id="logs" class="flex-column">
        <div v-for="log in props.logs" class="flex-row log-entry">
            <h3 class="ata-h3 ata-font-text bolder main"
            :class="log.event === 'success' ? 'highlighted' : ''">
                [{{log.event}}]
            </h3>
            <h3 class="ata-h3 ata-font-text main"
            :class="log.event === 'success' ? 'highlighted' : ''">
                {{log.message}}
            </h3>
        </div>
    </div>
</div>
</template>



<style scoped lang="scss">
#console {
    display: flex;
    flex-direction: column;
    
    width: 600px;
    height: 615px;

    padding-left: 20px;
    padding-right: 20px;
    padding-top: 10px;

    gap: 20px;

    border-width: 10px;
    border-radius: 24px;

    box-shadow: 0 5px 5px 2px;

    justify-content: top;
    align-items: left;

    pointer-events: none;
}

#initializing {
    gap: 10px;
}

#inputs {
    gap: 10px;
}
.input-entry {
    gap: 0;
}
.input {
    gap: 10px;
}

#logs {
    gap: 10px;
}
.log-entry {
    gap: 10px;
}
</style>