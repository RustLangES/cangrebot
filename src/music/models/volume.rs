use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::Display,
    ops::{Deref, DerefMut},
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct MusicVolume(f32);

impl Default for MusicVolume {
    fn default() -> Self {
        MusicVolume(1.0_f32)
    }
}

impl Deref for MusicVolume {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MusicVolume {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<f32> for MusicVolume {
    type Error = MusicVolumeError;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !(0.0_f32..=1.0_f32).contains(&value) {
            return Err(MusicVolumeError::OutOfRange);
        }
        Ok(MusicVolume(value))
    }
}

impl TryFrom<usize> for MusicVolume {
    type Error = MusicVolumeError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if !(0_usize..=100_usize).contains(&value) {
            return Err(MusicVolumeError::OutOfRange);
        }
        let vf = (value as f32) / 100.0_f32;
        Self::try_from(vf)
    }
}

impl From<MusicVolume> for usize {
    fn from(val: MusicVolume) -> Self {
        (val.0 * 100.0_f32) as usize
    }
}

#[derive(Debug)]
pub enum MusicVolumeError {
    OutOfRange,
}

impl Display for MusicVolumeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MusicVolumeError::OutOfRange => f.write_str("MusicVolume should be between 0.0 ~ 1.0"),
        }
    }
}

impl Error for MusicVolumeError {}
