<script setup lang="ts">
</script>



<template>
<div id="console">
    <div id="initializing" class="flex-row">
        <h1 class="ata-h1 ata-font-icons main">list</h1>
        <h1 class="ata-h1 ata-font-text main">Initializing...</h1>
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
    padding-bottom: 20px;

    border-width: 10px;
    border-radius: 24px;

    box-shadow: 0 5px 5px 2px;

    justify-content: top;
    align-items: left;
}

#initializing {
    gap: 10px;
}
</style>