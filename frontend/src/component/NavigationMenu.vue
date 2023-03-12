<template>
    <div :class="['navigation-menu', appStore.navMenuOpen ? 'open' : 'closed']">
        <nav>
            <div class="application submenu">   
                <h3>Application</h3>
                <MaterialRouterLink to="/">Home</MaterialRouterLink>
                <MaterialRouterLink to="/preferences">Preferences</MaterialRouterLink>
            </div>
            

            <div class="metrics submenu" v-if="metrics">
                <h3>Metrics</h3>
                <MaterialRouterLink v-for="metric of metrics" :key="metric.metric_id" :to="`/track/${metric.name}`">{{ metric.name }}</MaterialRouterLink>
            </div>
        </nav>
    </div>
</template>

<script setup lang="ts">
import { Metric } from '@/api/metric';
import { useAppStore } from '@/store/app';
import { useQuery } from '@tanstack/vue-query';
import MaterialRouterLink from '@/ui/material/MaterialRouterLink.vue';

const appStore = useAppStore();

const { data: metrics } = useQuery({
    queryKey: ['metric-list'],
    queryFn: Metric.fetchList,
});
</script>

<style scoped lang="scss">
@use "@/style/colors";

.navigation-menu {
    position: fixed;
    top: 4em;
    left: -100%;
    width: 100%;
    height: calc(100% - 4em);
    transition: 350ms;
    background: colors.$background-depth;
    overflow-y: scroll;
    overflow-x: hidden;
    z-index: 999999;

    &.open {
        left: 0;
    }

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
}
</style>