use chrono::Timelike;
use chrono::DateTime;
use chrono::{TimeZone, Utc};
use chrono_tz::Asia::Calcutta;
use chrono_tz::Asia::Jerusalem;
use chrono_tz::Asia::Shanghai;
use chrono_tz::Canada::Eastern;
use chrono_tz::Europe::London;
use chrono_tz::Europe::Prague;
use chrono_tz::Indian::Mauritius;

use std::env;
use std::fmt::Display;

fn pretty_print<Tz: chrono::TimeZone>(date: &DateTime<Tz>, timezone: &str)
    where Tz::Offset: Display {
    let time_format = "%H:%M";
    println!("    {} {}", date.format(time_format), timezone);
}

fn read_hour_minute(hhmm: &str) -> (u32, u32) {

    let tokens: Vec<&str> = hhmm.split(":").collect();
    let hour = tokens[0].parse().unwrap();
    let minute = tokens[1].parse().unwrap();

    (hour, minute)
}

fn read_yy_mm_dd(yymmdd: & str) -> (i32, u32, u32) {
    let tokens: Vec<&str> = yymmdd.split("-").collect();
    let year = tokens[0].parse().unwrap();
    let month = tokens[1].parse().unwrap();
    let day = tokens[2].parse().unwrap();
    (year, month, day)
}

fn main() {

    let args: Vec<String> = env::args().collect();

    // only time specified
    let utc_time = match args.len() {
        // no specific time or date
        1 => Utc::now(), 
        // only time specified: HH:MM
        2 => {
            let (hour, minute) = read_hour_minute(&args[1]);
            Utc::now().with_hour(hour).unwrap().with_minute(minute).unwrap()
        },
        // HH:MM and yy-mm-dd specified
        3 => {
            let (hour, minute) = read_hour_minute(&args[1]);
            let (year, month, day) = read_yy_mm_dd(&args[2]);
            Utc.with_ymd_and_hms(year, month, day, hour, minute, 0).unwrap()
        },
        _ => panic!("Too many arguments"),

    };

    pretty_print(&utc_time.with_timezone(&Eastern), "Eastern time");
    pretty_print(&utc_time, "UTC");
    pretty_print(&utc_time.with_timezone(&London), "London");
    pretty_print(&utc_time.with_timezone(&Prague), "Brno");
    pretty_print(&utc_time.with_timezone(&Jerusalem), "Israel");
    pretty_print(&utc_time.with_timezone(&Mauritius), "Mauritius");
    pretty_print(&utc_time.with_timezone(&Calcutta), "Pune");
    pretty_print(&utc_time.with_timezone(&Shanghai), "Beijing");
}
