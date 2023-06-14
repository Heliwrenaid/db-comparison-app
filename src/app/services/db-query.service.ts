import { PackageData } from './../model/package';
import { QueryResult, Db, QueryCommand, Duration } from './../model/query';
import { Injectable } from "@angular/core";
import { invoke } from '@tauri-apps/api/tauri';
import { BasicPackageData } from '../model/package';

@Injectable({
    providedIn: 'root',
})
export class DbQueryService {
    
    public runQuery(query: string, targetDb: Db): Promise<QueryResult<string>> {
        let queryCommand: QueryCommand = {
            query: query,
            target_db: targetDb
        }
        return invoke<QueryResult<string>>('run_query', { 'queryCommand': queryCommand })
    }

    public getQueryTime(query: string, targetDb: Db): Promise<Duration> {
        let queryCommand: QueryCommand = {
            query: query,
            target_db: targetDb
        }
        return invoke<Duration>('get_query_time', { 'queryCommand': queryCommand })
    }

    public sortPkgsByFieldWithLimit(targetDb: Db, field: string, limitStart: number, limitEnd: number) {
        return invoke<QueryResult<string[]>>('sort_pkgs_by_field_with_limit', 
            { 'targetDb': targetDb, 'field': field, 'limitStart': limitStart, 'limitEnd': limitEnd}
        )
    }

    public getMostVotedPackages(targetDb: Db, limit: number) {
        return invoke<QueryResult<BasicPackageData[]>>('get_most_voted_pkgs', 
            { 'targetDb': targetDb, 'number': limit }
        )
    }

    public insertPkg(targetDb: Db, pkg: PackageData) {
        return invoke<QueryResult<void>>('insert_pkg', 
            { 'targetDb': targetDb, 'pkg': pkg }
        )
    }

    public getPkg(targetDb: Db, name: string) {
        return invoke<QueryResult<PackageData>>('get_pkg', 
            { 'targetDb': targetDb, 'name': name }
        )
    }

    public removeComments(targetDb: Db, pkgName: string) {
        return invoke<QueryResult<void>>('remove_comments', 
            { 'targetDb': targetDb, 'pkgName': pkgName }
        )
    }

    public getPackagesOccurencesInDeps(targetDb: Db, pkgNames: string[]) {
        return invoke<QueryResult<Map<string, number>>>('get_packages_occurences_in_deps', 
            { 'targetDb': targetDb, 'pkgNames': pkgNames }
        )
    }

}