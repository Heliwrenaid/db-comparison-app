import { ChartDataSeries } from './../../../model/chart';
import { Component, ViewChild, Input, OnInit, OnChanges } from '@angular/core';
import { ChartConfiguration, ChartData, ChartType } from 'chart.js';
import { BaseChartDirective } from 'ng2-charts';

@Component({
  selector: 'app-bar-chart',
  templateUrl: './bar-chart.component.html',
  styleUrls: [ './bar-chart.component.scss' ],
})
export class BarChartComponent implements OnInit, OnChanges {
  @ViewChild(BaseChartDirective) 
  chart: BaseChartDirective | undefined;

  @Input()
  chartData!: ChartDataSeries;

  barChartOptions: ChartConfiguration['options'] = {
    responsive: true,
    plugins: {
      legend: {
        display: true,
        labels: {
          color: '#fff7f7'
        }
      },
    },
    scales: {
      x: {
        ticks: { color: '#fff7f7'}
      },
      y: {
        ticks: { color: '#fff7f7'}
      }
    }
  };
  barChartType: ChartType = 'bar';
  barChartPlugins = [];

  barChartData: ChartData<'bar'> = {
    labels: [],
    datasets: [
      { 
        data: [], 
        label: '',
        backgroundColor: '#d9600f',
      },
    ],
  };

  ngOnInit(): void {
    this.updateChart();
  }

  ngOnChanges(): void {
    this.updateChart();
  }

  private updateChart(): void {
    this.barChartData.labels = this.chartData.labels;
    this.barChartData.datasets[0].label = this.chartData.seriesLabel;
    this.barChartData.datasets[0].data = this.chartData.data;
    this.chart?.update();
  }
}
