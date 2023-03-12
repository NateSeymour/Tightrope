<template>
    <NumericWheel 
        v-if="props.metric.metric_type === MetricType.Mass" 
        @value-selected="val => emit('valueSelected', val)"
        :hint="hint"
        :metric="metric" />
    <NumericWheel 
        v-if="props.metric.metric_type === MetricType.Length" 
        @value-selected="val => emit('valueSelected', val)"
        :hint="hint"
        :metric="metric" />
</template>

<script setup lang="ts">
import { Metric, MetricType } from '@/api/metric';
import NumericWheel from '@/ui/input/DomainEditor/NumericWheel.vue';

const props = defineProps<{
    metric: Metric,
    hint?: number,
}>();

const emit = defineEmits(['valueSelected']);
</script>