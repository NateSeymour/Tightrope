<template>
    <div class="material-button">
        <MaterialOverlay :selectable="false" />
        <button>
            <slot></slot>
        </button>
    </div>
</template>

<script setup lang="ts">
import MaterialOverlay from './MaterialOverlay.vue';
</script>

<style scoped lang="scss">
@use "@/style/colors";

.material-button {
    height: 3em;

    button {
        width: 100%;
        height: 100%;
        border: none;
        border-radius: 0.25em;
        background: colors.$accent;
        color: white;
        user-select: none;
    }
}
</style>