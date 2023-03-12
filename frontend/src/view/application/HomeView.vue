<template>
    <div class="home">
        <div class="stats">
            <TrackerTimeFrameSelector />
            <MetricGraphList />
        </div>
    </div>
</template>

<script setup lang="ts">
import MetricGraphList from '@/component/MetricGraphList.vue';
import TrackerTimeFrameSelector from '@/component/TrackerTimeFrameSelector.vue';
</script>

<style scoped lang="scss">
.home {
    padding: 1em;
    height: 100%;
    width: 100%;
}
</style>
