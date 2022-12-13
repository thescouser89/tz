use chrono::Timelike;
use chrono::{TimeZone, Utc};
use chrono_tz::Asia::Calcutta;
use chrono_tz::Asia::Shanghai;
use chrono_tz::Canada::Eastern;
use chrono_tz::Europe::London;
use chrono_tz::Europe::Prague;
use chrono_tz::Indian::Mauritius;

use std::env;

fn main() {
    let time_format = "%H:%M";

    let args: Vec<String> = env::args().collect();
    let mut utc_time = Utc::now();

    // only time specified
    if args.len() >= 2 {
        let time_specified_utc = &args[1];
        let tokens: Vec<&str> = time_specified_utc.split(":").collect();
        let hour = tokens[0].parse().unwrap();
        let minute = tokens[1].parse().unwrap();

        // time and date specific
        if args.len() == 3 {
            let date_specified = &args[2];
            let tokens: Vec<&str> = date_specified.split("-").collect();
            let year = tokens[0].parse().unwrap();
            let month = tokens[1].parse().unwrap();
            let day = tokens[2].parse().unwrap();
            utc_time = Utc
                .with_ymd_and_hms(year, month, day, hour, minute, 0)
                .unwrap();
        } else {
            utc_time = utc_time
                .with_hour(hour)
                .unwrap()
                .with_minute(minute)
                .unwrap();
        }
    }

    println!(
        "    {} Eastern time",
        utc_time.with_timezone(&Eastern).format(time_format)
    );
    println!("    {} UTC", utc_time.format(time_format));
    println!(
        "    {} London",
        utc_time.with_timezone(&London).format(time_format)
    );
    println!(
        "    {} Brno",
        utc_time.with_timezone(&Prague).format(time_format)
    );
    println!(
        "    {} Mauritius",
        utc_time.with_timezone(&Mauritius).format(time_format)
    );
    println!(
        "    {} IST",
        utc_time.with_timezone(&Calcutta).format(time_format)
    );
    println!(
        "    {} Beijing",
        utc_time.with_timezone(&Shanghai).format(time_format)
    );
}
