<template>
    <div class="error">
        <h1>{{ errorMessages[errorCode].title }}</h1>
        <p>{{ errorMessages[errorCode].description }}</p>
        <RouterLink to="/">Go Home</RouterLink>
    </div>
</template>

<script lang="ts" setup>
import { useRoute, RouterLink } from 'vue-router';

const route = useRoute();

const errorCode = (route.params.code as string) || '404';
const errorMessages: { [key: string]: { title: string, description: string }} = {
    '404': {
        title: 'Sorry, that page couldn\'t be found!',
        description: '',
    },
};
</script>