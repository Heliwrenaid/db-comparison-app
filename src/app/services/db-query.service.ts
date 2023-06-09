import { QueryResult, Db, QueryCommand, Duration } from './../db-query/query-result-model';
import { Injectable } from "@angular/core";
import { invoke } from '@tauri-apps/api/tauri';

@Injectable({
    providedIn: 'root',
})
export class DbQueryService {
    
    public runQuery(query: string, targetDb: Db): Promise<QueryResult> {
        let queryCommand: QueryCommand = {
            query: query,
            target_db: targetDb
        }
        return invoke<QueryResult>('run_query', { 'queryCommand': queryCommand })
    }

    public getQueryTime(query: string, targetDb: Db): Promise<Duration> {
        let queryCommand: QueryCommand = {
            query: query,
            target_db: targetDb
        }
        return invoke<Duration>('get_query_time', { 'queryCommand': queryCommand })
    }

    public sortPkgsByFieldWithLimit(targetDb: Db, field: string, limitStart: number, limitEnd: number) {
        return invoke<QueryResult>('sort_pkgs_by_field_with_limit', 
            { 'targetDb': targetDb, 'field': field, 'limitStart': limitStart, 'limitEnd': limitEnd}
        )
    }

}