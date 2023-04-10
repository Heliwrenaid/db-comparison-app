import { QueryResult } from './../db-query/query-result-model';
import { Injectable } from "@angular/core";
import { invoke } from '@tauri-apps/api/tauri';

@Injectable({
    providedIn: 'root',
})
export class DbQueryService {
    
    public run_query(query: string): Promise<QueryResult> {
        return invoke<QueryResult>('run_query', { 'query': query })
    }

}