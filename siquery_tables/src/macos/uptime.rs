#![allow(unused_variables)]

extern crate libc;
extern crate time;

use std::mem;
use std::ptr;

use tables::Uptime;
use macos::SystemReaderInterface;

impl Uptime {
    pub fn get_specific(system_reader: &SystemReaderInterface) -> Vec<Uptime> {
        let mut output : Vec<Uptime> = Vec::new();
        let mut upt = Uptime {
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            total_seconds: 0.0,
        };

        let mut req = [libc::CTL_KERN, libc::KERN_BOOTTIME];
        let mut boottime: libc::timeval = unsafe { mem::zeroed() };
        let mut size: libc::size_t = mem::size_of_val(&boottime) as libc::size_t;

        let ret = unsafe {
            libc::sysctl(
                &mut req[0],
                2,
                &mut boottime as *mut libc::timeval as *mut libc::c_void,
                &mut size,
                ptr::null_mut(),
                0,
            )
        };


        if ret == 0 {
            let mut remaining_time = 0;
            let sec_to_days_converter = 60 * 60 * 24;
            let sec_to_hours_converter = 60 * 60;
            let sec_to_minutes_converter = 60;

            let t = (time::now().to_timespec() - time::Timespec::new(boottime.tv_sec, boottime.tv_usec * 1000)).num_seconds();

            if let Some(get_days) = Some(t / sec_to_days_converter) {
                remaining_time = t - (get_days * sec_to_days_converter);
                upt.days = get_days as u64;
            }

            if let Some(get_hours) = Some(remaining_time / sec_to_hours_converter) {
                remaining_time = remaining_time - (get_hours * sec_to_hours_converter);
                upt.hours = get_hours as u64;
            }

            if let Some(get_minutes) = Some(remaining_time / sec_to_minutes_converter) {
                remaining_time = remaining_time - (get_minutes * sec_to_minutes_converter);
                upt.minutes = get_minutes as u64;
            }

            if let Some(get_seconds) = Some(remaining_time) {
                upt.seconds = get_seconds as u64;
            }

            upt.total_seconds = t as f64;
        }

        output.push(upt);
        output
    }
}


