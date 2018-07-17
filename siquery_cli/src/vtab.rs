
use rusqlite::vtab::{
    eponymous_only_module, sqlite3_vtab, sqlite3_vtab_cursor,
    Context, IndexInfo, VTab, VTabConnection, VTabCursor, Values,
};
use rusqlite::types::Null;
use rusqlite::{version_number, Connection, Result, Error};
use std::os::raw::c_int;

use siquery::query::query_table;

#[repr(C)]
struct DummyTab {
    /// Base class. Must be first
    base: sqlite3_vtab,
    table_name: String,
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
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
            table_name: String::new(),
            columns: Vec::new(),
            rows: Vec::new(),
        };

        // we create the header
        let mut cols: Vec<String> = vec!["name".to_string(),
                                         "number".to_string(),
                                         "alias".to_string(),
                                         "comment".to_string(),];

        let mut schema= None;

        if schema.is_none() {
            let mut sql = String::from("CREATE TABLE x(");
            for (i, col) in cols.iter().enumerate() {
                sql.push('"');
                sql.push_str(col);
                sql.push_str("\" TEXT");
                if i == cols.len() - 1 {
                    sql.push_str(");");
                } else {
                    sql.push_str(", ");
                }
            }
            schema = Some(sql);
        }

        Ok((schema.unwrap().to_owned(), vtab))

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
    /// the column id
    column_id: i64,
    /// columns name
    cols : Vec<String>,
    /// rows
    rows : Vec<Vec<String>>,
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
        self.eot = false;

        // test etc_protocols table
        self.rows = query_table("etc_protocols",
                                    vec!["name".to_string(),
                                         "number".to_string(),
                                         "alias".to_string(),
                                         "comment".to_string(),
                                         ]);

        self.row_id = 0;
        self.next()
    }
    
    fn next(&mut self) -> Result<()> {
        {
            if self.row_id == self.rows.len() as i64 {
                self.eot = true;
                return Ok(());
            }
            else {
                self.eot = false;
                self.cols = self.rows[self.row_id as usize].clone()
            }
        }
        self.row_id += 1;
        Ok(())
    }
    
    fn eof(&self) -> bool {
        self.eot
    }
    fn column(&self, ctx: &mut Context, col: c_int) -> Result<()> {

        if col < 0 || col as usize >= self.cols.len() {
            /*return Err(Error::ModuleError(format!(
                "column index out of bounds: {}",
                col
            )));*/
            return ctx.set_result(&Null);
        }

        // TODO Make sur we have the good format of the table
        if self.cols.is_empty() {
            return ctx.set_result(&Null);
        }
        ctx.set_result(&self.cols[col as usize].to_owned())

    }
    
    fn rowid(&self) -> Result<i64> {
        Ok(self.row_id)
    }
}

fn next_col(cursor: &mut DummyTabCursor) -> i64 {
    if cursor.column_id >= cursor.cols.len() as i64{
        cursor.column_id = 0;
    }
        else {
            cursor.column_id += 1;
        }
    cursor.column_id
}

pub fn sql_query() {
    let module = eponymous_only_module::<DummyTab>();
    let db = Connection::open_in_memory().unwrap();

    db.create_module::<DummyTab>("dummy", &module, None).unwrap();

    let version = version_number();

    if version < 3008012 {
        return;
    }

    let mut s = db.prepare("SELECT name,comment FROM dummy()").unwrap();

    let ids: Result<Vec<String>> = s
        .query_map(&[], |row| row.get::<_, String>(0))
        .unwrap()
        .collect();

    println!("Dummy table :     {:?} ", ids.unwrap());

}
