use rusqlite::vtab::{
    sqlite3_vtab, sqlite3_vtab_cursor,
    Context, IndexInfo, VTab, VTabConnection, VTabCursor, Values,simple_module,
    dequote, Module,
};

use rusqlite::types::Null;
use rusqlite::{Connection, Result, Error};
use std::os::raw::c_int;
use std::str;

use std::time::{Duration, SystemTime};
use query::{query_table, query_header};

pub fn load_module(conn: &Connection) -> Result<()> {
    let aux: Option<()> = None;
    conn.create_module("siquery", &SIQUERY_MODULE, aux)
}

lazy_static! {
    static ref SIQUERY_MODULE: Module<SiqueryTab> = simple_module::<SiqueryTab>();
}

#[repr(C)]
struct SiqueryTab {
    /// Base class. Must be first
    base: sqlite3_vtab,
    table_name: String,
    table: Vec<Vec<String>>,
    columns: Vec<String>,
    header: Vec<String>,
    table_in_memory: bool,
}

impl SiqueryTab {
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
            if value.len() > 0 {
                v.push(value.to_string());
            }
        }
        v
    }
}

impl VTab for SiqueryTab {
    type Aux = ();
    type Cursor = SiqueryTabCursor;

    fn connect(
        _: &mut VTabConnection,
        _aux: Option<&()>,
        _args: &[&[u8]],
    ) -> Result<(String, SiqueryTab)> {

        if _args.len() < 4 {
            return Err(Error::ModuleError("no table name specified".to_owned()));
        }

        let mut vtab = SiqueryTab {
            base: sqlite3_vtab::default(),
            table_name: String::new(),
            table: Vec::new(),
            columns: Vec::new(),
            header: Vec::new(),
            table_in_memory: false,
        };

        let args= &_args[3..];

        for c_slice in args {
            let (param, value) = try!(SiqueryTab::parameter(c_slice));
            match param {
                "table_name" => {
                    vtab.table_name = value.to_string();
                }
                "columns" => {
                    if value.len() > 1 {
                        vtab.columns = SiqueryTab::get_from_args(value);
                    } else{
                        vtab.columns = Vec::new();
                    }
                }
                _ => {
                    return Err(Error::ModuleError(format!(
                        "unrecognized parameter '{}'",
                        param
                    )));
                }
            }
        }

        // create the header & register table in memory
        vtab.header = query_header(vtab.table_name.as_str(), vtab.columns.clone());
        vtab.table = query_table(vtab.table_name.as_str(), vtab.header.clone());

        let mut schema = None;
        if schema.is_none() {
            let mut sql = String::from("CREATE TABLE x(");
            for (i, col) in vtab.header.iter().enumerate() {
                sql.push('"');
                sql.push_str(col);
                sql.push_str("\" TEXT");
                if i == vtab.header.len() - 1 {
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
        info.set_estimated_cost(0.);
        Ok(())
    }

    fn open(&self) -> Result<SiqueryTabCursor> { Ok(SiqueryTabCursor::default())
    }
}

#[derive(Default)]
#[repr(C)]
struct SiqueryTabCursor {
    /// Base class. Must be first
    base: sqlite3_vtab_cursor,
    /// The rowid
    row_id: i64,
    /// columns name
    cols : Vec<String>,
    /// rows
    rows : Vec<Vec<String>>,
    /// the end of the table
    eot : bool
}

impl VTabCursor for SiqueryTabCursor {
    type Table = SiqueryTab;

    fn filter(
        &mut self,
        _idx_num: c_int,
        _idx_str: Option<&str>,
        _args: &Values,
    ) -> Result<()> {

        let siquery_table = unsafe {&*(self.base.pVtab as * const SiqueryTab)};

        self.rows = siquery_table.table.to_owned();
        self.row_id = 0;
        self.next()
    }
    fn next(&mut self) -> Result<()> {
        {
            if self.row_id == self.rows.len() as i64 {
                self.eot = true;
                return Ok(());
            } else {
                self.cols = self.rows[self.row_id as usize].clone();
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
            return Err(Error::ModuleError(format!(
                "column index out of bounds: {}",
                col
            )));
        }

        if self.cols.is_empty() {
            return ctx.set_result(&Null);
        }
        ctx.set_result(&self.cols[col as usize].to_owned())
    }
    fn rowid(&self) -> Result<i64> {
        Ok(self.row_id)
    }
}
