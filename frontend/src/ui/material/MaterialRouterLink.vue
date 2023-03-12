<template>
    <div class="material-router-link">
        <MaterialOverlay @click="router.push(to)" :selectable="false" />

        <RouterLink ref="link" :to="to"><slot></slot></RouterLink>
    </div>
</template>

<script setup lang="ts">
import { RouterLink, useRouter } from 'vue-router';
import MaterialOverlay from './MaterialOverlay.vue';

const props = defineProps<{
    to: string,
}>();

const router = useRouter();
</script>

<style scoped lang="scss">
.material-router-link {
    padding: 1em;

    a {
        text-decoration: none;
        font-size: 20px;
        color: white;
        user-select: none;
    }
}
</style>