<template>
    <div class="mass-editor">
        <svg 
            @touchstart="touchStart" 
            @touchmove="touchMove"
            @touchend="touchEnd"
            ref="editor"
            viewBox="0 0 100 100" 
            xmlns="http://www.w3.org/2000/svg">
            <defs>
                <clipPath id="circleClip">
                    <path d="M 50,50 L 0,100 L 0,0 L 100,0 L 100,100 Z"/>
                </clipPath>
            </defs>
            
            <path 
                class="caret"
                fill="white"
                stroke="none"
                d="M 50,12 l -2,5 h 4 Z" />

            <text class="selected-value" font-size="8" fill="white" x="50%" y="50%" text-anchor="middle" dominant-baseline="middle">
                {{ selectedValue }} {{ prefStore.units[metric.metric_type].short }}
            </text>

            <g class="value-ring" transform-origin="center" clip-path="url(#circleClip)">
                <g transform-origin="center" :transform="`rotate(${rotation})`" class="inner">
                    <g class="guides">
                        <g class="tenthGuideMarkers">
                            <path 
                                v-for="i of range(availableElementsCount * 10)" 
                                :key="i"
                                :transform="`rotate(${i * (360/(availableElementsCount * 10))})`"
                                stroke="rgb(150,150,150)"
                                stroke-width="0.5"
                                transform-origin="50% 50%"
                                d="M 50,10 v 1" />
                        </g>

                        <g class="halfGuideMarkers">
                            <path 
                                v-for="i of range(availableElementsCount)" 
                                :key="i"
                                :transform="`rotate(${(i + 0.5) * (360/availableElementsCount)})`"
                                stroke="white"
                                stroke-width="0.5"
                                transform-origin="50% 50%"
                                d="M 50,10 v 1.5" />
                        </g>

                        <g class="fullGuideMarkers">
                            <path 
                                v-for="i of range(availableElementsCount)" 
                                :key="i"
                                :transform="`rotate(${i * (360/availableElementsCount)})`"
                                stroke="white"
                                stroke-width="1"
                                transform-origin="50% 50%"
                                d="M 50,10 v 2" />
                        </g>
                    </g>
                    
                    <path 
                        id="scale-guide"
                        stroke="white"
                        stroke-width="0.5"
                        fill="none"
                        transform-origin="center"
                        :pathLength="circumference"
                        d="M 50,90
                            A 40,40,0,1,1,50,10
                            A 40,40,0,1,1,50,90
                            Z" />

                    <text v-for="i of range(availableElementsCount)" class="scale-text" dy="-2" :key="i" fill="white" text-anchor="middle" dominant-baseline="middle">
                        <textPath x="0" y="0" font-size="4" :startOffset="(i * spacing)" xlink:href="#scale-guide">
                            {{ baseValue + (i - (availableElementsCount/2)) + ((-1 * Math.floor((((rotation) + ((i/availableElementsCount) * 360))) / 360)) * availableElementsCount) }}
                        </textPath>
                    </text>
                </g>
            </g>
        </svg>
    </div>
</template>

<script setup lang="ts">
import { usePrefStore } from '@/store/preferences';
import { ref, computed, reactive, Ref, onMounted } from 'vue';
import { range } from '@/tools/range';
import { Metric } from '@/api/metric/metric';

const props = defineProps<{
    hint?: number,
    metric: Metric,
}>();

const emit = defineEmits(['valueSelected']);

const prefStore = usePrefStore();

const editor: Ref<HTMLElement | undefined> = ref();
const spacing = 5;
const circumference = 100;
const availableElementsCount = circumference / spacing;
const baseValue = ref(0);
const rotation = ref(0);
const selectedValue = computed(() => {
    const value = baseValue.value + (-1 * 100 * (rotation.value / 360)) / spacing;
    const valueRounded = Math.round(value * 10) / 10;

    return valueRounded;
});

const touchStartInfo = reactive({
    x: 0,
    y: 0,
    rotation: 0,
    direction: 0,
});

function setValue(value: number) {
    baseValue.value = Math.round(value);

    // reset rotation
    const dif = baseValue.value - value;
    rotation.value = dif * (360/availableElementsCount);

    emit('valueSelected', value);
}

function touchStart(e: TouchEvent) {
    if(!editor.value) return;
    e.preventDefault();

    const rect = editor.value.getBoundingClientRect();

    touchStartInfo.x = e.touches[0].pageX;
    touchStartInfo.y = e.touches[0].pageY;
    touchStartInfo.rotation = rotation.value;
    touchStartInfo.direction = touchStartInfo.x - rect.left >= (rect.width / 2) ? 1 : -1;
}

function touchMove(e: TouchEvent) {
    e.preventDefault();
    
    rotation.value = (touchStartInfo.rotation + (e.touches[0].pageY - touchStartInfo.y)) * touchStartInfo.direction;
}

function touchEnd(e: TouchEvent) {
    e.preventDefault();

    setValue(selectedValue.value);
}

onMounted(() => {
    setValue(props.hint || 0);
});
</script>

<style scoped lang="scss">
.mass-editor {
    position: relative;
    color: white;
}
</style>