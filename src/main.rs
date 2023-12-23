use chrono::DateTime;
use chrono::Timelike;
use chrono::{Local, TimeZone, Utc};
use chrono_tz::Asia::Calcutta;
use chrono_tz::Asia::Jerusalem;
use chrono_tz::Asia::Shanghai;
use chrono_tz::Canada::Eastern;
use chrono_tz::Europe::London;
use chrono_tz::Europe::Prague;
use chrono_tz::Indian::Mauritius;
use clap::Parser;

use colored::*;

use std::fmt::Display;

/// Simple program to show timezone
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Time in UTC in format HH:MM
    time: Option<String>,

    /// Date in format yy-mm-dd
    date: Option<String>,

    /// Consider date in UTC instead of local time
    #[clap(long, action)]
    utc: bool,
}

fn pretty_print<Tz: chrono::TimeZone>(date: &DateTime<Tz>, timezone: &str)
where
    Tz::Offset: Display,
{
    let time_format = "%H:%M";
    println!(
        "    {} {}",
        date.format(time_format).to_string().white(),
        timezone.bright_blue()
    );
}

fn read_hour_minute(hhmm: &str) -> (u32, u32) {
    let error_msg = "Cannot parse the time. Needs to be in format HH:MM";
    let tokens: Vec<&str> = hhmm.split(":").collect();

    if tokens.len() != 2 {
        panic!("{}", error_msg);
    }
    let hour = tokens[0].parse().expect(error_msg);
    let minute = tokens[1].parse().expect(error_msg);

    (hour, minute)
}

fn read_yy_mm_dd(yymmdd: &str) -> (i32, u32, u32) {
    let error_msg = "Cannot parse the date. Needs to be in format yyyy-mm-dd";
    let tokens: Vec<&str> = yymmdd.split("-").collect();

    if tokens.len() != 3 {
        panic!("{}", error_msg);
    }
    let year = tokens[0].parse().expect(error_msg);
    let month = tokens[1].parse().expect(error_msg);
    let day = tokens[2].parse().expect(error_msg);
    (year, month, day)
}

fn main() {
    let args = Args::parse();

    // only time specified
    let utc_time = match args.time {
        // no specific time or date
        None => Utc::now(),
        // only time specified: HH:MM
        Some(time) => {
            match args.date {
                None => {
                    let (hour, minute) = read_hour_minute(&time);
                    if args.utc {
                        Utc::now()
                            .with_hour(hour)
                            .unwrap()
                            .with_minute(minute)
                            .unwrap()
                    } else {
                        Local::now()
                            .with_hour(hour)
                            .unwrap()
                            .with_minute(minute)
                            .unwrap()
                            .with_timezone(&Utc)
                    }
                }
                // HH:MM and yy-mm-dd specified
                Some(date) => {
                    let (hour, minute) = read_hour_minute(&time);
                    let (year, month, day) = read_yy_mm_dd(&date);
                    if args.utc {
                        Utc.with_ymd_and_hms(year, month, day, hour, minute, 0)
                            .unwrap()
                    } else {
                        Local
                            .with_ymd_and_hms(year, month, day, hour, minute, 0)
                            .unwrap()
                            .with_timezone(&Utc)
                    }
                }
            }
        }
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
