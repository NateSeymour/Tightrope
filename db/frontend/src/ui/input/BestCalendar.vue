<template>
    <div class="best-calendar">
        <div ref="carouselEl" class="carousel">
            <div @touchmove="touchMove"
                @touchstart="touchStart"
                @touchend="touchEnd"
                v-for="dynoDate in dateCarousel" 
                class="page" 
                :style="`transform: translateX(calc(-100% + ${scrollValuePx}px))`"
                :key="dynoDate.getUTCSeconds()">
                <div class="top">
                    <h4 class="title">{{ monthYearFormat.format(dynoDate) }}</h4>
                </div>
                <div class="calendar-body">
                    <div @click="() => { year = dynoDate.getFullYear(); month = dynoDate.getMonth(); date = i; $emit('dateSelected', selectedTime); }" 
                        v-for="i in daysInMonth(dynoDate)" 
                        :class="date == i && month == dynoDate.getMonth() && year == dynoDate.getFullYear() ? 'selected' : ''" 
                        class="calendar-el"
                        :key="i">

                        <span>{{ i }}</span>
                        <svg>
                            <circle></circle>
                        </svg>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { monthYearFormat } from '@/intl';
import { computed } from 'vue';
import { ref, Ref, ComputedRef, onMounted } from 'vue';

const emit = defineEmits(['dateSelected']);

const props = defineProps<{
    hint?: number,
}>();

const initialDate: Ref<Date> = ref(new Date(props.hint || Date.now()));

const year: Ref<number> = ref(initialDate.value.getFullYear());
const month: Ref<number> = ref(initialDate.value.getMonth());
const date: Ref<number> = ref(initialDate.value.getDate());

const selectedTime: ComputedRef<Date> = computed(() => new Date(year.value, month.value, date.value));
const displayTime: Ref<Date> = ref(new Date(initialDate.value));

const dateCarousel: ComputedRef<Date[]> = computed(() => [
    new Date(displayTime.value.getFullYear(), displayTime.value.getMonth() - 1, displayTime.value.getDate()),
    displayTime.value,
    new Date(displayTime.value.getFullYear(), displayTime.value.getMonth() + 1, displayTime.value.getDate()),
]);

const canScroll: Ref<boolean> = ref(true);
const scrollValuePx: Ref<number> = ref(0);
const touchStartX: Ref<number> = ref(0);

const carouselEl: Ref<HTMLElement | undefined> = ref();

function daysInMonth(myDate: Date) {
    return new Date(myDate.getFullYear(), myDate.getMonth() + 1, 0).getDate();
}

function touchEnd() {
    const el = carouselEl.value!;
    const anims: Animation[] = [];

    if(scrollValuePx.value >= (el.clientWidth / 4) || scrollValuePx.value <= -(el.clientWidth / 4)) {
        canScroll.value = false;

        // @ts-ignore
        for(const page of el.children) {
            anims.push(page.animate(
                [
                    {
                        transform: `translateX(calc(-100% + ${scrollValuePx.value}px))`,
                    },
                    {
                        transform: `translateX(${scrollValuePx.value > 0 ? '0' : '-200%'})`,
                    },
                ],
                {
                    duration: 100,
                    fill: 'forwards',
                    easing: 'ease-out',
                },
            ));
        }

        setTimeout(() => {
            displayTime.value = new Date(displayTime.value.getFullYear(), displayTime.value.getMonth() + (scrollValuePx.value > 0 ? -1 : 1), displayTime.value.getDate());
            for(const anim of anims) anim.cancel();
            scrollValuePx.value = 0;
            canScroll.value = true;
        }, 100);
    } else {
        canScroll.value = false;

        // @ts-ignore
        for(const page of el.children) {
            anims.push(page.animate(
                [
                    {
                        transform: `translateX(calc(-100% + ${scrollValuePx.value}px))`,
                    },
                    {
                        transform: 'translateX(-100%)',
                    },
                ],
                {
                    duration: 100,
                    fill: 'forwards',
                    easing: 'ease-out',
                },
            ));
        }

        setTimeout(() => {
            for(const anim of anims) anim.cancel();
            scrollValuePx.value = 0;
            canScroll.value = true;
        }, 100);
    }
}

function touchStart(e: TouchEvent) {
    touchStartX.value = e.touches[0].clientX;
    //canScroll.value = true;
}

function touchMove(e: TouchEvent) {
    if(!canScroll.value) return;

    scrollValuePx.value = e.touches[0].clientX - touchStartX.value;
}

onMounted(() => {
    emit('dateSelected', selectedTime.value);
});
</script>

<style lang="scss" scoped>
@use "@/style/colors";

.best-calendar {
    color: white;
    
    .carousel {
        width: 100%;
        height: 100%;
        display: grid;
        grid-template-columns: 100% 100% 100%;
        overflow: hidden;

        .page {
            height: 100%;
            width: 100%;
            display: flex;
            flex-direction: column;

            .title {
                text-align: center;
                margin-bottom: 0.75em;
            }

            .calendar-body {
                display: grid;
                grid-template-columns: repeat(7, 1fr);
                height: 100%;
                width: 100%;

                .calendar-el {
                    position: relative;
                    display: flex;
                    justify-content: center;
                    align-items: center;

                    span {
                        pointer-events: none;
                        z-index: 1;
                    }

                    svg {
                        position: absolute;
                        top: 0;
                        left: 0;
                        width: 100%;
                        height: 100%;
                        pointer-events: none;
                        z-index: 0;

                        circle {
                            cx: 50%;
                            cy: 50%;
                            r: 0;
                            fill: colors.$accent;
                        }
                    }

                    &.selected {
                        svg > circle {
                            animation: day-in-out 80ms ease-out forwards;
                        }
                    }

                    @keyframes day-in-out {
                        from {
                            r: 0;
                        }

                        to {
                            r: 45%;
                        }
                    }
                }
            }
        }
    }    
}
</style>