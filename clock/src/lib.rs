use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

const HOURS_IN_DAY    : i32 = 24;
const MINUTES_IN_HOUR : i32 = 60;
const MINUTES_IN_DAY  : i32 = MINUTES_IN_HOUR * HOURS_IN_DAY;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // '.rem_euclid()' does a modulo that is different than the one performed by Rust with negative numbers.
        let minutes = (hours * MINUTES_IN_HOUR + minutes).rem_euclid(MINUTES_IN_DAY);

        Clock {
            minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes / MINUTES_IN_HOUR,
            self.minutes % MINUTES_IN_HOUR
        )
    }
}
