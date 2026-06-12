<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick } from 'vue';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

interface LogEntry {
  event: string;
  message: string;
}

const logs = ref<LogEntry[]>([]);
const consoleEl = ref<HTMLElement | null>(null);
const unlisteners: UnlistenFn[] = [];

const EVENTS = [
    'setup',
    'decompression',
    'mod-analysis',
    'conflicts-check',
    'installation',
    'data-update',
    'success',
    'error',
];

onMounted(async () => {
    for (const event of EVENTS) {
        const unlisten = await listen<string>(event, (e) => {
            logs.value.push({ event: e.event, message: e.payload });
        });
        unlisteners.push(unlisten);
    }
});

onUnmounted(() => {
    unlisteners.forEach(fn => fn());
});

watch(logs, async () => {
    await nextTick();
    if (consoleEl.value) {
        consoleEl.value.scrollTop = consoleEl.value.scrollHeight;
    }
}, { deep: true });
</script>



<template>
<main id="console" class="ata-colors-accent ata-main ata-flex-column ata-border-radius" ref="consoleEl">
    <strong v-for="log in logs" id="log-{{ log.event }}" :class="{
        'log-error': log.event === 'error',
        'log-success': log.event === 'success',
        'log-line ata-colors-accent': log.event !== 'error' && log.event !== 'success'
    }">
        [{{ log.event }}] {{ log.message }}
    </strong>
</main>
</template>



<style scoped lang="scss">
#console {
    padding: 15px 15px 15px 15px;
    
    scroll-behavior: smooth;
    overflow-y: auto;
    
    box-shadow: 5px 5px 10px $ata-black;
}

.log-line {
    margin: 5px 0 5px 0;
}
.log-success {
    color: $ata-accent-tertiary;
    font-weight: bold;
    font-size: 1.3em;

    text-shadow: $ata-accent-tertiary 0px 0px 20px, 0px 0px 30px $ata-accent-tertiary;
}
.log-error {
    color: $ata-accent-secondary;
    font-weight: bold;
    font-size: 1.3em;

    text-shadow: $ata-accent-secondary 0px 0px 20px, 0px 0px 30px $ata-accent-secondary;
}
</style>