import 'chartjs-plugin-annotation';
import 'chart.js';

declare module 'chartjs-plugin-annotation' {
  interface AnnotationTypeRegistry {
    doughnutLabel: {
      type: 'doughnutLabel';
      content?: (ctx: any) => any;
      font?: { size?: number };
    };
  }
}

declare module 'chart.js' {
  interface ChartMeta {
    total?: number;
  }
}

declare module '@sgratzl/chartjs-chart-boxplot';
