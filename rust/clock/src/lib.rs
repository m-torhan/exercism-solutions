use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut total_minutes = minutes + hours * 60;

        total_minutes = total_minutes.rem_euclid(24 * 60);

        Clock {
            minutes: total_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut total_minutes = self.minutes + minutes;

        total_minutes = total_minutes.rem_euclid(24 * 60);

        return Clock {
            minutes: total_minutes,
        };
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}
