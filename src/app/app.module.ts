import { ChartDataSeriesFromMapPipe } from './data/chart-data-series-from-map.pipe';
import { ChartDataSeriesPipe } from './data/chart-data-series.pipe';
import { NgModule } from "@angular/core";
import { BrowserModule } from "@angular/platform-browser";
import { AppComponent } from "./app.component";
import { NoopAnimationsModule } from '@angular/platform-browser/animations';
import { AppRoutingModule } from './app-routing.module';
import { DbQueryComponent } from './db-query/db-query.component'; 
import { FormsModule, ReactiveFormsModule } from "@angular/forms";
import {MatButtonModule} from '@angular/material/button'; 
import {MatFormFieldModule, MAT_FORM_FIELD_DEFAULT_OPTIONS} from '@angular/material/form-field'; 
import {MatInputModule} from '@angular/material/input';
import { DbTestComponent } from './db-test/db-test.component'; 
import {MatButtonToggleModule} from '@angular/material/button-toggle';
import {MatCheckboxModule} from '@angular/material/checkbox';
import {CdkAccordionModule} from '@angular/cdk/accordion';
import { NgChartsModule } from 'ng2-charts';
import { BarChartComponent } from './db-test/charts/bar-chart/bar-chart.component';

@NgModule({
  declarations: [
    AppComponent,
    DbQueryComponent,
    DbTestComponent,
    BarChartComponent,
    ChartDataSeriesPipe,
    ChartDataSeriesFromMapPipe,
  ],
  imports: [
    BrowserModule,
    NoopAnimationsModule,
    AppRoutingModule,
    FormsModule,
    ReactiveFormsModule,
    MatButtonModule,
    MatFormFieldModule,
    MatInputModule,
    MatButtonToggleModule,
    MatCheckboxModule,
    CdkAccordionModule,
    NgChartsModule
  ],
  providers: [
    {provide: MAT_FORM_FIELD_DEFAULT_OPTIONS, useValue: {appearance: 'fill'}}
  ],
  bootstrap: [AppComponent],
})
export class AppModule {}

