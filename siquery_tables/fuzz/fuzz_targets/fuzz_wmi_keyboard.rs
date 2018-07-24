#![no_main]
#[macro_use] extern crate cfg_if;
#[macro_use] extern crate libfuzzer_sys;
extern crate siquery;

use siquery::tables::{WmiKeyboardIface,WmiKeyboard};

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
        impl WmiKeyboardIface for StringFuzzer {
            fn get_wmi_keyboard_info(&self) -> Option<String> {
                Some(self.s.to_string())
            }
        }
    }
}

fuzz_target!(|str_data: &[u8]| {
    WmiKeyboard::get_specific_ex(&StringFuzzer::new_str(str_data));
});
