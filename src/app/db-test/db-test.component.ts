import { BasicPackageData } from './../model/package';
import { QueryResult } from './../model/query';
import { Component } from '@angular/core';
import { Db } from '../model/query';
import { DbQueryService } from '../services/db-query.service';
import { FormGroup, FormControl, Validators } from '@angular/forms';

@Component({
  selector: 'app-db-test',
  templateUrl: './db-test.component.html',
  styleUrls: ['./db-test.component.scss']
})
export class DbTestComponent {

  actions = ['Get names of <n> sorted packages by given field and offset', 'Get <n> most voted packages basic data'];
  expandedIndex = 0;

  namesOfSortedPkgsResult: QueryResult<string[]> | void = undefined;
  getNamesOfSortedPkgsForm = new FormGroup({
    targetDb: new FormControl(Db.SurrealDb, [Validators.required]),
    fieldName: new FormControl('popularity', [Validators.required]),
    limit: new FormControl(5, [Validators.required]),
    offset: new FormControl(0, [Validators.required])
  })

  mostVotedPkgsResult: QueryResult<BasicPackageData[]> | void = undefined;
  mostVotedPkgsForm = new FormGroup({
    targetDb: new FormControl(Db.SurrealDb, [Validators.required]),
    limit: new FormControl(5, [Validators.required]),
  })

  constructor (private dbQueryService: DbQueryService) {}

  async getNamesOfSortedPackagesByName() {
    let data = this.getNamesOfSortedPkgsForm.value;
    this.dbQueryService.sortPkgsByFieldWithLimit(
        data.targetDb as Db,
        data.fieldName as string,
        data.offset as number,
        data.limit as number
      )
      .catch(err => console.error(err))
      .then(response => this.namesOfSortedPkgsResult = response)
  }

  async getMostVotedPkgs() {
    let data = this.mostVotedPkgsForm.value;
    
    this.dbQueryService.getMostVotedPackages(
        data.targetDb as Db,
        data.limit as number
      )
      .catch(err => console.error(err))
      .then(response => this.mostVotedPkgsResult = response)
  }

}
