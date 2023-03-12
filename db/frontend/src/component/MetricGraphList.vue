<template>
    <div v-if="isLoading"></div>
    <div v-else-if="isError"></div>
    <div v-else-if="data" class="overlay" v-for="metric in data" @click="router.push(`/track/${metric.name}`)" :key="metric.metric_id">
        <MetricGraph class="chart" :metric="metric" />
    </div>
</template>

<script setup lang="ts">
import { Metric } from '@/api/metric';
import { useQuery } from '@tanstack/vue-query';
import { useRouter } from 'vue-router';
import MetricGraph from './MetricGraph.vue';

const router = useRouter();

const { isLoading, isError, data } = useQuery({
    queryKey: ['metric-list'],
    queryFn: Metric.fetchList,
});
</script>

<style scoped lang="scss">
.overlay {
    .chart {
        width: 100%;
        height: 20em;
        pointer-events: none;

        &:focus {
            outline: none;
        }
    }
}
</style>