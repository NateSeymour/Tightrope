export enum MeasurementSystem {
    Metric,
    Obscure,
    Funny,
}

export interface UnitOfMeasurement {
    name: string,
    short: string,
    conversion: number,
    system: MeasurementSystem,
}

export const unitDefinitions = {
    Mass: {
        kg: {
            name: 'Kilogram',
            short: 'kg',
            conversion: 1,
            system: MeasurementSystem.Metric,
        },
        lbs: {
            name: 'Pound',
            short: 'lbs',
            conversion: 2.204,
            system: MeasurementSystem.Obscure,
        },
        dmsnacl: {
            name: 'Decimegasmidgens of Table Salt',
            short: 'dmsnacl',
            conversion: 1.0845648377,
            system: MeasurementSystem.Funny,
        },
    },
    Speed: {
        mps: {
            name: 'Meters per Second',
            short: 'm/s',
            conversion: 1,
            system: MeasurementSystem.Metric,
        },
    },
    Time: {
        s: {
            name: 'Second',
            short: 's',
            conversion: 1,
            system: MeasurementSystem.Metric,
        },
    },
    Length: {
        m: {
            name: 'Meter',
            short: 'm',
            conversion: 1,
            system: MeasurementSystem.Metric,
        },
        cm: {
            name: 'Centimeter',
            short: 'cm',
            conversion: 100,
            system: MeasurementSystem.Metric,
        },
    },
    PsychScale: {
        pv: {
            name: 'Perceived Value',
            short: 'pv',
            conversion: 1,
            system: MeasurementSystem.Metric,
        },
    },
    Binary: {
        yn: {
            name: 'Yes/No',
            short: 'yn',
            conversion: 1,
            system: MeasurementSystem.Metric,
        },
    },
};