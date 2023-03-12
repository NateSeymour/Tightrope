import { defineStore } from 'pinia';
import { reactive, ref } from 'vue';

export interface Profile {
    username?: string,
    firstName?: string,
    lastName?: string,
}

export const useAppStore = defineStore('app', () => {
    const profile: Profile = reactive({});

    const loggedIn = ref(false);
    const navMenuOpen = ref(false);

    return { profile, loggedIn, navMenuOpen };
});