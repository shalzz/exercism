use std::fmt;

const MINUTES_PER_HOUR: i32 = 60;
const MINUTES_PER_DAY: i32 = 24 * MINUTES_PER_HOUR;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    // The number of minutes past midnight.
    // Invariant: 0 <= minutes < MINUTES_PER_DAY
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: Self::wrap_minutes(MINUTES_PER_HOUR * hours + minutes),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }

    fn wrap_minutes(minutes: i32) -> i32 {
        minutes.rem_euclid(MINUTES_PER_DAY)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.minutes / MINUTES_PER_HOUR;
        let minutes = self.minutes % MINUTES_PER_HOUR;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
