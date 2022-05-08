use rusqlite::{Connection, Params, params, Row, MappedRows, Result};
use crate::workflow::AlfredWorkflow;


impl AlfredWorkflow {
    pub fn open(path: &str) -> Connection {
        Connection::open(path).unwrap()
    }

    pub fn query<P: Params, F, T>(db: &Connection, sql: &str, param: P, map_fun: F) -> MappedRows<F>
        where
            P: Params,
            F: FnMut(&Row<'_>) -> T,
    {
        let stmt = db.prepare(sql).unwrap();
        stmt.query_map(param, map_fun).unwrap()
    }

    pub fn close(db: Connection) {
        db.close()
    }
}