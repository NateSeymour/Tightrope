<template>
    <div class="metric-tracker">
        <TrackerTimeFrameSelector />

        <div class="chart-container">
            <MetricGraph v-if="data" :metric="data" />
        </div>

        <MeasurementTable v-if="data" :metric="data" class="measurement-list" />

        <div class="log-controls">
            <button @click="router.push(`/log/${metricName}`)">
                <AddCircleIcon class="add-circle"/>
            </button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useQuery } from '@tanstack/vue-query';
import { Metric } from '@/api/metric';
import TrackerTimeFrameSelector from '@/component/TrackerTimeFrameSelector.vue';
import MeasurementTable from '@/component/MeasurementTable.vue';
import MetricGraph from '@/component/MetricGraph.vue';
import AddCircleIcon from '@/ui/material/icon/AddCircleIcon.vue';

const route = useRoute();
const router = useRouter();

const metricName = ref(route.params.metricName as string);

const { data } = useQuery({
    queryKey: ['metric', metricName.value],
    queryFn: Metric.fetchMetricByName.bind(null, metricName.value),
});
</script>

<style scoped lang="scss">
@use "@/style/colors";

.metric-tracker {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    padding: 1em;

    .chart-container {
        color: white;

        .chart {
            width: 100%;
            height: 20em;
            -webkit-tap-highlight-color: transparent;

            &:focus {
                outline: none;
            }
        }
    }    .log-controls {
        display: flex;
        flex-direction: column;
        margin-top: 1em;

        button {
            border: none;
            background: transparent;
            color: white;
            font-weight: bold;
            transition: 200ms;
            height: 3em;

            .add-circle {
                height: 100%;
                width: 100%;
                fill: white;
            }
        }
    }
}
</style>