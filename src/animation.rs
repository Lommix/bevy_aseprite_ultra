use crate::loader::Aseprite;
use bevy::prelude::*;
use sprity::aseprite::binary::chunks::tags::AnimationDirection as RawDirection;

pub struct AsepriteAnimationPlugin;
impl Plugin for AsepriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (insert_aseprite_animation, update_aseprite_animation),
        );
        app.add_systems(
            Update,
            reload_aseprite_animation.run_if(on_event::<AssetEvent<Aseprite>>()),
        );
    }
}

#[derive(Bundle, Default)]
pub struct AsepriteAnimationBundle {
    pub aseprite: Handle<Aseprite>,
    pub animation: AnimationTag,
}

#[derive(Component, Default)]
pub struct AnimationFrame(usize);

#[derive(Component, Default)]
pub struct AnimationTag(String);

impl Into<AnimationTag> for &str {
    fn into(self) -> AnimationTag {
        AnimationTag(self.to_string())
    }
}

#[derive(Event)]
pub struct AnimationFinished(Entity);

#[derive(Default, Component)]
pub enum AnimationDirection {
    #[default]
    Forward,
    Reverse,
    PingPong,
    PingPongReverse,
}

impl From<RawDirection> for AnimationDirection {
    fn from(direction: RawDirection) -> AnimationDirection {
        match direction {
            RawDirection::Forward => AnimationDirection::Forward,
            RawDirection::Reverse => AnimationDirection::Reverse,
            RawDirection::PingPong => AnimationDirection::PingPong,
            RawDirection::PingPongReverse => AnimationDirection::PingPongReverse,
            _ => panic!("Invalid AnimationDirection"),
        }
    }
}

#[derive(Default, Component)]
pub enum AnimationRepeat {
    #[default]
    Loop,
    Count(u32),
}

impl From<u16> for AnimationRepeat {
    fn from(value: u16) -> Self {
        match value {
            0 => AnimationRepeat::Loop,
            n => AnimationRepeat::Count(u32::from(n)),
        }
    }
}

fn insert_aseprite_animation(
    query: Query<(Entity, &Handle<Aseprite>), Without<Sprite>>,
    tag_query: Query<&AnimationTag>,
    mut cmd: Commands,
    asesprites: Res<Assets<Aseprite>>,
    atlases: Res<Assets<TextureAtlas>>,
) {
    query.iter().for_each(|(entity, aseprite_handle)| {
        let Some(aseprite) = asesprites.get(aseprite_handle) else {
            return;
        };

        let Some(atlas_handle) = aseprite.atlas.as_ref() else {
            return;
        };

        let Some(atlas) = atlases.get(atlas_handle) else {
            return;
        };

        let Ok(animation) = tag_query.get(entity) else {
            return;
        };

        // @todo
        // Non animation tags?
        let tag = aseprite.tags.get(&animation.0).expect(
            format!(
                "animation tag '{}' not found in '{:?}'",
                animation.0,
                aseprite_handle.path()
            )
            .as_str(),
        );

        let start_frame_index = usize::from(tag.range.start);
        let rect = atlas.textures[start_frame_index];

        cmd.entity(entity)
            .insert(SpriteBundle {
                sprite: Sprite {
                    rect: Some(rect),
                    ..default()
                },
                texture: atlas.texture.clone(),
                ..default()
            })
            .insert(AnimationFrame(0))
            .insert(AnimationRepeat::from(tag.repeat))
            .insert(AnimationDirection::from(tag.direction));
    });
}

fn reload_aseprite_animation() {}

fn update_aseprite_animation() {}
