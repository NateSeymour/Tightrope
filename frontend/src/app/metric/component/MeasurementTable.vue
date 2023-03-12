<template>
    <div class="measurement-table">
        <div class="controls" v-if="selectedCount">
            <div class="wrapper">
                <DeleteIcon @click="deleteMeasurements" class="delete-icon" />
            </div>
        </div>
        <table v-if="data">
            <MeasurementEl 
                v-for="measurement in data.slice().reverse()"
                @click="measurementClick(measurement)" 
                @select="(selected) => measurementSelect(selected, measurement)"
                :measurement="measurement" 
                :metric="metric" 
                :key="measurement.millis + measurement.value"
            />
        </table>
    </div>
</template>

<script lang="ts" setup>
import { Ref, ref, computed } from 'vue';
import { Metric } from '@/api/metric/metric';
import { Measurement } from '@/api/metric/measurement';
import { useRouter } from 'vue-router';
import { useMutation, useQuery, useQueryClient } from '@tanstack/vue-query';
import MeasurementEl from '@/app/metric/ui/MeasurementTableRow.vue';
import DeleteIcon from '@/ui/material/icon/DeleteIcon.vue';

const router = useRouter();
const queryClient = useQueryClient();

const props = defineProps<{
    metric: Metric,
}>();

const selectedMeasurements: Ref<Measurement[]> = ref([]);
const selectedCount = computed(() => selectedMeasurements.value.length);

const { data } = useQuery({
    queryKey: ['metric', props.metric.name, 'measurements'],
    queryFn: props.metric.fetchMeasurements,
});
const deletionEnabled = computed(() => !!data.value);

const { mutate } = useMutation({
    // @ts-ignore
    mutationFn: async ({ measurement_id }: Measurement) => props.metric.deleteMeasurement(measurement_id),
    onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: ['metric', props.metric.name, 'measurements'] });
    },
    enabled: deletionEnabled,
});

function measurementClick(measurement: Measurement) {
    router.push(`/metric/edit/${props.metric.name}/${measurement.measurement_id}`);
}

function measurementSelect(selected: boolean, measurement: Measurement) {
    if(selected) {
        selectedMeasurements.value.push(measurement);
    } else {
        selectedMeasurements.value.splice(selectedMeasurements.value.indexOf(measurement), 1);
    }
}

function deleteMeasurements() {
    selectedMeasurements.value.forEach((measurement) => { mutate(measurement); });
    selectedMeasurements.value = [];
}
</script>

<style scoped lang="scss">
.measurement-table {
    display: flex;
    flex-direction: column;
    overflow: hidden;

    .controls {
        display: flex;
        flex-direction: row;
        justify-content: right;
        overflow: hidden;

        @keyframes grow-in {
            from {
                height: 0
            }

            to {
                height: 2.5em;
            }
        }

        animation: grow-in 200ms ease-out forwards;

        .wrapper {
            height: 2em;
            width: 2em;
            margin-right: 0.5em;

            .delete-icon {
                fill: white;
                width: 100%;
                height: 100%;
            }
        }
    }

    table {
        display: flex;
        flex-direction: column;
        overflow-y: scroll;
        color: white;
    }
}
</style>