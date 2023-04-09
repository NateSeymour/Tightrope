<template>
    <div class="metric-home">
        <TrackerTimeFrameSelector />
        <MetricGraphList />
    </div>
</template>

<script setup lang="ts">
import MetricGraphList from '@/app/metric/component/MetricGraphList.vue';
import TrackerTimeFrameSelector from '@/app/metric/component/MetricTimeFrameSelector.vue';
</script>