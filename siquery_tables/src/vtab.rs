use rusqlite::vtab::{
    sqlite3_vtab, sqlite3_vtab_cursor, Context, IndexInfo,
    VTab, VTabConnection, VTabCursor, Values, read_only_module,
    dequote, Module, CreateVTab};

use rusqlite::types::*;
use rusqlite::{Connection, Result, Error};
use std::os::raw::c_int;
use std::str;

use query::{query_table, get_schema};

pub fn load_module(conn: &Connection) -> Result<()> {
    let aux: Option<()> = None;
    conn.create_module("siquery", &SIQUERY_MODULE, aux)
}

lazy_static! {
    static ref SIQUERY_MODULE: Module<SiqueryTab> = read_only_module::<SiqueryTab>(1);
}

#[repr(C)]
struct SiqueryTab {
    /// Base class. Must be first
    base: sqlite3_vtab,
    table_name: String,
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
        };
        let schema;
        let args= &_args[3..];

        for c_slice in args {
            let (param, value) = try!(SiqueryTab::parameter(c_slice));
            match param {
                "table_name" => {
                    vtab.table_name = value.to_string();
                }
                _ => {
                    return Err(Error::ModuleError(format!(
                        "unrecognized parameter '{}'",
                        param
                    )));
                }
            }
        }

        schema = get_schema(vtab.table_name.as_str());
        Ok((schema.unwrap().to_owned(), vtab))
    }

    fn best_index(&self, info: &mut IndexInfo) -> Result<()> {
        info.set_estimated_cost(1_000_000.);
        Ok(())
    }

    fn open(&self) -> Result<SiqueryTabCursor> {Ok(SiqueryTabCursor::default())}
}

impl CreateVTab for SiqueryTab {}

#[derive(Default)]
#[repr(C)]
struct SiqueryTabCursor {
    /// Base class. Must be first
    base: sqlite3_vtab_cursor,
    /// table is in memory
    table_in_memory: bool,
    /// The rowid
    row_id: i64,
    /// columns name
    cols : Vec<Value>,
    /// rows
    rows : Vec<Vec<Value>>,
    /// the end of the table
    eot : bool,
}

impl VTabCursor for SiqueryTabCursor {

    fn filter(
        &mut self,
        _idx_num: c_int,
        _idx_str: Option<&str>,
        _args: &Values,
    ) -> Result<()> {
        let siquery_table = unsafe {&*(self.base.pVtab as * const SiqueryTab)};
        // register table in memory
        if !self.table_in_memory {
            self.rows = query_table(siquery_table.table_name.as_str(), vec![]);
            self.table_in_memory = true;
        }
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
                self.row_id += 1;
            }
        }
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
