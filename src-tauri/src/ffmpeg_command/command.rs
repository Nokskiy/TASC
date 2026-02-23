use crate::ffmpeg_command::time::Time;

#[derive(Debug)]
pub struct Command {
    pub start_time: Time,
    pub stick_lenght: Time,
}

impl Command {
    pub fn get_command(&self) -> String {
        return format!("ffmpeg -ss {}", self.start_time);
    }

    pub fn get_end_time(&self) -> Time {
        self.start_time + self.stick_lenght
    }
}

#[test]
fn test_command() {
    println!("TEST COMMAND");
    let c = Command {
        start_time: Time::from(0, 1, 2, 40),
        stick_lenght: Time::from(0, 0, 2, 40),
    };

    println!("{:#?}", c.get_end_time());
}
