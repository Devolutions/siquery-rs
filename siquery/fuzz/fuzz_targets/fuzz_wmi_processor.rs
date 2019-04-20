#![no_main]
#[macro_use] extern crate cfg_if;
#[macro_use] extern crate libfuzzer_sys;

use siquery::tables::{WmiProcessorIface,WmiProcessor};

struct StringFuzzer {
    s: String,
}

impl StringFuzzer {
    fn new_str(str_data: &[u8]) -> StringFuzzer {
        let mut reader = StringFuzzer {s: String::new()};
        if let Ok(s) = std::str::from_utf8(str_data) {
            reader.s = s.to_string();
        }
        reader
    }
}

cfg_if! {
    if #[cfg(target_os = "linux")] {
        impl WmiProcessorIface for StringFuzzer {
            fn get_wmi_processor_info(&self) -> Option<String> {
                Some(self.s.to_string())
            }
        }
    }
}

fuzz_target!(|str_data: &[u8]| {
    WmiProcessor::get_specific_ex(&StringFuzzer::new_str(str_data));
});
