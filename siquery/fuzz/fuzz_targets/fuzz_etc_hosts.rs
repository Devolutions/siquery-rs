#![no_main]
#[macro_use] extern crate cfg_if;
#[macro_use] extern crate libfuzzer_sys;

use siquery::tables::{EtcHostsIface,EtcHosts};

struct Fuzzer {
    s: String,
}   //TODO: define in tables and import instead of creating new instances of same struct every time.

impl Fuzzer {
    fn new_str(str_data: &[u8]) -> Fuzzer {
        let mut reader = Fuzzer {s: String::new()};
        if let Ok(s) = std::str::from_utf8(str_data) {
            reader.s = s.to_string();
        }
        reader
    }
}

cfg_if! {
    if #[cfg(target_os = "linux")] {
        impl EtcHostsIface for Fuzzer {
            fn get_hosts_file(&self) -> Option<String> {
                Some(self.s.to_string())
            }
        }
    }   // TODO impl for other platforms
}

fuzz_target!(|str_data: &[u8]| {
    EtcHosts::get_specific_ex(&(Fuzzer::new_str(str_data)));
});
