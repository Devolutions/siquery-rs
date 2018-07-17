use rusqlite::vtab::{
    sqlite3_vtab, sqlite3_vtab_cursor,
    Context, IndexInfo, VTab, VTabConnection, VTabCursor, Values,simple_module,
    dequote, escape_double_quote, parse_boolean, Module,
};

use rusqlite::types::Null;
use rusqlite::{version_number, Connection, Result, Error};
use std::os::raw::c_int;
use std::str;

use siquery::query::query_table;

pub fn load_module(conn: &Connection) -> Result<()> {
    let aux: Option<()> = None;
    conn.create_module("dummy", &DUMMY_MODULE, aux)
}

lazy_static! {
    static ref DUMMY_MODULE: Module<DummyTab> = simple_module::<DummyTab>();
}

#[repr(C)]
struct DummyTab {
    /// Base class. Must be first
    base: sqlite3_vtab,
    table_name: String,
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl DummyTab {
    fn parameter(c_slice: &[u8]) -> Result<(&str, &str)> {
        let arg = try!(str::from_utf8(c_slice)).trim();
        let mut split = arg.split('=');
        if let Some(key) = split.next() {
            if let Some(value) = split.next() {
                let param = key.trim();
                let value = dequote(value);
                return Ok((param, value));
            }
        }
        Err(Error::ModuleError(format!("illegal argument: '{}'", arg)))
    }

    fn get_from_args(args: &str)-> Vec<String>{
        let mut v : Vec<String> = Vec::new();
        let split : Vec<_>= args.split(';').collect();
        for value in split {
            v.push(value.to_string());
        }
        v
    }
}

impl VTab for DummyTab {
    type Aux = ();
    type Cursor = DummyTabCursor;

    fn connect(
        _: &mut VTabConnection,
        _aux: Option<&()>,
        _args: &[&[u8]],
    ) -> Result<(String, DummyTab)> {

        if _args.len() < 4 {
            return Err(Error::ModuleError("no table name specified".to_owned()));
        }

        let mut vtab = DummyTab {
            base: sqlite3_vtab::default(),
            table_name: String::new(),
            columns: Vec::new(),
            rows: Vec::new(),
        };

        let args= &_args[3..];

        for c_slice in args {
            let (param, value) = try!(DummyTab::parameter(c_slice));
            match param {
                "table_name" => {
                    vtab.table_name = value.to_string();
                }
                "columns" => {
                    vtab.columns = DummyTab::get_from_args(value);
                }
                _ => {
                    return Err(Error::ModuleError(format!(
                        "unrecognized parameter '{}'",
                        param
                    )));
                }
            }
        }

        // create the table in memory
        vtab.rows = query_table(vtab.table_name.as_str(), vtab.columns.clone());

        let mut schema= None;

        if schema.is_none() {
            let mut sql = String::from("CREATE TABLE x(");
            for (i, col) in vtab.rows[0].iter().enumerate() {
                sql.push('"');
                sql.push_str(col);
                sql.push_str("\" TEXT");
                if i == vtab.rows[0].len() - 1 {
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

        // test etc_protocols table
        let mut DummyTable = unsafe {&*(self.base.pVtab as * const DummyTab)};

        self.rows = DummyTable.rows.clone();

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

pub fn sql_query() {
    let db = Connection::open_in_memory().unwrap();

    load_module(&db).unwrap();

    let command =  "CREATE VIRTUAL TABLE process_memory_map USING dummy(table_name=process_memory_map)";

    db.execute_batch(&command).unwrap();

    let version = version_number();

    if version < 3008012 {
        return;
    }

    let mut s = db.prepare("SELECT count(pid) FROM process_memory_map").unwrap();

    let ids: Result<Vec<i32>> = s
        .query_map(&[], |row| row.get::<_, i32>(0))
        .unwrap()
        .collect();

    println!("pid counter :     {:?} ", ids.unwrap());

    let command2 =  "CREATE VIRTUAL TABLE etc_protocols USING dummy(table_name=etc_protocols)";
    db.execute_batch(&command2).unwrap();

    let mut s2 = db.prepare("SELECT name FROM etc_protocols").unwrap();

    let ids: Result<Vec<String>> = s2
        .query_map(&[], |row| row.get::<_, String>(0))
        .unwrap()
        .collect();

    println!("protocols name :     {:?} ", ids.unwrap());
}
