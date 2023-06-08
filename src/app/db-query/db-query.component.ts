import { DbQueryService } from './../services/db-query.service';
import { Db, QueryResult } from './query-result-model';
import { Component, Input } from '@angular/core';

@Component({
  selector: 'app-db-query',
  templateUrl: './db-query.component.html',
  styleUrls: ['./db-query.component.scss']
})
export class DbQueryComponent {
  @Input()
  query = '';

  result: QueryResult = {
    result: '',
    duration: {
      nanos: 0,
      secs: 0
    }
  };
  targetDb = Db.SurrealDb;
  doRepeat = false;
  numberOfRepeatings = 0;

  constructor (private dbQueryService: DbQueryService) {}

  async runQuery() {
    if (this.query && this.targetDb) {
      if (this.doRepeat) {
        let nanosec = 0;
        for (let i = 0; i < this.numberOfRepeatings; i++) {
          await this.dbQueryService.run_query(this.query, this.targetDb)
            .then(result => nanosec += result.duration.secs * 1000000000 + result.duration.nanos);
        }
        
        this.result.duration.nanos = 0;
        this.result.duration.secs = 0;
        this.result.result = "Nanoseconds:" + nanosec / this.numberOfRepeatings;
      } else {
        this.dbQueryService.run_query(this.query, this.targetDb)
          .then(result => this.result = result);
      }
    }
  }
}
