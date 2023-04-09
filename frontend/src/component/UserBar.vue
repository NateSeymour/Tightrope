<template>
    <div class="user-bar">
        <div class="menu">
            <svg 
                @click="appStore.navMenuOpen = !appStore.navMenuOpen" 
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 10 10"
                class="hamburger">
                <path
                    stroke="white"
                    stroke-width="1"
                    d="M 0,2 h 10" />

                <path
                    :class="['middle', appStore.navMenuOpen ? 'open' : 'closed']"
                    stroke="white"
                    stroke-width="1"
                    d="M 0,5 h 10" />

                <path
                    stroke="white"
                    stroke-width="1"
                    d="M 0,8 h 10" />
            </svg>
        </div>
        <div class="session-info">
            <span>{{ appStore.profile.firstName || appStore.profile.username }}</span>
        </div>
    </div>
</template>

<script setup lang="ts">
import { useAppStore } from '@/store/app';

const appStore = useAppStore();
</script>

<style scoped lang="scss">
@use "@/style/colors";
@use "@/style/responsive";

.user-bar {
    height: 4em;
    width: 100%;
    display: flex;
    flex-direction: row;
    align-items: center;
    background: colors.$background-depth;
    justify-content: space-between;
    padding: 1em;
    color: white;

    .menu {
        width: 2em;
        height: 2em;
        margin-left: 0.25em;

        @include responsive.desktop {
            .hamburger {
                display: none;
            }
        }

        @include responsive.mobile {
            .hamburger {
                width: 100%;
                height: 100%;

                .middle {
                    &.open {
                        transition: 250ms;
                        d: path("M 5,5 h 10");
                    }

                    &.closed {
                        transition: 250ms;
                        d: path("M 0,5 h 10");
                    }
                }
            }
        }
    }
}
</style>