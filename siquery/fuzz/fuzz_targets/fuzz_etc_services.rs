#![no_main]
#[macro_use] extern crate cfg_if;
#[macro_use] extern crate libfuzzer_sys;

use siquery::tables::{EtcServicesIface,EtcServices};

struct Fuzzer {
    s: String,
}

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
        impl EtcServicesIface for Fuzzer {
            fn get_services_file(&self) -> Option<String> {
                Some(self.s.to_string())
            }
        }
    }
}

fuzz_target!(|str_data: &[u8]| {
    EtcServices::get_specific_ex(&(Fuzzer::new_str(str_data)));
});
