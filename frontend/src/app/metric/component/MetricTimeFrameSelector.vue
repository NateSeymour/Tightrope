<template>
    <div class="time-selector">
        <button @click="timeStore.displayTimeFrame = TimeFrame.Week" :class="timeStore.displayTimeFrame == TimeFrame.Week ? 'selected' : ''">W</button>
        <button @click="timeStore.displayTimeFrame = TimeFrame.Month" :class="timeStore.displayTimeFrame == TimeFrame.Month ? 'selected' : ''">M</button>
        <button @click="timeStore.displayTimeFrame = TimeFrame.Year" :class="timeStore.displayTimeFrame == TimeFrame.Year ? 'selected' : ''">Y</button>
    </div>
</template>

<script lang="ts" setup>
import { useTimeStore, TimeFrame } from '@/store/time';

const timeStore = useTimeStore();
</script>

<style lang="scss" scoped>
@use "@/style/_colors.scss";

.time-selector {
    width: 100%;
    display: flex;
    justify-content: right;
    
    button {
        border: none;
        border-radius: 0.2em;
        padding: 0.75em;
        margin-left: 0.5em;
        background: colors.$dark;
        color: white;
        font-weight: bold;
        transition: 200ms;

        &.selected {
            background: colors.$accent;
        }
    }
}
</style>