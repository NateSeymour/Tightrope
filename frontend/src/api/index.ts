import config from '@config/config';
import { keycloak, getAuth } from '@/keycloak';

export const fetchApi = async (endpoint: string, method: string, body?: BodyInit) => {
    await getAuth();

    if(!keycloak.token) {
        throw 'Authentication Error!';
    } else {
        return fetch(`${config.api.base}${endpoint}`, {
            method: method,
            headers: {
                'Authorization': keycloak.token,
                'Content-Type': (body ? 'application/json' : ''),
            },
            body,
        });
    }
};