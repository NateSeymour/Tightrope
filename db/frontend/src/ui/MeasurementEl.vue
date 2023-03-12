<template>
    <tr class="weight-el">
        <MaterialOverlay @click="emit('click')" @select="(selected) => emit('select', selected)" :selectable="true" />
        
        <div class="content">
            <span>{{ dateTimeFormat.format(props.measurement.millis) }} </span>
            <span>{{ localizedValue }} {{ unit.short }}</span>
        </div>
    </tr>
</template>

<script lang="ts" setup>
import { dateTimeFormat } from '@/intl';
import { Metric } from '@/api/metric';
import { Measurement } from '@/api/measurement';
import MaterialOverlay from '@/ui/material/MaterialOverlay.vue';

const emit = defineEmits(['click', 'select']);

const props = defineProps<{
    metric: Metric,
    measurement: Measurement,
}>();

const unit = props.metric.useLocalizedUnit();
const localizedValue = props.measurement.value * unit.conversion;
</script>

<style lang="scss" scoped>
    .weight-el {
        color: white;
        height: 3em;

        .content {
            width: 100%;
            height: 100%;
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            align-items: center;
            padding: 0.5em;
            font-size: 20px;

            span {
                user-select: none;
                pointer-events: none;
            }
        }
    }
</style>