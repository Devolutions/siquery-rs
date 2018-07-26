#![no_main]
#[macro_use] extern crate cfg_if;
#[macro_use] extern crate libfuzzer_sys;
extern crate siquery;

use siquery::tables::{SystemInfoDataIface,SystemInfoData};

struct StringFuzzer {
    s: String,
    u: u32,
}

cfg_if! {
    if #[cfg(target_os = "linux")] {
        impl SystemInfoDataIface for StringFuzzer {
            fn get_wmi_cpu_info(&self) -> Option<String> {
                Some(String::new())
            }
            fn get_wmi_system_info(&self) -> Option<String> {
                Some(String::new())
            }
            fn hostname(&self) -> Option<String> {
                Some(self.s.to_string())
            }
            fn meminfo(&self) -> Option<String> {
                Some(self.s.to_string())
            }
            fn cpuinfo(&self) -> Option<String> {
                Some(self.s.to_string())
            }
            fn cpu_count(&self) -> u32 {
                self.u
            }
        }
    }
}

impl StringFuzzer {
    fn new_str(str_data: &[u8]) -> StringFuzzer {
        let mut reader = StringFuzzer {s: String::new(), u: 0};
        if let Ok(s) = std::str::from_utf8(str_data) {
            reader.s = s.to_string();
        }
        reader
    }
}

fuzz_target!(|str_data: &[u8]| {
    SystemInfoData::get_specific_ex(&(StringFuzzer::new_str(str_data)));
});

