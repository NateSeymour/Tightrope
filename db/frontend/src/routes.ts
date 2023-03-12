export const routes = [
    // Application
    { path: '/', component: () => import('@/view/application/HomeView.vue') },
    { path: '/preferences', component: () => import('@/view/application/PreferencesView.vue') },
    { path: '/error/:code', component: () => import('@/view/application/ErrorView.vue') },

    // Health
    { path: '/track/:metricName', component: () => import('@/view/metric/MetricTrackerView.vue') },
    { path: '/edit/:metricName/:measurementId', component: () => import('@/view/metric/EditMeasurementView.vue') },
    { path: '/log/:metricName', component: () => import('@/view/metric/LogMeasurementView.vue') },
];