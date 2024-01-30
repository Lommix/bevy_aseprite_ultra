use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    sprite::Anchor,
    utils::HashMap,
};

use sprity::aseprite::{binary::chunks::tags::AnimationDirection, loader::AsepriteFile};

pub struct AsepriteLoaderPlugin;
impl Plugin for AsepriteLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Aseprite>();
        app.register_asset_loader(AsepriteLoader);
        app.add_systems(
            Update,
            build_atlas.run_if(on_event::<AssetEvent<Aseprite>>()),
        );
    }
}

#[derive(Asset, Default, TypePath, Debug)]
pub struct Aseprite {
    pub atlas: Option<Handle<TextureAtlas>>,
    pub slices: HashMap<String, SliceMeta>,
    pub tags: HashMap<String, TagMeta>,

    atlas_buffer: Vec<Handle<Image>>,
    atlas_frame_lookup: Vec<usize>,
}

impl Aseprite {
    pub fn frame_index(&self, frame: usize) -> usize {
        self.atlas_frame_lookup[frame]
    }
}

#[derive(Debug)]
pub struct TagMeta {
    pub direction: AnimationDirection,
    pub range: std::ops::Range<u16>,
    pub repeat: u16,
}

#[derive(Debug)]
pub struct SliceMeta {
    pub rect: Rect,
    pub pivot: Option<Vec2>,
    pub nine_patch: Option<Vec4>,
}

impl From<&SliceMeta> for Anchor {
    fn from(value: &SliceMeta) -> Self {
        match value.pivot {
            Some(pivot) => {
                let size = value.rect.size();
                let uv = (pivot.min(size).max(Vec2::ZERO) / size) - Vec2::new(0.5, 0.5);
                Anchor::Custom(uv)
            }
            None => Anchor::Center,
        }
    }
}

#[derive(Default)]
pub struct AsepriteLoader;

impl AssetLoader for AsepriteLoader {
    type Asset = Aseprite;
    type Settings = ();
    type Error = anyhow::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let raw = AsepriteFile::load(&bytes)?;
            let mut atlas_buffer = Vec::new();

            for (index, _frame) in raw.frames().iter().enumerate() {
                let (width, height) = raw.size();
                let mut buffer = vec![0; width as usize * height as usize * 4];

                let _hash = raw.combined_frame_image(index, buffer.as_mut_slice())?;
                let image = Image::new(
                    Extent3d {
                        width: width as u32,
                        height: height as u32,
                        depth_or_array_layers: 1,
                    },
                    TextureDimension::D2,
                    buffer.clone(),
                    TextureFormat::Rgba8UnormSrgb,
                );

                let handle =
                    load_context.add_labeled_asset((format!("frame_{}", index)).into(), image);
                atlas_buffer.push(handle);
            }

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

                slices.insert(
                    slice.name.into(),
                    SliceMeta {
                        rect: Rect::from_corners(min, max),
                        pivot,
                        nine_patch,
                    },
                );
            });
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

            Ok(Aseprite {
                atlas_buffer,
                slices,
                tags,
                ..default()
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["aseprite", "ase"]
    }
}

#[derive(Component)]
pub(crate) struct Dirty;

fn build_atlas(
    enties: Query<(Entity, &Handle<Aseprite>), (With<Handle<Image>>, With<Sprite>)>,
    mut events: EventReader<AssetEvent<Aseprite>>,
    mut images: ResMut<Assets<Image>>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    mut aseprites: ResMut<Assets<Aseprite>>,
    mut cmd: Commands,
) {
    events.read().for_each(|event| match event {
        AssetEvent::Added { id } | AssetEvent::Modified { id } => {
            let Some(asesprite) = aseprites.get(*id) else {
                debug!("Aseprite not found, how an this be");
                return;
            };

            if asesprite.atlas.is_some() {
                return;
            }

            let asesprite = aseprites.get_mut(*id).unwrap();

            let mut atlas = TextureAtlasBuilder::default();
            let mut frame_handles = vec![];

            asesprite.atlas_buffer.drain(..).for_each(|image_handle| {
                let image = images
                    .get_mut(image_handle.clone())
                    .expect("Image not found, how can this be");
                atlas.add_texture(image_handle.clone().into(), image);
                frame_handles.push(image_handle);
            });

            let atlas = atlas.finish(&mut images).unwrap();
            asesprite.atlas_frame_lookup = frame_handles
                .drain(..)
                .map(|handle| atlas.get_texture_index(handle).unwrap())
                .collect();

            asesprite.atlas = Some(atlases.add(atlas));

            // @todo, can this be done better?
            enties
                .iter()
                .filter(|(_, handle)| handle.id() == *id)
                .for_each(|(entity, _)| {
                    cmd.entity(entity)
                        .remove::<Sprite>()
                        .remove::<Handle<Image>>();
                });
        }
        _ => {}
    });
}
