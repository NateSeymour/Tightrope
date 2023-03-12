<template>
    <div class="home">
        <div class="stats">
            <TrackerTimeFrameSelector />
            <MetricGraphList />
        </div>
    </div>
</template>

<script setup lang="ts">
import MetricGraphList from '@/app/metric/component/MetricGraphList.vue';
import TrackerTimeFrameSelector from '@/app/metric/component/MetricTimeFrameSelector.vue';
</script>

<style scoped lang="scss">
.home {
    padding: 1em;
    height: 100%;
    width: 100%;
}
</style>
