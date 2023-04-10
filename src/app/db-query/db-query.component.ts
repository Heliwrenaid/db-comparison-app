import { DbQueryService } from './../services/db-query.service';
import { QueryResult } from './query-result-model';
import { Component, Input } from '@angular/core';

@Component({
  selector: 'app-db-query',
  templateUrl: './db-query.component.html',
  styleUrls: ['./db-query.component.scss']
})
export class DbQueryComponent {
  @Input()
  query = ''

  result = ''

  constructor (private dbQueryService: DbQueryService) {}

  runQuery() {
    if (this.query) {
      this.dbQueryService.run_query(this.query)
        .then(response => this.result = response.result)
    }
  }
}
