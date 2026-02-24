use crate::ffmpeg_command::scale::Orientation;
use crate::ffmpeg_command::time::Time;

#[derive(Debug)]
pub struct Command {
    pub start_time: Time,
    pub stick_lenght: Time,
    pub input_path: String,
    pub orientation: Orientation,
}

impl Command {
    pub fn from(
        start_time: Time,
        stick_lenght: Time,
        input_path: String,
        orientation: Orientation,
    ) -> Command {
        Command {
            start_time,
            stick_lenght,
            input_path,
            orientation,
        }
    }
    pub fn get_command(&self) -> String {
        return format!(
            "ffmpeg -ss {} -i {} --to {} -an -c:v libvpx-vp9 -c:a libopus -crf 30 -vf {}",
            self.start_time,
            self.input_path,
            self.get_end_time(),
            self.orientation.tete()
        );
    }

    pub fn get_end_time(&self) -> Time {
        self.start_time + self.stick_lenght
    }
}

#[test]
fn test_command() {
    println!("TEST COMMAND");
    let c = Command::from(
        Time::from(0, 1, 2, 40),
        Time::from(0, 0, 2, 40),
        String::from("D:\\mgr-stics\\material\\lets dance.mp4"),
        Orientation::Album,
    );

    println!("{}", c.get_command());
}
