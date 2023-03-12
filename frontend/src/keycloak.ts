import Keycloak from 'keycloak-js';
import config from '@config/config';

let authQueue: Promise<void>[] = [];

export const keycloak = new Keycloak(config.authentication.keycloak);

export const initializeAuth = async () => {
    // Handle Authentication
    const authPromise = keycloak.init({
        onLoad: 'login-required',
        flow: 'hybrid',
        checkLoginIframe: true,
    }).then(async auth => {
        if(!auth) {
            keycloak.logout();
        }
    });

    authQueue.push(authPromise);

    return authPromise;
};

export const getAuth = async () => {
    if(!keycloak.authenticated && authQueue.length == 0) {
        await initializeAuth();
    }

    await Promise.all(authQueue);

    authQueue = [];
}; 
