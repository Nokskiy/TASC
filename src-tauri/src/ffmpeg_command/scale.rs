#[derive(Debug)]
pub enum Orientation {
    Album,
    Portrait,
}

impl Orientation {
    pub fn tete(&self) -> String {
        match self {
            Orientation::Album => String::from("scale=512:-1"),
            Orientation::Portrait => String::from("scale=-1:512"),
        }
    }
}
