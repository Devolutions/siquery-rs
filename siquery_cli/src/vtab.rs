
use libsqlite3_sys as ffi;
use rusqlite::vtab::{Context, IndexInfo, Module, VTab, VTabConnection, VTabCursor, Values};
use rusqlite::{error_from_sqlite_code, Connection, Error, Result};
use std::os::raw::{c_char, c_int, c_void};

eponymous_module!(
    QUERY_MODULE,
    QueryModule,
    QueryTab,
    (),
    QueryTabCursor,
    None,
    query_connect,
    query_best_index,
    query_disconnect,
    None,
    query_open,
    query_close,
    query_filter,
    query_next,
    query_eof,
    query_column,
    query_rowid
);

#[repr(C)]
struct QueryModule(&'static ffi::sqlite3_module);

impl Module for QueryModule {
    type Aux = ();
    type Table = QueryTab;

    fn as_ptr(&self) -> *const ffi::sqlite3_module {
        self.0
    }

    fn connect(
        _: &mut VTabConnection,
        _aux: Option<&()>,
        _args: &[&[u8]],
    ) -> Result<(String, QueryTab)> {
        let vtab = QueryTab {
            base: ffi::sqlite3_vtab::default(),
        };
        Ok(("CREATE TABLE x(value)".to_owned(), vtab))
    }
}

#[repr(C)]
struct QueryTab {
    /// Base class. Must be first
    base: ffi::sqlite3_vtab,
}

impl VTab for QueryTab {
    type Cursor = QueryTabCursor;

    fn best_index(&self, info: &mut IndexInfo) -> Result<()> {
        info.set_estimated_cost(1.);
        Ok(())
    }

    fn open(&self) -> Result<QueryTabCursor> {
        Ok(QueryTabCursor::new())
    }
}

#[derive(Default)]
#[repr(C)]
struct QueryTabCursor {
    /// Base class. Must be first
    base: ffi::sqlite3_vtab_cursor,
    /// The rowid
    row_id: i64,
}

impl QueryTabCursor {
    fn new() -> QueryTabCursor {
        QueryTabCursor {
            base: ffi::sqlite3_vtab_cursor::default(),
            row_id: 0
        }
    }
}

impl VTabCursor for QueryTabCursor {
    type Table = QueryTab;

    fn vtab(&self) -> &QueryTab {
        unsafe { &*(self.base.pVtab as *const QueryTab) }
    }

    fn filter(&mut self, _idx_num: c_int, _idx_str: Option<&str>, _args: &Values) -> Result<()> {
        self.row_id = 1;
        Ok(())
    }

    fn next(&mut self) -> Result<()> {
        self.row_id += 1; // move to next row
        Ok(())
    }
    
    fn eof(&self) -> bool {
        self.row_id > 1 // return true if end of table is reached
    }
    
    fn column(&self, ctx: &mut Context, _i: c_int) -> Result<()> {
        ctx.set_result(&self.row_id)
    }
    
    fn rowid(&self) -> Result<i64> {
        Ok(self.row_id) // return current row id
    }
}

pub fn sql_query() {
    let db = Connection::open_in_memory().unwrap();

    let module = QueryModule(&QUERY_MODULE);

    db.create_module("query", module, None).unwrap();

    let version = unsafe { ffi::sqlite3_libversion_number() };

    if version < 3008012 {
        return;
    }

    let mut s = db.prepare("SELECT * FROM query()").unwrap();

    let query = s.query_row(&[], |row| row.get::<_, i32>(0)).unwrap();
    println!("my query is {}", query);
    assert_eq!(1, query);
}
