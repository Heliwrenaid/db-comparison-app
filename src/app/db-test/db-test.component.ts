import { Component } from '@angular/core';
import { Db } from '../model/query';
import { DbQueryService } from '../services/db-query.service';

@Component({
  selector: 'app-db-test',
  templateUrl: './db-test.component.html',
  styleUrls: ['./db-test.component.scss']
})
export class DbTestComponent {

  constructor (private dbQueryService: DbQueryService) {}

  async sortPkgsByFieldWithLimit() {
    // this.dbQueryService.sortPkgsByFieldWithLimit(Db.SurrealDb, "popularity", 0, 5)
    //   .catch(err => console.error(err))
    //   .then(response => console.log(response))
    this.dbQueryService.getMostVotedPackages(Db.Skytable, 5)
      .catch(err => console.error(err))
      .then(response => console.log(response))
  }

}
