import { dateTimeFormat } from '@/intl';
import { computed } from 'vue';
import { defineStore } from 'pinia';
import { Ref, onBeforeMount, ref } from 'vue';

export enum TimeFrame {
    Millisecond = 1,
    Second = Millisecond * 1000,
    Minute = Second * 60,
    Hour = Minute * 60,
    Day = Hour * 24,
    Week = Day * 7,
    Month = Day * 30,
    Year = Day * 365,
}

export const useTimeStore = defineStore('time', () => {
    const displayTimeFrame: Ref<TimeFrame> = ref(TimeFrame.Month);
    const timeFrameDateLabels = computed(() => {
        const now = Date.now();

        const dates = [];
        for(let i = (now + TimeFrame.Day) - displayTimeFrame.value; i <= now; i += TimeFrame.Day) {
            const dayString = dateTimeFormat.format(i);
            dates.push(dayString);
        }

        return dates;
    });

    onBeforeMount(() => {
        if(window.innerWidth < 500) {
            displayTimeFrame.value = TimeFrame.Week;
        } else {
            displayTimeFrame.value = TimeFrame.Month;
        }
    });

    return { displayTimeFrame, timeFrameDateLabels };
});