
use rusqlite::vtab::{
    eponymous_only_module, sqlite3_vtab, sqlite3_vtab_cursor,
    Context, IndexInfo, VTab, VTabConnection, VTabCursor, Values,
};
use rusqlite::types::Null;
use rusqlite::{version_number, Connection, Result, Error};
use std::os::raw::c_int;

#[repr(C)]
struct DummyTab {
    /// Base class. Must be first
    base: sqlite3_vtab,
}

impl VTab for DummyTab {
    type Aux = ();
    type Cursor = DummyTabCursor;

    fn connect(
        _: &mut VTabConnection,
        _aux: Option<&()>,
        _args: &[&[u8]],
    ) -> Result<(String, DummyTab)> {
        let vtab = DummyTab {
            base: sqlite3_vtab::default(),
        };
        Ok(("CREATE TABLE x(value)".to_owned(), vtab))
    }

    fn best_index(&self, info: &mut IndexInfo) -> Result<()> {
        info.set_estimated_cost(1.);
        Ok(())
    }

    fn open(&self) -> Result<DummyTabCursor> {
        Ok(DummyTabCursor::default())
    }
}

#[derive(Default)]
#[repr(C)]
struct DummyTabCursor {
    /// Base class. Must be first
    base: sqlite3_vtab_cursor,
    /// The rowid
    row_id: i64,
    /// columns
    cols : Vec<String>,
    /// the length of the table
    table_length: u32,
    /// the end of the table
    eot : bool
}

impl VTabCursor for DummyTabCursor {
    type Table = DummyTab;

    fn filter(
        &mut self,
        _idx_num: c_int,
        _idx_str: Option<&str>,
        _args: &Values,
    ) -> Result<()> {
        self.row_id = 0;
        self.eot = false;
        self.cols = vec!["test1".to_string(),
                         "test2".to_string(),
                         "test3".to_string(),
                         "test4".to_string(),
                         "test5".to_string(),
                         "test6".to_string(),
                         "test7".to_string(),
                         "test8".to_string()];
        self.next()
    }
    
    fn next(&mut self) -> Result<()> {
        {
            if self.row_id == self.cols.len() as i64 {
                self.eot = true;
                return Ok(());
            }
        }

        self.row_id += 1;
        Ok(())
    }
    
    fn eof(&self) -> bool {
        self.eot
    }

    fn column(&self, ctx: &mut Context, col: c_int) -> Result<()> {

        // TODO Make sur we have the good format of the table
        if self.cols.is_empty() {
            return ctx.set_result(&Null);
        }
        // TODO Affinity
        ctx.set_result(&self.cols[(self.row_id - 1)  as usize].to_owned())
    }
    
    fn rowid(&self) -> Result<i64> {
        Ok(self.row_id)
    }
}

pub fn sql_query() {
    let module = eponymous_only_module::<DummyTab>();
    let db = Connection::open_in_memory().unwrap();

    db.create_module::<DummyTab>("dummy", &module, None).unwrap();

    let version = version_number();

    if version < 3008012 {
        return;
    }

    let mut s = db.prepare("SELECT * FROM dummy()").unwrap();

    let ids: Result<Vec<String>> = s
        .query_map(&[], |row| row.get::<_, String>(0))
        .unwrap()
        .collect();

    println!("Dummy table :     {:?} ", ids);

}
