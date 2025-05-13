use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: (minutes / 60 + hours) % 24,
            minutes: minutes % 60,
        }
    }
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}


fn main() {
    println!("Clock: {}", prinClock::new(8, 0).toString());
    let clock = Clock::new(10, 0).add_minutes(3);
    println!("Clock Add: {}", clock.to_string());
}
