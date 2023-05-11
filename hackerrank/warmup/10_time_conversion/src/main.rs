// https://www.hackerrank.com/challenges/time-conversion/problem?isFullScreen=true

use ::std::{fmt, io};

#[derive(Clone, Copy, Debug)]
enum Meridian {
    AM,
    PM,
}

impl fmt::Display for Meridian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Meridian::AM => write!(f, "AM"),
            Meridian::PM => write!(f, "PM"),
        }
    }
}

#[derive(Debug)]
struct DateTime {
    hour: u8,
    minutes: u8,
    seconds: u8,
    meridian: Meridian,
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} hours {} minuts {} seconds - {}",
            self.hour, self.minutes, self.seconds, self.meridian
        )
    }
}

impl DateTime {
    fn cast_to_24_hour(&self) -> String {
        let hour = match self.meridian {
            Meridian::AM => match self.hour {
                12 => 0,
                _ => self.hour,
            },
            Meridian::PM => match self.hour {
                12 => 12,
                _ => self.hour + 12,
            },
        };

        let hours = if hour > 9 {
            format!("{}", hour)
        } else {
            format!("0{}", hour)
        };
        let minutes = if self.minutes > 9 {
            format!("{}", self.minutes)
        } else {
            format!("0{}", self.minutes)
        };
        let seconds = if self.seconds > 9 {
            format!("{}", self.seconds)
        } else {
            format!("0{}", self.seconds)
        };
        format!("{}:{}:{}", hours, minutes, seconds)
    }
}

fn read_input() -> DateTime {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let time_split: Vec<&str> = buffer.trim().split(":").collect();

    let minutes_chars: Vec<char> = time_split.get(2).unwrap().chars().collect();
    let meridian = if minutes_chars.get(2).unwrap() == &'A' {
        Meridian::AM
    } else {
        Meridian::PM
    };

    let datetime = DateTime {
        hour: time_split
            .get(0)
            .unwrap()
            .parse::<u8>()
            .expect("Failed Parsing Hours"),
        minutes: time_split
            .get(1)
            .unwrap()
            .parse::<u8>()
            .expect("Failed Parsing Minutes"),
        seconds: minutes_chars[0..=1]
            .iter()
            .collect::<String>()
            .parse::<u8>()
            .expect("Failed Parsing seconds"),
        meridian,
    };
    datetime
}

fn main() {
    let time_of_day = read_input();
    println!("{}", time_of_day.cast_to_24_hour());
}
