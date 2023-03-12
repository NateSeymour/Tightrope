<template>
    <div class="loading-screen">

    </div>
</template>

<style lang="scss" scoped>
@use "@/style/colors";

.loading-screen {
    position: absolute;
    height: 100vh;
    width: 100vw;
    background: colors.$background;
    top: 0;
    left: 0;
    color: white;
    z-index: 100000;

    &.hidden {
        pointer-events: none;
        animation: come-to-life 50ms forwards;
    }

    @keyframes come-to-life {
        from {
            opacity: 100;
            display: block;
        }

        to {
            opacity: 0;
            display: hidden;
        }
    }
}
</style>