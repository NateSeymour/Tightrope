import { createApp } from 'vue';
import { createRouter, createWebHistory } from 'vue-router';
import { createPinia } from 'pinia';
import { routes } from '@/routes';
import { VueQueryPlugin } from '@tanstack/vue-query';
import App from './App.vue';

import '@/style/global.scss';

// Vue Plugins
const router = createRouter({
    history: createWebHistory(),
    routes,
});

const pinia = createPinia();

// Create App
const app = createApp(App);

app.use(router);
app.use(pinia);
app.use(VueQueryPlugin);

app.mount('#app');
