<template>
    <div class="measurement-editor">
        <div v-if="metric && measurement">
            <BestCalendar @date-selected="dateSelected" class="best-calendar" :hint="measurement.millis" />
            <DomainEditor @value-selected="valueSelected" :metric="metric" :hint="measurement.value * unit.conversion" />
            <MaterialButton 
                v-if="metric" 
                @click="mutate()"
                class="log-btn">
                Update {{ metric.name }}
            </MaterialButton>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { Ref, ref, computed } from 'vue'; 
import { useMutation, useQuery, useQueryClient } from '@tanstack/vue-query';
import { Metric } from '@/api/metric';
import { useRoute, useRouter } from 'vue-router';
import DomainEditor from '@/ui/input/DomainEditor/DomainEditor.vue';
import BestCalendar from '@/ui/input/BestCalendar.vue';
import MaterialButton from '@/ui/material/MaterialButton.vue';

const router = useRouter();
const route = useRoute();

const metricName = route.params.metricName as string;
const measurementId = parseInt(route.params.measurementId as string);

const millis: Ref<number> = ref(0);
const measurementValue: Ref<number> = ref(0);

const queryClient = useQueryClient();

const { data: metric } = useQuery({
    queryKey: ['metric', metricName],
    queryFn: Metric.fetchMetricByName.bind(null, metricName),
});

const measurementEnabled = computed(() => !!metric.value);

const { data: measurement } = useQuery({
    queryKey: ['measurement', measurementId],
    queryFn: () => metric.value!.fetchMeasurement(measurementId),
    enabled: measurementEnabled,
});

const sumbitMeasurementEnabled = computed(() => !!measurement.value);
const unit = computed(() => metric.value!.useLocalizedUnit());

const { mutate } = useMutation({
    // @ts-ignore
    mutationFn: async () => await metric.value!.updateMeasurement(measurementId, measurementValue.value / unit.value!.conversion, millis.value),
    onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: ['metric', metric.value!.name]});
        queryClient.invalidateQueries({ queryKey: ['measurement', measurementId]});

        router.back();
    },
    enabled: sumbitMeasurementEnabled,
});

function dateSelected(newDate: Date) {
    millis.value = newDate.getTime();
}

function valueSelected(value: number) {
    measurementValue.value = value;
}
</script>

<style lang="scss" scoped>
.measurement-editor {
    height: 100%;
    width: 100%;
    padding: 1em;

    .best-calendar {
        height: 17em;
    }

    .log-btn {
        width: 100%;
    }
}
</style>