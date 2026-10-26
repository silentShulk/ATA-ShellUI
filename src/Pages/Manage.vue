<script setup>

</script>



<template>
<div id="title" class="flex-column align-center">
    <h1 class="ata-h1 ata-font-text">Manage</h1>
</div>

<div id="content" class="flex-column">
    <div id="buttons" class="flex-row justify-center g-40">
        <button class="ata-btn-small dark-secondary ata-h3 ata-font-icons accent-secondary">toggle_off</button>
        <button class="ata-btn-small dark-tertiary ata-h3 ata-font-icons accent-tertiary">toggle_on</button>
    </div>
    <div id="mod-list">
        <div class="flex-row g-20">
            <button class="ata-btn-small-long flex-row g-5">
                <h3 class="ata-h3 ata-font-icons">person</h3>
                <h3 class="ata-h3 ata-font-text">aaaa</h3>
            </button>
            <h2 class="ata-h2 ata-font-text">sigma</h2>
        </div>
    </div>
</div>
</template>



<style scoped>
#mod-list {
    display: grid;

    grid-template-rows: 4 1fr 1fr 1fr 1fr;
    grid-template-columns: 3 1fr 1fr 1fr;

    margin: 35px 70px 25px 70px;
}
</style>