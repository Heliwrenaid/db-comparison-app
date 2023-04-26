import { QueryResult, Db, QueryCommand } from './../db-query/query-result-model';
import { Injectable } from "@angular/core";
import { invoke } from '@tauri-apps/api/tauri';

@Injectable({
    providedIn: 'root',
})
export class DbQueryService {
    
    public run_query(query: string, targetDb: Db): Promise<QueryResult> {
        let queryCommand: QueryCommand = {
            query: query,
            target_db: targetDb
        }
        return invoke<QueryResult>('run_query', { 'queryCommand': queryCommand })
    }

}