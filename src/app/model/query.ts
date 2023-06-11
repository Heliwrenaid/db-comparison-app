export interface QueryResult<T> {
    result: T
    duration: Duration
}

export interface Duration {
    nanos: number,
    secs: number
}

export interface QueryCommand {
    target_db: Db,
    query: string
}

export enum Db {
    SurrealDb = "SurrealDb",
    Redis = "Redis",
    Skytable = "Skytable"
}
