export const routes = [
    // Tightrope
    { path: '/', component: () => import('@/view/HomeView.vue') },
    { path: '/preferences', component: () => import('@/view/PreferencesView.vue') },
    { path: '/error/:code', component: () => import('@/view/ErrorView.vue') },

    // Metric
    { path: '/metric', component: () => import('@/app/metric/view/MetricHomeView.vue') },
    { path: '/metric/track/:metricName', component: () => import('@/app/metric/view/MetricTrackerView.vue') },
    { path: '/metric/edit/:metricName/:measurementId', component: () => import('@/app/metric/view/EditMeasurementView.vue') },
    { path: '/metric/log/:metricName', component: () => import('@/app/metric/view/LogMeasurementView.vue') },
];