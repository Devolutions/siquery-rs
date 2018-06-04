extern crate kernel32;

use tables::Uptime;

impl Uptime {

    pub fn get_uptime() ->Result <Uptime, String> {
        let mut upt = Uptime{
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            total_seconds: 0.0,
        };

        let t: u64 = unsafe { kernel32::GetTickCount64() };
        let mut remaining_time;
        let milli_to_days_converter = 1000 * 60 * 60 * 24;
        let milli_to_hours_converter = 1000 * 60 * 60;
        let milli_to_minutes_converter = 1000 * 60;
        let milli_to_seconds_converter =  1000;

        let get_days = t / milli_to_days_converter;
        remaining_time = t - (get_days *  milli_to_days_converter);

        let get_hours = remaining_time / milli_to_hours_converter;
        remaining_time = remaining_time - (get_hours *  milli_to_hours_converter);

        let get_minutes = remaining_time / milli_to_minutes_converter;
        remaining_time = remaining_time - (get_minutes *  milli_to_minutes_converter);

        let get_seconds = remaining_time / milli_to_seconds_converter;

        upt.days = get_days;
        upt.hours = get_hours ;
        upt.minutes = get_minutes;
        upt.seconds = get_seconds;
        upt.total_seconds = t as f64 / 1000.0;

        Ok(upt)
    }
}



