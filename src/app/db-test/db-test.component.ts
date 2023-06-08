import { Component } from '@angular/core';
import { Db } from '../db-query/query-result-model';
import { DbQueryService } from '../services/db-query.service';

@Component({
  selector: 'app-db-test',
  templateUrl: './db-test.component.html',
  styleUrls: ['./db-test.component.scss']
})
export class DbTestComponent {

  constructor (private dbQueryService: DbQueryService) {}

  async sortPkgsByFieldWithLimit() {
    this.dbQueryService.sortPkgsByFieldWithLimit(Db.Redis, "votes", 0, 5)
      .catch(err => console.error(err))
      .then(response => console.log(response))
  }

}
