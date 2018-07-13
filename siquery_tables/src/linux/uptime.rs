extern crate libc;

use std::mem;

use tables::Uptime;
use linux::SystemReaderInterface;

impl Uptime {

    pub fn get_specific(_system_reader: &SystemReaderInterface) ->  Vec<Uptime> {
        let mut output : Vec<Uptime> = Vec::new();
        let mut upt = Uptime {
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            total_seconds: 0.0,
        };

        let mut info: libc::sysinfo = unsafe { mem::zeroed() };
        let mut _t = unsafe { libc::sysinfo(&mut info) };

        _t = info.uptime as i32;

        let mut remaining_time = 0;
        let sec_to_days_converter = 60 * 60 * 24;
        let sec_to_hours_converter = 60 * 60;
        let sec_to_minutes_converter = 60;

        if let Some(get_days) = Some(_t / sec_to_days_converter){
            remaining_time = _t - (get_days * sec_to_days_converter);
            upt.days = get_days as u64;
        }

        if let Some(get_hours) = Some(remaining_time / sec_to_hours_converter){
            remaining_time = remaining_time - (get_hours * sec_to_hours_converter);
            upt.hours = get_hours as u64;
        }

        if let Some(get_minutes) = Some(remaining_time / sec_to_minutes_converter){
            remaining_time = remaining_time - (get_minutes * sec_to_minutes_converter);
            upt.minutes = get_minutes as u64;
        }

        if let Some(get_seconds) = Some(remaining_time){
            upt.seconds = get_seconds as u64;
        }

        upt.total_seconds = _t as f64;

        output.push(upt);
        output
    }
}
