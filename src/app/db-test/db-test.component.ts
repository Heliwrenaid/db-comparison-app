import { BasicPackageData, PackageData } from './../model/package';
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

  actions = ['Insert package', 'Get package by name', 'Remove comments of package', 'Get names of <n> sorted packages by given field and offset', 'Get <n> most voted packages basic data'];
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

  getPkgResult: QueryResult<PackageData> | void = undefined;
  getPkgForm = new FormGroup({
    targetDb: new FormControl(Db.SurrealDb, [Validators.required]),
    name: new FormControl('dropbox', [Validators.required]),
  })

  insertPkgResult: QueryResult<void> | void = undefined;
  insertPkgForm = new FormGroup({
    targetDb: new FormControl(Db.SurrealDb, [Validators.required]),
    pkgJson: new FormControl('', [Validators.required]),
  })

  removeCommentsResult: QueryResult<void> | void = undefined;
  removeCommentsForm = new FormGroup({
    targetDb: new FormControl(Db.SurrealDb, [Validators.required]),
    pkgName: new FormControl('test-7777', [Validators.required]),
  })

  constructor (private dbQueryService: DbQueryService) {}

  async getNamesOfSortedPackagesByName() {
    this.namesOfSortedPkgsResult = undefined;
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
    this.mostVotedPkgsResult = undefined;
    let data = this.mostVotedPkgsForm.value;
    this.dbQueryService.getMostVotedPackages(
        data.targetDb as Db,
        data.limit as number
      )
      .catch(err => console.error(err))
      .then(response => this.mostVotedPkgsResult = response)
  }

  async getPkg() {
    this.getPkgResult = undefined;
    let data = this.getPkgForm.value;
    this.dbQueryService.getPkg(
        data.targetDb as Db,
        data.name as string
      )
      .catch(err => console.error(err))
      .then(response => this.getPkgResult = response)
  }

  async insertPkg() {
    this.insertPkgResult = undefined;
    let data = this.insertPkgForm.value;
    let pkg: PackageData = JSON.parse(data.pkgJson as string);
    this.dbQueryService.insertPkg(
        data.targetDb as Db,
        pkg
      )
      .catch(err => console.error(err))
      .then(response => this.insertPkgResult = response)
  }

  async removeComments() {
    this.removeCommentsResult = undefined;
    let data = this.removeCommentsForm.value;
    this.dbQueryService.removeComments(
        data.targetDb as Db,
        data.pkgName as string
      )
      .catch(err => console.error(err))
      .then(response => this.removeCommentsResult = response)
  }

}
