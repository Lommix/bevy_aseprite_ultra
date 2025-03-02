use bevy::{
    image::ImageSamplerDescriptor,
    math::vec3,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d, Material2dPlugin},
};
use bevy_aseprite_ultra::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor::nearest(),
        }))
        .add_plugins(AsepriteUltraPlugin)
        .add_systems(Startup, setup)
        .add_plugins(Material2dPlugin::<MyMaterial>::default())
        .add_animation_render_system(render_aseprite_animation_my_material)
        .run();
}

fn setup(
    mut cmd: Commands,
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut my_material: ResMut<Assets<MyMaterial>>,
) {
    cmd.spawn((Camera2d, Transform::default().with_scale(Vec3::splat(0.15))));

    cmd.spawn((
        AseAnimation {
            animation: Animation::tag("walk-right"),
            aseprite: server.load("player.aseprite"),
        },
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(my_material.add(MyMaterial::default())),
        Transform {
            translation: vec3(15.0, 0.0, 0.0),
            scale: Vec3::splat(50.0),
            ..default()
        },
    ));
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
        aa_material.texture_size = atlas_layout.size;
        aa_material.time = time.elapsed_secs();
    }
}

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath, Default)]
pub struct MyMaterial {
    #[texture(1)]
    #[sampler(2)]
    image: Handle<Image>,
    #[uniform(3)]
    texture_min: UVec2,
    #[uniform(4)]
    texture_max: UVec2,
    #[uniform(5)]
    texture_size: UVec2,
    #[uniform(6)]
    time: f32,
}

impl Material2d for MyMaterial {
    fn fragment_shader() -> ShaderRef {
        "my_shader.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}
