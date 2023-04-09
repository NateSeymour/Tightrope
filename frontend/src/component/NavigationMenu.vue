<template>
    <div :class="['navigation-menu', appStore.navMenuOpen ? 'open' : 'closed']">
        <nav>
            <div class="tightrope submenu">
                <h3>Tightrope</h3>
                <MaterialRouterLink to="/">Home</MaterialRouterLink>
                <MaterialRouterLink to="/preferences">Preferences</MaterialRouterLink>
            </div>
            <div class="application submenu">   
                <h3>Apps</h3>
                <MaterialRouterLink  :to="`/metric`">Metrics</MaterialRouterLink>
            </div>
        </nav>
    </div>
</template>

<script setup lang="ts">
import { useAppStore } from '@/store/app';
import MaterialRouterLink from '@/ui/material/MaterialRouterLink.vue';

const appStore = useAppStore();
</script>

<style scoped lang="scss">
@use "@/style/colors";
@use "@/style/responsive";

.navigation-menu {
    background: colors.$background-depth;
    overflow-y: auto;
    overflow-x: hidden;
    z-index: 999999;

    nav {
        display: flex;
        flex-direction: column;
        color: white;
        padding: 1em;

        .material-router-link {
            width: 100%;
        }

        .submenu {
            margin-bottom: 2em;
        }
    }

    @include responsive.mobile {
        position: fixed;
        top: 4em;
        left: -100%;
        transition: 350ms;
        width: 100%;
        height: calc(100% - 4em);

        &.open {
            left: 0;
        }
    }
}
</style>