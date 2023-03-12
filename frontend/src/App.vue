<template>
    <Loading :class="appStore.loggedIn ? 'hidden' : 'loading'" />

    <UserBar />
    <NavigationMenu />

    <div class="viewport-container">
        <router-view :key="route.fullPath"></router-view>
    </div>
</template>

<script setup lang="ts">
import { onBeforeMount } from 'vue';
import { initializeAuth } from '@/keycloak';
import { useAppStore } from './store/app';
import { keycloak } from '@/keycloak';
import Loading from '@/view/LoadingView.vue';
import UserBar from '@/component/UserBar.vue';
import NavigationMenu from '@/component/NavigationMenu.vue';
import { useRoute, useRouter } from 'vue-router';

const router = useRouter();
const route = useRoute();
const appStore = useAppStore();

router.afterEach(() => {
    appStore.navMenuOpen = false;
});

onBeforeMount(() => {
    window.oncontextmenu = (e: MouseEvent) => {
        const pointerEvent = e as PointerEvent;

        if (pointerEvent.pointerType == 'touch') {
            return false;
        }
    };

    initializeAuth().then(() => {
        if (!keycloak.tokenParsed) throw 'Authentication Error!';

        appStore.profile.username = keycloak.tokenParsed['preferred_username'];
        appStore.profile.firstName = keycloak.tokenParsed['given_name'];
        appStore.profile.lastName = keycloak.tokenParsed['family_name'];

        appStore.loggedIn = true;
    });
});
</script>

<style lang="scss">
@use "@/style/colors";

html {
    overflow: hidden;
    width: 100%;
    background: colors.$background;

    body {
        height: 100%;
        width: 100%;
        position: fixed;
        overflow-y: scroll;
        -webkit-overflow-scrolling: touch;
        background: colors.$background;

        #app {
            width: 100%;
            height: 100%;
            max-height: 100%;
            max-width: 100%;
            background: colors.$background;
            display: grid;
            grid-template-rows: 4em auto;
            position: relative;

            .viewport-container {
                overflow-y: scroll;
            }
        }
    }
}
</style>
