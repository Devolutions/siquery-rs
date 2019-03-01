use std::io;
use mustache::MapBuilder;
use rusqlite::Rows;
use rusqlite::types::Type;
use rusqlite::types::Value;

use tables::WmiOsVersion;

pub fn print_html(columns: Vec<String>, values: &mut Rows) {
    let template = mustache::compile_str("{{build_number}}: {{value}}").unwrap();

    let os_version = WmiOsVersion::get_specific();
    //let serialized = serde_json::to_string(&os_version).unwrap();
    //println!("{}",serialized);

    //for entry in os_version {
        //let names = stringify!(entry);
        //for entry_1 in names.iter() {
        //    print!("{}",entry_1);
        //}
        //template.render(&mut io::stdout(), &names).unwrap();
        //println!("");
    //}
    //let data = MapBuilder::new();
    loop {
        if let Some(v) = values.next(){
            if let Some (res) = v.ok() {
                for i in 0..res.column_count() {
                    let val = Value::data_type(&res.get(i));
                    match val {
                        Type::Real | Type::Integer => {
                            let some = MapBuilder::new().insert(columns[i].clone(), &res.get::<usize,i64>(i).to_string())
                                .expect("Failed to encode name");
                            template.render_data(&mut io::stdout(), &some.build()).unwrap();
                        },
                        Type::Text => {
                            let some = MapBuilder::new().insert(columns[i].clone(), &res.get::<usize,String>(i))
                                .expect("Failed to encode name");
                            template.render_data(&mut io::stdout(), &some.build()).unwrap();
                        },
                        _ => {
                            // Do nothing.
                        }
                    }
                }
                // write row values
            }
        } else {
            break
        }
    }
    //template.render_data(&mut io::stdout(), &data.build()).unwrap();
    println!("{:?}",template);
}


macro_rules! stringify {
    (struct $name:ident { $($fname:ident : $ftype:ty),* }) => {
        struct $name {
            $($fname : $ftype),*
        }

        impl $name {
            fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                NAMES
            }
        }
    }
}