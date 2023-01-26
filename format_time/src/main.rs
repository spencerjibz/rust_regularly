#![allow(dead_code, unused)]
use std::time::Instant;
fn main() {
    let now = Instant::now();
    format_duration(132030240 * 999999999);
    println!(" {:?}", now.elapsed())
}

#[derive(Debug)]
pub enum Interval {
    Min = 60,
    Hour = 60 * 60,
    Day = 60 * 60 * 24,
    Year = 60 * 60 * 24 * 365,
}
#[derive(Debug, Default)]
struct Calculator {
    minutes: u64,
    hours: u64,
    days: u64,
    years: u64,
    seconds: u64,
}

impl Calculator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_readable(&self) -> String {
        let mut res = Vec::with_capacity(4);

        if self.seconds > 0 {
            let words = if self.seconds > 1 {
                "seconds"
            } else {
                "second"
            };
            res.push(format!("{} {}", self.seconds, words));
        }

        if self.minutes > 0 {
            let words = if self.minutes > 1 {
                "minutes"
            } else {
                "minute"
            };
            res.push(format!("{} {}", self.minutes, words));
        }

        if self.hours > 0 {
            let words = if self.hours > 1 { "hours" } else { "hour" };
            res.push(format!("{} {}", self.hours, words));
        }
        if self.days > 0 {
            let words = if self.days > 1 { "days" } else { "day" };
            res.push(format!("{} {}", self.days, words));
        }

        if self.years > 0 {
            let words = if self.years > 1 { "years" } else { "year" };
            res.push(format!("{} {}", self.years, words));
        }

        // check for the length of the vector and add "and to the last e"
        res.reverse();

        let length = res.len();

        match length {
            x if x > 1 => {
                let last = &res[length - 1];

                res[length - 1] = format!("{}", format_args!("{} {}", "and", last));
                if x > 2 {
                    for i in 0..=length - 3 {
                        let first = &res[i];
                        //println!("{first}");
                        res[i] = format!("{}", format_args!("{first},"))
                    }
                }
            }

            _ => res.push("now".to_owned()),
        };

        res.join(" ")
    }
}
use std::{f32::consts::PI, fmt::Display};

use Interval::*;
fn calculate(n: u64) -> Calculator {
    let mut c = Calculator::default();
    match n {
        0..=59 => {
            c.seconds += n;
            c
        }

        x if x >= Min as u64 && x < Hour as u64 => {
            let secs = x % Min as u64;
            let minutes = x / Min as u64;
            c.seconds += secs;
            c.minutes += minutes;
            c
        }

        x if x >= Hour as u64 && x < Day as u64 => {
            //println!("{}", x / Hour as u64);
            let hours = x / Hour as u64;
            let minutes = (x - (Hour as u64 * hours)) / Min as u64;
            let secs = x - (hours * Hour as u64 + minutes * Min as u64);

            c.seconds += secs;
            c.minutes += minutes;
            c.hours += hours;
            c
        }

        x if x >= Day as u64 && x < Year as u64 => {
            let days = x / Day as u64;
            let hours = (x - (Day as u64 * days)) / Hour as u64;
            let minutes = (x - (Day as u64 * days + hours * Hour as u64)) / Min as u64;
            let secs = (x - (Day as u64 * days + hours * Hour as u64 + minutes * Min as u64));
            c.days += days;
            c.hours += hours;
            c.minutes += minutes;
            c.seconds += secs;

            c
        }

        x if x >= Year as u64 => {
            let years = x / Year as u64;
            let days = (x - (Year as u64 * years)) / Day as u64;
            let hours = (x - (Year as u64 * years + days * Day as u64)) / Hour as u64;
            let minutes =
                (x - (Year as u64 * years + days * Day as u64 + hours * Hour as u64)) / Min as u64;
            let s = x
                - (Year as u64 * years
                    + days * Day as u64
                    + hours * Hour as u64
                    + minutes * Min as u64);
            c.years += years;
            c.days += days;
            c.hours += hours;
            c.minutes += minutes;
            c.seconds += s;

            c
        }
        _ => c,
    }
}

fn format_duration(seconds: u64) -> String {
    // Complete this function

    let result = calculate(seconds);
    result.to_readable()
    // println!("{result:?}");
}

fn format_duration2(seconds: u64) -> String {
    let result = [
        ("year", 31536000, 100000),
        ("day", 86400, 365),
        ("hour", 3600, 24),
        ("minute", 60, 60),
        ("second", 1, 60),
    ]
    .iter()
    .map(|(unit, duration, modulo)| (seconds / duration % modulo, unit))
    .filter_map(|(duration, unit)| match duration {
        _ if duration == 1 => Some(format!("{duration} {unit}")),
        _ if duration != 0 => Some(format!("{duration} {unit}s")),
        _ => None,
    })
    .collect::<Vec<String>>();

    match result.len() {
        0 => String::from("now"),
        1 => result.join(""),
        _ => result
            .split_last()
            .map(|(r, l)| l.join(", ") + " and " + r)
            .unwrap(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(62), "1 minute and 2 seconds");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
    }
}
