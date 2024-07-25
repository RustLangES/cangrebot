use super::auto_leave::MusicAutoleaveType;
use super::MusicVolume;

#[derive(Debug, Clone)]
pub struct MusicConfig {
    pub auto_leave: MusicAutoleaveType,
    pub repeat: bool,
    pub volume: MusicVolume,
}

impl MusicConfig {
    pub fn new() -> Self {
        Self {
            auto_leave: MusicAutoleaveType::Off,
            repeat: false,
            volume: MusicVolume::default(),
        }
    }
}
