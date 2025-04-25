use std::io::Cursor;

use bevy::{
    asset::{
        processor::LoadTransformAndSave,
        saver::{AssetSaver, SavedAsset},
        transformer::IdentityAssetTransformer,
        AssetLoader, AsyncWriteExt,
    },
    image::{CompressedImageFormats, ImageFormatSetting, ImageLoaderSettings, ImageType},
    prelude::*,
    render::renderer::RenderDevice,
};
use image::ImageFormat;
use serde::{Deserialize, Serialize};

use crate::{
    error::AsepriteError,
    loader::{Aseprite, AsepriteLoader},
};

pub struct AsepriteProcessorPlugin;

impl Plugin for AsepriteProcessorPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_processor::<LoadTransformAndSave<AsepriteLoader, IdentityAssetTransformer<Aseprite>, AsepriteSaver>>(LoadTransformAndSave::new(IdentityAssetTransformer::new(), AsepriteSaver));
        app.set_default_asset_processor::<LoadTransformAndSave<AsepriteLoader, IdentityAssetTransformer<Aseprite>, AsepriteSaver>>("aseprite");
        app.set_default_asset_processor::<LoadTransformAndSave<AsepriteLoader, IdentityAssetTransformer<Aseprite>, AsepriteSaver>>("ase");
    }

    fn finish(&self, app: &mut App) {
        let supported_compressed_formats = match app.world().get_resource::<RenderDevice>() {
            Some(render_device) => CompressedImageFormats::from_features(render_device.features()),

            None => CompressedImageFormats::NONE,
        };

        app.register_asset_loader(ProcessedAsepriteLoader {
            supported_compressed_formats,
        });
    }
}

#[derive(Serialize)]
struct AsepriteSerialize<'a> {
    #[serde(flatten)]
    pub aseprite: &'a Aseprite,
    pub atlas_layout: &'a TextureAtlasLayout,
}

#[derive(Deserialize)]
struct AsepriteDeserialize {
    #[serde(flatten)]
    pub aseprite: Aseprite,
    pub atlas_layout: TextureAtlasLayout,
}

struct AsepriteSaver;

impl AssetSaver for AsepriteSaver {
    type Asset = Aseprite;

    type Settings = ();

    type OutputLoader = ProcessedAsepriteLoader;

    type Error = super::error::AsepriteError;

    async fn save(
        &self,
        writer: &mut bevy::asset::io::Writer,
        asset: bevy::asset::saver::SavedAsset<'_, Self::Asset>,
        _settings: &Self::Settings,
    ) -> Result<<Self::OutputLoader as bevy::asset::AssetLoader>::Settings, Self::Error> {
        let texture_atlas_layout: SavedAsset<TextureAtlasLayout> = asset
            .get_labeled("atlas_layout")
            .expect("atlas_layout should exist");
        let atlas_texture: SavedAsset<Image> = asset
            .get_labeled("atlas_texture")
            .expect("atlas_texture should exist");

        let aseprite_ser = AsepriteSerialize {
            aseprite: asset.get(),
            atlas_layout: texture_atlas_layout.get(),
        };

        let msgpack_buf = rmp_serde::to_vec(&aseprite_ser)?;

        // Write length of msgpack segment
        writer
            .write_all(&((msgpack_buf.len() as u64).to_be_bytes()))
            .await
            .map_err(|_| AsepriteError::WriteError)?;

        // Write msgpack itself
        writer
            .write_all(&msgpack_buf)
            .await
            .map_err(|_| AsepriteError::WriteError)?;

        let mut image_buf = Vec::new();
        let mut image_write = Cursor::new(&mut image_buf);

        let dynamic = atlas_texture
            .clone()
            .try_into_dynamic()
            .expect("Atlas image should be of a supported image type");
        dynamic.write_to(&mut image_write, ImageFormat::Qoi)?;

        writer
            .write_all(&image_buf)
            .await
            .map_err(|_| AsepriteError::WriteError)?;

        Ok(ImageLoaderSettings {
            format: ImageFormatSetting::Format(bevy::prelude::ImageFormat::Qoi),
            is_srgb: atlas_texture.texture_descriptor.format.is_srgb(),
            sampler: atlas_texture.sampler.clone(),
            asset_usage: atlas_texture.asset_usage,
        })
    }
}

struct ProcessedAsepriteLoader {
    supported_compressed_formats: CompressedImageFormats,
}

impl AssetLoader for ProcessedAsepriteLoader {
    type Asset = Aseprite;

    type Settings = ImageLoaderSettings;

    type Error = AsepriteError;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        settings: &Self::Settings,
        load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut buf = Vec::new();
        reader
            .read_to_end(&mut buf)
            .await
            .map_err(|_| AsepriteError::ReadError)?;

        let msgpack_size = u64::from_be_bytes(
            (&buf[..8])
                .try_into()
                .expect("Buffer should be at least 8 bytes long"),
        );

        let msgpack = &buf[8..8 + msgpack_size as usize];

        let de: AsepriteDeserialize = rmp_serde::from_slice(msgpack)?;

        let atlas_texture_buf = &buf[8 + msgpack_size as usize..];

        let atlas_texture = Image::from_buffer(
            atlas_texture_buf,
            ImageType::Format(bevy::prelude::ImageFormat::Qoi),
            self.supported_compressed_formats,
            settings.is_srgb,
            settings.sampler.clone(),
            settings.asset_usage,
        )?;

        let atlas_layout_handle =
            load_context.add_labeled_asset("atlas_layout".into(), de.atlas_layout);
        let atlas_texture_handle =
            load_context.add_labeled_asset("atlas_texture".into(), atlas_texture);

        Ok(Aseprite {
            atlas_layout: atlas_layout_handle,
            atlas_image: atlas_texture_handle,
            ..de.aseprite
        })
    }
}
