use std::num::TryFromIntError;

use ab_glyph::InvalidFont;
use image::ImageError;

#[derive(Debug)]
pub enum GenWelcomeError {
    ImageGenerationError(String),
    TryFromIntError(String),
    InvalidFont(String),
}

impl From<ImageError> for GenWelcomeError {
    fn from(value: ImageError) -> Self {
        Self::ImageGenerationError(value.to_string())
    }
}

impl From<TryFromIntError> for GenWelcomeError {
    fn from(value: TryFromIntError) -> Self {
        Self::TryFromIntError(value.to_string())
    }
}

impl From<InvalidFont> for GenWelcomeError {
    fn from(value: InvalidFont) -> Self {
        Self::InvalidFont(value.to_string())
    }
}
