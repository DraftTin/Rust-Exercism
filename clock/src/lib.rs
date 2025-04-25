use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    total_minutes: i64,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            total_minutes: ((hours * 60) as i64 + minutes as i64).rem_euclid(60 * 24),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            total_minutes: (self.total_minutes + minutes as i64).rem_euclid(60 * 24),
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = self.total_minutes / 60;
        let minutes = self.total_minutes % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
