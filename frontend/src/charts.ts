import { use } from 'echarts/core';
import { SVGRenderer } from 'echarts/renderers';
import { LineChart } from 'echarts/charts';
import { TooltipComponent, GridComponent, TitleComponent, DataZoomComponent } from 'echarts/components';

export const initialized = false;

export function initializeCharts() {
    if(!initialized) {
        use([SVGRenderer, LineChart, TooltipComponent, GridComponent, TitleComponent, DataZoomComponent]);
    }
}