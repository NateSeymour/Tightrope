<template>
    <div class="measurement-logger">
        <BestCalendar @date-selected="dateSelected" class="best-calendar" />

        <div v-if="metric">
            <DomainEditor @value-selected="valueSelected" :metric="metric" />
        </div>

        <MaterialButton 
            v-if="metric" 
            @click="mutate()"
            class="log-btn">
            Log {{ metric.name }}
        </MaterialButton>
    </div>
</template>

<script lang="ts" setup>
import { Ref, ref, computed } from 'vue'; 
import { useMutation, useQuery, useQueryClient } from '@tanstack/vue-query';
import { Metric } from '@/api/metric/metric';
import { useRoute, useRouter } from 'vue-router';
import DomainEditor from '@/app/metric/ui/input/MetricValueEditor.vue';
import BestCalendar from '@/ui/input/BestCalendar.vue';
import MaterialButton from '@/ui/material/MaterialButton.vue';

const router = useRouter();
const route = useRoute();

const metricName = route.params.metricName as string;

const millis: Ref<number> = ref(0);
const measurementValue: Ref<number> = ref(0);

const queryClient = useQueryClient();

const { data: metric } = useQuery({
    queryKey: ['metric', metricName],
    queryFn: Metric.fetchMetricByName.bind(null, metricName),
});

const sumbitMeasurementEnabled = computed(() => !!metric.value);
const unit = computed(() => metric.value!.useLocalizedUnit());

const { mutate } = useMutation({
    // @ts-ignore
    mutationFn: () => metric.value!.postMeasurement(measurementValue.value / unit.value!.conversion, millis.value),
    onSuccess: () => {
        queryClient.invalidateQueries([metric, 'measurements']);
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
.measurement-logger {
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