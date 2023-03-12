import { fetchApi } from '@/api';
import { Measurement } from '@/api/measurement';
import { UnitOfMeasurement } from '@/measurement';
import { usePrefStore } from '@/store/preferences';

export enum MetricType {
    Mass = 'Mass',
    Speed = 'Speed',
    Time = 'Time',
    Length = 'Length',
    PsychScale = 'PsychScale',
    Binary = 'Binary',
}

export class Metric {
    metric_id: number;
    metric_type: MetricType;
    username: string | null;
    global: boolean;
    name: string;
    description: string | null;

    constructor(m: Metric) {
        this.metric_id = m.metric_id;
        this.metric_type = m.metric_type;
        this.username = m.username;
        this.global = m.global;
        this.name = m.name;
        this.description = m.description;
    }

    // API functions
    static async fetchList(): Promise<Metric[]> {
        const res = await fetchApi('/metric/list', 'GET');
        const parsed = await res.json();
        return parsed.map((v: Metric) => new Metric(v));
    }

    static async fetchMetricByName(name: string): Promise<Metric> {
        const res = await fetchApi(`/metric/${name}`, 'GET');
        return new Metric(await res.json());
    }

    fetchMeasurements = async (): Promise<Measurement[]> => {
        const res = await fetchApi(`/metric/${this.name}/measurements`, 'GET');
        return await res.json();
    };

    postMeasurement = async (value: number, millis: number) => {
        return await fetchApi(`/metric/${this.name}/measurement`, 'POST', JSON.stringify({ value, millis }));
    };

    updateMeasurement = async (id: number, value: number, millis: number) => {
        return await fetchApi(`/metric/${this.name}/measurement/${id}`, 'PATCH', JSON.stringify({ value, millis }));
    };

    deleteMeasurement = async (id: number) => {
        return await fetchApi(`/metric/${this.name}/measurement/${id}`, 'DELETE');
    };

    fetchMeasurement = async (id: number): Promise<Measurement> => {
        const res = await fetchApi(`/metric/${this.name}/measurement/${id}`, 'GET');
        return new Measurement(await res.json());
    };

    // Composables
    useLocalizedUnit = (): UnitOfMeasurement => {
        const prefStore = usePrefStore();

        return prefStore.units[this.metric_type];
    };
}