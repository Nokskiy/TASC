use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Time {
    h: u8,
    m: u8,
    s: u8,
    ms: u8,
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}::{}::{}.{}",
            self.cspot(&self.h),
            self.cspot(&self.m),
            self.cspot(&self.s),
            self.ms
        )
    }
}
//Addition only: no subtraction is required in the project.

impl Copy for Time {}

impl Add for Time {
    type Output = Time;
    fn add(self, other: Time) -> Time {
        let mut new_ms: u8 = 0;
        let mut new_s: u8 = 0;
        let mut new_m: u8 = 0;
        let mut new_h: u8 = 0;
        if self.ms + other.ms > 100 {
            new_ms = self.ms + other.ms - 100;
            new_s += 1;
        } else {
            new_ms = self.ms + other.ms;
        }

        if self.s + other.s + new_s > 60 {
            new_s = self.s + other.s + new_s - 60;
            new_m += 1;
        } else {
            new_s = self.s + other.s + new_s;
        }

        if self.m + other.m + new_m > 60 {
            new_m = self.m + other.m + new_m - 60;
            new_h += 1;
        } else {
            new_m = self.m + other.m;
        }

        if self.h + other.h + new_h > 24 {
            panic!("time.h sum bigger than 24");
        } else {
            new_h = self.h + other.h + new_h;
        }

        return Time::from(new_h, new_m, new_s, new_ms);
    }
}

#[test]
fn test_add() {
    println!("TEST ADD");
    let t1 = Time::from(2, 5, 40, 60);
    let t2 = Time::from(0, 2, 0, 50);
    let sum = t1 + t2;

    println!("{:#?}", sum);
}

impl Time {
    pub fn from(h: u8, m: u8, s: u8, ms: u8) -> Time {
        Time { h, m, s, ms }
    }

    fn cspot(&self, p: &u8) -> String {
        let mut converted_h = String::new();

        if *p < 10 {
            converted_h = format!("0{}", p);
        } else if *p >= 10 {
            converted_h = format!("{}", p);
        }

        return converted_h;
    }
}
