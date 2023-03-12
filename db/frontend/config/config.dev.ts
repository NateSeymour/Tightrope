export default {
    authentication: {
        keycloak: {
            url: 'https://auth.seymour.global',
            realm: 'development',
            clientId: 'fitness-app',
        },
    },
    api: {
        base: 'http://localhost:8000/v1',
    },
};