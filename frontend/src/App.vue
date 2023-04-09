<template>
    <Loading :class="appStore.loggedIn ? 'hidden' : 'loading'" />

    <UserBar />

    <div class="content">
        <NavigationMenu class="navigation-menu" />

        <div class="viewport-container">
            <div class="viewport">
                <router-view :key="route.fullPath"></router-view>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onBeforeMount } from 'vue';
import { initializeAuth } from '@/keycloak';
import { useAppStore } from '@/store/app';
import { keycloak } from '@/keycloak';
import { useRoute, useRouter } from 'vue-router';
import Loading from '@/view/LoadingView.vue';
import UserBar from '@/component/UserBar.vue';
import NavigationMenu from '@/component/NavigationMenu.vue';

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
@use "@/style/responsive";

html {
    overflow: hidden;
    width: 100%;
    background: colors.$background;

    body {
        height: 100%;
        width: 100%;
        position: fixed;
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

            @include responsive.desktop {
                .content {
                    width: 100%;
                    height: 100%;
                    display: grid;
                    grid-template-columns: 1fr 4fr;
                }

                .viewport-container {
                    overflow-y: auto;
                    width: 100%;
                    height: 100%;
                    display: flex;
                    justify-content: center;
                    flex-direction: row;

                    .viewport {
                        width: 75%;
                        height: 100%;
                        padding: 1em;
                    }
                }
            }
        }
    }
}
</style>
