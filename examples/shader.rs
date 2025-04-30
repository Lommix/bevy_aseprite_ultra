use bevy::{
    image::ImageSamplerDescriptor,
    prelude::*,
    render::render_resource::AsBindGroup,
    sprite::{Material2d, Material2dPlugin},
};
use bevy_aseprite_ultra::prelude::*;

/*
 * Shader Example
 * render any animation to any custom Material.
 */

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor::nearest(),
        }))
        .add_plugins(AsepriteUltraPlugin)
        .add_plugins(Material2dPlugin::<MyMaterial>::default())
        .add_systems(Startup, setup)
        .add_animation_render_system(render_aseprite_animation_my_material)
        .run();
}

pub fn render_aseprite_animation_my_material(
    mut animations: Query<(
        &mut AseAnimation,
        &AnimationState,
        &MeshMaterial2d<MyMaterial>,
    )>,
    aseprites: Res<Assets<Aseprite>>,
    mut aa_materials: ResMut<Assets<MyMaterial>>,
    time: Res<Time>,
    atlas_layouts: Res<Assets<TextureAtlasLayout>>,
) {
    for (animation, state, aa_material) in &mut animations {
        let Some(aseprite) = aseprites.get(&animation.aseprite) else {
            continue;
        };
        let Some(aa_material) = aa_materials.get_mut(aa_material) else {
            continue;
        };
        let Some(atlas_layout) = atlas_layouts.get(&aseprite.atlas_layout) else {
            return;
        };
        aa_material.image = aseprite.atlas_image.clone();
        let index = aseprite.get_atlas_index(usize::from(state.current_frame));
        aa_material.texture_min = atlas_layout.textures[index].min;
        aa_material.texture_max = atlas_layout.textures[index].max;
        aa_material.time = time.elapsed_secs();
    }
}

// We make our asset a component at the same time, since the trait can only target components.
// Later we need a glue system to sync the change back to the material.
#[derive(AsBindGroup, Component, Debug, Clone, Asset, TypePath, Default)]
pub struct MyMaterial {
    #[texture(1)]
    #[sampler(2)]
    image: Handle<Image>,
    #[uniform(3)]
    texture_min: UVec2,
    #[uniform(4)]
    texture_max: UVec2,
    #[uniform(5)]
    time: f32,
}

impl Material2d for MyMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "my_shader.wgsl".into()
    }
}

fn setup(
    mut cmd: Commands,
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MyMaterial>>,
) {
    cmd.spawn((Camera2d, Transform::default().with_scale(Vec3::splat(0.15))));
    cmd.spawn((
        AseAnimation {
            aseprite: server.load("player.aseprite"),
            animation: Animation::tag("walk-down"),
        },
        Mesh2d(meshes.add(Mesh::from(Rectangle::from_size(Vec2::splat(100.0))))),
        MeshMaterial2d(materials.add(MyMaterial::default())),
    ));
}
