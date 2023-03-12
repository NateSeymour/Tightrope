import { unitDefinitions } from '@/measurement';
import { defineStore } from 'pinia';
import { reactive } from 'vue';

export const usePrefStore = defineStore('preferences', () => {
    const units = reactive({
        Mass: unitDefinitions.Mass.lbs,
        Speed: unitDefinitions.Speed.mps,
        Time: unitDefinitions.Time.s,
        Length: unitDefinitions.Length.cm,
        PsychScale: unitDefinitions.PsychScale.pv,
        Binary: unitDefinitions.Binary.yn,
    });

    return { units };
});