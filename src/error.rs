use aseprite_loader::loader::{LoadImageError, LoadSpriteError};
use bevy::sprite::TextureAtlasBuilderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AsepriteError {
    #[error("failed to build atlas")]
    TextureAtlasError(#[from] TextureAtlasBuilderError),
    #[error("failed to read aseprite binary")]
    LoadingError(#[from] LoadSpriteError),
    #[error("failed to combine aseprite layers")]
    LoadingImageError(#[from] LoadImageError),
    #[error("failed to read byte stream")]
    ReadError,
    #[cfg(feature = "asset_processing")]
    #[error("failed to write to processed asset")]
    WriteError,
    #[cfg(feature = "asset_processing")]
    #[error("failed to serialize aseprite data")]
    SerializeError(#[from] rmp_serde::encode::Error),
    #[cfg(feature = "asset_processing")]
    #[error("failed to deserialize processed aseprite data {0}")]
    DeserializeError(#[from] rmp_serde::decode::Error),
    #[cfg(feature = "asset_processing")]
    #[error("failed to write image data")]
    ImageError(#[from] image::ImageError),
    #[cfg(feature = "asset_processing")]
    #[error("failed to read image data")]
    BevyTextureError(#[from] bevy::image::TextureError),
}
