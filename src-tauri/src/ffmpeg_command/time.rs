use std::fmt;

pub struct Time {
    h: u8,
    m: u8,
    s: u8,
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}::{}::{}",
            self.cspot(&self.h),
            self.cspot(&self.m),
            self.cspot(&self.s)
        )
    }
}

impl Time {
    pub fn from(h: u8, m: u8, s: u8) -> Time {
        Time { h, m, s }
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
