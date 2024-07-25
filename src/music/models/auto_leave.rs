#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MusicAutoleaveType {
    On,
    Empty,
    Silent,
    Off,
}

impl TryFrom<&str> for MusicAutoleaveType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "on" => Ok(Self::On),
            "empty" => Ok(Self::Empty),
            "silent" => Ok(Self::Silent),
            "off" => Ok(Self::Off),
            _ => Err("Cannot parse AutoLeave"),
        }
    }
}
