import { ChartDataSeries } from './../model/chart';
import { Pipe, PipeTransform } from '@angular/core';

@Pipe({name: 'chartDataSeriesFromMap'})
export class ChartDataSeriesFromMapPipe implements PipeTransform {
  transform(data: Map<any, any>, seriesLabel: string): ChartDataSeries {
    let chartData: ChartDataSeries = {
      labels: [],
      data: [],
      seriesLabel: seriesLabel
    };
    
    let obj = data as Object;
    let key: keyof typeof obj;
    for (key in obj) {
      chartData.data.push(obj[key]);
      chartData.labels.push(key)
    }
    return chartData;
  }
}