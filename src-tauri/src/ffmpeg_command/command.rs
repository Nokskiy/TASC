use crate::ffmpeg_command::time::Time;

pub struct Command {
    pub start_time: Time,
    pub stick_lenght: Time,
}

impl Command {
    pub fn get_command(&self) -> String {
        return format!("ffmpeg -ss {}", self.start_time);
    }
}
