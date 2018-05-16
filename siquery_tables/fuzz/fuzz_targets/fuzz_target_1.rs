#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate siquery;

use siquery::SystemInfo;
use siquery::sys::SystemReaderInterface;

struct FuzzSystemReader {
    s: String,
}

impl FuzzSystemReader {
    fn new(data: &[u8]) -> FuzzSystemReader {
        
        let mut reader = FuzzSystemReader{s: String::from("")};
        
        if let Ok(s) = std::str::from_utf8(data) {
            reader.s = s.to_string();
        }
        
        reader
    }
}

impl SystemReaderInterface for FuzzSystemReader {
    fn hostname(&self) -> Option<String> {
        Some(self.s.to_string())
    }

    fn cpuinfo(&self) -> Option<String> {
        Some(self.s.to_string())
    }

    fn cpu_count(&self) -> u32 {
        4
    }

    fn os_release(&self) -> Option<String> {
        Some(self.s.to_string())
    }

    fn os_platform(&self) -> Option<String> {
        Some(self.s.to_string())
    }

    fn meminfo(&self) -> Option<String> {
        Some(self.s.to_string())
    }

    fn get_hosts_file(&self) -> Option<String> {
        Some(self.s.to_string())
    }

    fn get_protocols_file(&self) -> Option<String> {
        Some(self.s.to_string())
    }

    fn get_services_file(&self) -> Option<String> {
        Some(self.s.to_string())
    }
}

fuzz_target!(|data: &[u8]| {
    let system_info = SystemInfo::new(Box::new(FuzzSystemReader::new(data)));
});
