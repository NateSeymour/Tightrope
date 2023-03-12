export class Measurement {
    measurement_id: number;
    metric_id: number;
    username: string;
    millis: number;
    value: number;

    constructor(m: Measurement) {
        this.measurement_id = m.measurement_id;
        this.metric_id = m.metric_id;
        this.username = m.username;
        this.millis = m.millis;
        this.value = m.value;
    }
}