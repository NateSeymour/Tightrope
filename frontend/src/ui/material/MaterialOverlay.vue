<template>
    <div 
        ref="overlay" 
        class="material-overlay"
        @touchstart="touchStart" 
        @touchend="touchEnd"
        @touchmove="touchMove"
        @mousedown="touchStart"
        @mouseup="touchEnd">
        <svg :viewBox="`0 0 ${overlaySize.width} ${overlaySize.height}`" xmlns="http://www.w3.org/2000/svg">
            <circle 
                :data-touch-state="touchState"
                :data-selected="selected" 
                :cx="circleCoordinates.x" 
                :cy="circleCoordinates.y" 
                r="0" />
        </svg>
    </div>
</template>

<script setup lang="ts">
import { Ref, ref, reactive, computed } from 'vue';

const emit = defineEmits(['click', 'select']);

const props = defineProps<{
    selectable: boolean,
}>();

const overlay: Ref<HTMLElement | undefined> = ref();
const overlaySize = computed(() => {
    if(!overlay.value) return { width: 0, height: 0 };

    const rect = overlay.value.getBoundingClientRect();
    return {
        width: rect.width,
        height: rect.height,
    };
});

const touchState: Ref<'inactive' | 'touchstart' | 'touchend'> = ref('inactive');
const startTime = ref(0);
const selectionThreshhold = 250;
const selected = ref(false);

const circleCoordinates = reactive({ x: 0, y: 0 });

function setCoordinates(e: MouseEvent | TouchEvent) {
    if(e instanceof MouseEvent) {
        circleCoordinates.x = e.offsetX;
        circleCoordinates.y = e.offsetY;
    } else {
        const rect = (e.target as HTMLElement).getBoundingClientRect();
        circleCoordinates.x = e.targetTouches[0].pageX - rect.left;
        circleCoordinates.y = e.targetTouches[0].pageY - rect.top;
    }
}

function touchStart(e: TouchEvent) {
    touchState.value = 'touchstart';
    startTime.value = Date.now();

    setCoordinates(e);
}

function touchMove(e: TouchEvent) {
    // Cancel touch if moved to allow for normal scrolling
    touchState.value = 'touchend';
}

function touchEnd(e: TouchEvent) {
    if(touchState.value === 'touchend') return;

    if(props.selectable) {
        const holdTime = Date.now() - startTime.value;

        if(holdTime >= selectionThreshhold || selected.value) {
            navigator.vibrate(50);
            selected.value = !selected.value;

            emit('select', selected.value);

            touchState.value = 'touchend';

            return;
        }
    }

    emit('click');
    touchState.value = 'touchend';
}
</script>

<style lang="scss" >
*:has(> .material-overlay) {
    position: relative;
}

.material-overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    cursor: pointer;

    svg {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        pointer-events: none;

        circle {
            fill: rgba(255,255,255,0.1);
            r: 100%;

            &[data-selected=false] {
                &[data-touch-state='inactive'] {
                    r: 0;
                }

                &[data-touch-state='touchstart'] {
                    @keyframes touchstart {
                        from {
                            r: 0;
                        }

                        to {
                            r: 100%;
                        }
                    }
                    animation: touchstart 350ms ease-out forwards;
                }

                &[data-touch-state='touchend'] {
                    transition: 200ms;
                    r: 100%;
                    fill: rgba(255,255,255,0);
                }
            }

            &[data-selected=true] {
                fill: rgba(255,255,255,0.1);
                r: 100%;
            }
        }
    }
}
</style>