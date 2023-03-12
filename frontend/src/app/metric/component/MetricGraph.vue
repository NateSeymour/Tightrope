<template>
    <v-chart v-if="data" class="chart" :option="chart" autoresize />
</template>

<script setup lang="ts">
import { computed, onBeforeMount } from 'vue';
import { useTimeStore } from '@/store/time';
import { dateTimeFormat } from '@/intl';
import { Metric } from '@/api/metric/metric';
import { useQuery } from '@tanstack/vue-query';
import { initializeCharts } from '@/charts';
import VChart from 'vue-echarts';

const timeStore = useTimeStore();

const props = defineProps<{
    metric: Metric,
    displayTitle?: boolean,
}>();

const { data } = useQuery({
    queryKey: ['metric', props.metric.name, 'measurements'],
    queryFn: props.metric.fetchMeasurements,
});

const unit = props.metric.useLocalizedUnit();

const chart = computed(() => {
    const seriesData = new Array(timeStore.timeFrameDateLabels.length);
    seriesData.fill('-');

    /*
     * The "|| []" is really just to make ts shut up.
     * The computed will only be run when data is available, as it
     * is protected through the "v-if" on the component. 
     */
    for(const measurement of data.value || []) {
        const dayString = dateTimeFormat.format(measurement.millis);

        const localeWeight = measurement.value * unit.conversion;
        seriesData[timeStore.timeFrameDateLabels.indexOf(dayString)] = localeWeight;
    }

    return {
        title: {
            show: !!props.displayTitle,
            text: props.metric.name,
        },
        dataZoom:[ {
            type: 'inside',
            id: 'insideX',
            xAxisIndex: 0,
            zoomOnMouseWheel: true,
            moveOnMouseMove: true,
        }],
        xAxis: {
            type: 'category',
            data: timeStore.timeFrameDateLabels,
            axisLabel: {
                rotate: 45,
            },
        },
        yAxis: {
            type: 'value',
            name: unit.short,
            scale: true,
        },
        tooltip: {
            formatter: `{c} ${unit.short}`,
        },
        series: [
            {
                data: seriesData,
                type: 'line',
                smooth: true,
                connectNulls: true,
                itemStyle: {
                    color: 'white',
                    size: 20,
                    borderWidth: 3,
                    borderColor: '#ee6c4d',
                },
                lineStyle: {
                    color: '#ee6c4d',
                    width: 4,
                },
            },
        ],
    };
});

onBeforeMount(() => {
    initializeCharts();
});
</script>