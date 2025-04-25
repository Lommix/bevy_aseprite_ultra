use aseprite_loader::loader::{LoadImageError, LoadSpriteError};
use bevy::image::TextureAtlasBuilderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AsepriteError {
    #[error("failed to build atlas")]
    TextureAtlasError(#[from] TextureAtlasBuilderError),
    #[error("failed to read aseprite binary")]
    LoadingError(#[from] LoadSpriteError),
    #[error("failed to combine aseprite layers")]
    LoadingImageError(#[from] LoadImageError),
    #[error("failed to read byte stream to end")]
    ReadError,
}
