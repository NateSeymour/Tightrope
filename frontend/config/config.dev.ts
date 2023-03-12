export default {
    authentication: {
        keycloak: {
            url: 'https://auth.seymour.global',
            realm: 'development',
            clientId: 'tightrope',
        },
    },
    api: {
        base: 'http://localhost:8000/v1',
    },
};