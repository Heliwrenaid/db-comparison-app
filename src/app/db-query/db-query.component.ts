import { DbQueryService } from './../services/db-query.service';
import { Db, QueryResult } from './../model/query';
import { Component, Input } from '@angular/core';

@Component({
  selector: 'app-db-query',
  templateUrl: './db-query.component.html',
  styleUrls: ['./db-query.component.scss']
})
export class DbQueryComponent {
  @Input()
  query = '';

  result: QueryResult<string> = {
    result: '',
    duration: {
      nanos: 0,
      secs: 0
    }
  };
  targetDb = Db.SurrealDb;
  doRepeat = false;
  numberOfRepeatings = 0;
  timeOnly = false;

  timeData: Map<number, number> = new Map();
  displayChart = false;

  constructor (private dbQueryService: DbQueryService) {}

  async runQuery() {
    if (this.query && this.targetDb) {
      this.displayChart = false;
      if (this.doRepeat) {
        let nanosec = 0;
        for (let i = 0; i < this.numberOfRepeatings; i++) {
          await this.dbQueryService.runQuery(this.query, this.targetDb)
            .then(result => nanosec += result.duration.secs * 1000000000 + result.duration.nanos)
            .catch(error => this.result.result = error.message);
        }
        
        this.result.duration.nanos = 0;
        this.result.duration.secs = 0;
        this.result.result = "Nanoseconds:" + nanosec / this.numberOfRepeatings;
      } else {
        this.dbQueryService.runQuery(this.query, this.targetDb)
          .then(result => this.result = result)
          .catch(error => this.result.result = error.message)
      }
    }
  }

  async getQueryTime() {
    if (this.query && this.targetDb) {
      this.displayChart = false;
      if (this.doRepeat) {
        this.timeData = new Map();
        let timeSum = 0;
        for (let i = 0; i < this.numberOfRepeatings; i++) {
          await this.dbQueryService.getQueryTime(this.query, this.targetDb)
            .then(duration => {
              let durationNs = duration.secs * 1000000000 + duration.nanos;
              timeSum += durationNs;
              this.timeData.set(i, durationNs);
            })
            .catch(error => this.result.result = error.message);
        }
        this.result.duration.nanos = 0;
        this.result.duration.secs = 0;
        this.result.result = "" + timeSum / this.numberOfRepeatings;
        this.displayChart = true;
      } else {
        this.dbQueryService.getQueryTime(this.query, this.targetDb)
          .then(duration => {
            this.result.result = "OK";
            this.result.duration = duration;
          })
          .catch(error => this.result.result = error.message);
      }
    }
  }
}
