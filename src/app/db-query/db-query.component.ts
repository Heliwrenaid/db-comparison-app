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
  query = ''

  result: QueryResult = {
    result: '',
    duration: {
      nanos: 0,
      secs: 0
    }
  }
  targetDb = Db.SurrealDb;

  constructor (private dbQueryService: DbQueryService) {}

  runQuery() {
    if (this.query && this.targetDb) {
      this.dbQueryService.run_query(this.query, this.targetDb)
        .then(result => this.result = result)
    }
  }
}
