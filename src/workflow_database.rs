use rusqlite::{Connection, Params, params, Row, MappedRows, Result};
use crate::workflow::AlfredWorkflow;


impl AlfredWorkflow {
    pub fn open(path: &str) -> Connection {
        Connection::open(path).unwrap()
    }

    pub fn query<P: Params, F, T>(db: &Connection, sql: &str, param: P, map_fun: F) -> Vec<T>
        where
            P: Params,
            F: FnMut(&Row<'_>) -> Result<T>,
    {
        let mut stmt = db.prepare(sql).unwrap();
        let rows = stmt.query_map(param, map_fun).unwrap();
        let mut rts = Vec::new();
        for row in rows {
            rts.push(row.unwrap())
        }
        rts
    }

    pub fn close(db: Connection) {
        db.close().unwrap();
    }
}