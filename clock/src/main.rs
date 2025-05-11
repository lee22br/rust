use std::fmt;
use std::ops::Rem;

//#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: minutes / 60 + hours,
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
    println!("Hello, world!");
}
