import { ChartDataSeries } from './../model/chart';
import { Pipe, PipeTransform } from '@angular/core';

@Pipe({name: 'chartDataSeries'})
export class ChartDataSeriesPipe implements PipeTransform {
  transform(data: Object[], xField: string, yField: string, seriesLabel: string): ChartDataSeries {
    let chartData: ChartDataSeries = {
      labels: [],
      data: [],
      seriesLabel: seriesLabel
    };
    data.forEach(item => {
      chartData.labels.push(item[xField as keyof typeof item])
      chartData.data.push(item[yField as keyof typeof item])
    });
    return chartData;
  }
}