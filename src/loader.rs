use crate::error::AsepriteError;
use aseprite_loader::{binary::chunks::tags::AnimationDirection, loader::AsepriteFile};
use bevy::{
    asset::{io::Reader, AssetLoader, RenderAssetUsages},
    image::ImageSampler,
    platform::collections::HashMap,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    sprite::Anchor,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct AsepriteLoaderPlugin;
impl Plugin for AsepriteLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Aseprite>();
        app.register_asset_loader(AsepriteLoader);
    }
}

//@todo: if this can be serialized, we basicly have a intermediate binary
//represantion and can make use of the asset prepocessor. No longer need
//to ship or bundle aseprite binaries into your release.
#[derive(Asset, Default, TypePath, Debug)]
#[cfg_attr(feature = "asset_processing", derive(Serialize, Deserialize))]
pub struct Aseprite {
    pub slices: HashMap<String, SliceMeta>,
    pub tags: HashMap<String, TagMeta>,
    pub frame_durations: Vec<std::time::Duration>,
    #[cfg_attr(feature = "asset_processing", serde(skip))]
    pub atlas_layout: Handle<TextureAtlasLayout>,
    #[cfg_attr(feature = "asset_processing", serde(skip))]
    pub atlas_image: Handle<Image>,
    pub(crate) frame_indicies: Vec<usize>,
}

impl Aseprite {
    pub fn get_atlas_index(&self, frame: usize) -> usize {
        if self.frame_indicies.len() <= frame {
            return self.frame_indicies.last().cloned().unwrap_or_default();
        }
        self.frame_indicies[frame]
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "asset_processing", derive(Serialize, Deserialize))]
pub struct TagMeta {
    #[cfg_attr(feature = "asset_processing", serde(with = "AnimationDirectionDef"))]
    pub direction: AnimationDirection,
    pub range: std::ops::RangeInclusive<u16>,
    pub repeat: u16,
}

#[cfg(feature = "asset_processing")]
#[derive(Serialize, Deserialize)]
#[serde(remote = "AnimationDirection")]
enum AnimationDirectionDef {
    Forward,
    Reverse,
    PingPong,
    PingPongReverse,
    Unknown(u8),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "asset_processing", derive(Serialize, Deserialize))]
pub struct SliceKeyMeta {
    pub frame: usize,
    pub rect: Rect,
    pub pivot: Option<Vec2>,
    pub nine_patch: Option<Vec4>,
}

#[derive(Debug)]
#[cfg_attr(feature = "asset_processing", derive(Serialize, Deserialize))]
pub struct SliceMeta {
    pub rect: Rect,
    pub atlas_id: usize,
    pub pivot: Option<Vec2>,
    pub nine_patch: Option<Vec4>,
    pub keys: Vec<SliceKeyMeta>,
}

impl From<&SliceMeta> for Anchor {
    fn from(value: &SliceMeta) -> Self {
        match value.pivot {
            Some(pivot) => {
                let size = value.rect.size();
                let uv = (pivot.min(size).max(Vec2::ZERO) / size) - Vec2::new(0.5, 0.5);
                Anchor(uv * Vec2::new(1.0, -1.0))
            }
            None => Anchor::CENTER,
        }
    }
}

#[derive(Default, TypePath)]
pub struct AsepriteLoader;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AsepriteLoaderSettings {
    pub sampler: ImageSampler,
}

impl Default for AsepriteLoaderSettings {
    fn default() -> Self {
        Self {
            sampler: ImageSampler::nearest(),
        }
    }
}

impl AssetLoader for AsepriteLoader {
    type Asset = Aseprite;
    type Settings = AsepriteLoaderSettings;
    type Error = super::error::AsepriteError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &Self::Settings,
        load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader
            .read_to_end(&mut bytes)
            .await
            .map_err(|_| AsepriteError::ReadError)?;

        let raw = AsepriteFile::load(&bytes)?;

        let mut frame_images = Vec::new();
        let mut atlas_builder = TextureAtlasBuilder::default();
        atlas_builder.max_size(UVec2::splat(4096));

        let mut images = Vec::new();

        for (index, _frame) in raw.frames().iter().enumerate() {
            let (width, height) = raw.size();
            let mut buffer = vec![0; width as usize * height as usize * 4];

            let _hash = raw.combined_frame_image(index, buffer.as_mut_slice())?;

            let image = Image {
                sampler: settings.sampler.clone(),
                ..Image::new(
                    Extent3d {
                        width: width as u32,
                        height: height as u32,
                        depth_or_array_layers: 1,
                    },
                    TextureDimension::D2,
                    buffer.clone(),
                    TextureFormat::Rgba8UnormSrgb,
                    RenderAssetUsages::default(),
                )
            };
            images.push(image);
        }

        for image in images.iter() {
            let handle_id = AssetId::Uuid {
                uuid: Uuid::new_v4(),
            };

            frame_images.push(handle_id);
            atlas_builder.add_texture(Some(handle_id), &image);
        }

        // ----------------------------- atlas
        let (mut layout, source, image) = atlas_builder.build()?;

        let frame_indicies = frame_images
            .iter()
            .map(|id| source.texture_ids.get(id).cloned().unwrap())
            .collect::<Vec<_>>();

        // ----------------------------- slices
        let mut slices = HashMap::new();
        raw.slices().iter().for_each(|slice| {
            let slice_key = slice.slice_keys.first().unwrap();

            let min = Vec2::new(slice_key.x as f32, slice_key.y as f32);
            let max = min + Vec2::new(slice_key.width as f32, slice_key.height as f32);

            let pivot = match slice_key.pivot {
                Some(pivot) => Some(Vec2::new(pivot.x as f32, pivot.y as f32)),
                None => None,
            };

            let nine_patch = match slice_key.nine_patch {
                Some(nine_patch) => Some(Vec4::new(
                    nine_patch.x as f32,
                    nine_patch.y as f32,
                    nine_patch.width as f32,
                    nine_patch.height as f32,
                )),
                None => None,
            };

            let layout_id = layout.add_texture(URect::from_corners(min.as_uvec2(), max.as_uvec2()));

            let mut keys = Vec::new();
            for key in &slice.slice_keys {
                let k_min = Vec2::new(key.x as f32, key.y as f32);
                let k_max = k_min + Vec2::new(key.width as f32, key.height as f32);

                let k_pivot = key.pivot.map(|p| Vec2::new(p.x as f32, p.y as f32));

                let k_nine_patch = key.nine_patch.map(|np| {
                    Vec4::new(np.x as f32, np.y as f32, np.width as f32, np.height as f32)
                });

                keys.push(SliceKeyMeta {
                    frame: key.frame_number as usize,
                    rect: Rect::from_corners(k_min, k_max),
                    pivot: k_pivot,
                    nine_patch: k_nine_patch,
                });
            }

            slices.insert(
                slice.name.into(),
                SliceMeta {
                    rect: Rect::from_corners(min, max),
                    atlas_id: layout_id,
                    pivot,
                    nine_patch,
                    keys,
                },
            );
        });

        let atlas_layout = load_context.add_labeled_asset("atlas_layout".into(), layout);
        let atlas_image = load_context.add_labeled_asset("atlas_texture".into(), image);

        // ---------------------------- tags
        let mut tags = HashMap::new();
        raw.tags().iter().for_each(|tag| {
            tags.insert(
                tag.name.clone(),
                TagMeta {
                    direction: tag.direction,
                    range: tag.range.clone(),
                    repeat: tag.repeat.unwrap_or(0),
                },
            );
        });

        // ---------------------------- frames
        let frame_durations = raw
            .frames()
            .iter()
            .map(|frame| std::time::Duration::from_millis(u64::from(frame.duration)))
            .collect();

        Ok(Aseprite {
            slices,
            tags,
            frame_durations,
            atlas_layout,
            atlas_image,
            frame_indicies,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["aseprite", "ase"]
    }
}
