use std::time::Duration;

use bevy::{
    image::ImageSamplerDescriptor, prelude::*, render::render_resource::AsBindGroup,
    shader::ShaderRef, time::common_conditions::on_timer,
};
use bevy_aseprite_ultra::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor::nearest(),
        }))
        .add_plugins(AsepriteUltraPlugin)
        .add_plugins(MaterialPlugin::<MyMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, render_animation::<MeshMaterial3d<MyMaterial>>)
        .add_systems(Update, rotate_cube)
        .add_systems(Update, render_slice::<MeshMaterial3d<MyMaterial>>)
        .add_systems(
            Update,
            change_slice.run_if(on_timer(Duration::from_secs(2))),
        )
        .run();
}

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath, Default)]
pub struct MyMaterial {
    #[texture(0)]
    #[sampler(1)]
    image: Handle<Image>,
    #[uniform(2)]
    texture_min: UVec2,
    #[uniform(3)]
    texture_max: UVec2,
    #[uniform(4)]
    time: f32,
}

impl Material for MyMaterial {
    fn fragment_shader() -> ShaderRef {
        "my_shader3d.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}

impl RenderAnimation for MyMaterial {
    type Extra<'e> = (Res<'e, Time>, Res<'e, Assets<TextureAtlasLayout>>);
    fn render_animation(
        &mut self,
        aseprite: &Aseprite,
        state: &AnimationState,
        extra: &mut Self::Extra<'_>,
    ) {
        let Some(atlas_layout) = extra.1.get(&aseprite.atlas_layout) else {
            return;
        };
        self.image = aseprite.atlas_image.clone();
        let index = aseprite.get_atlas_index(usize::from(state.current_frame));
        self.texture_min = atlas_layout.textures[index].min;
        self.texture_max = atlas_layout.textures[index].max;
        self.time = extra.0.elapsed_secs();
    }
}

impl RenderSlice for MyMaterial {
    type Extra<'e> = Res<'e, Time>;
    fn render_slice(
        &mut self,
        aseprite: &Aseprite,
        slice_meta: &SliceMeta,
        extra: &mut Self::Extra<'_>,
    ) {
        self.image = aseprite.atlas_image.clone();
        self.texture_min = slice_meta.rect.min.as_uvec2();
        self.texture_max = slice_meta.rect.max.as_uvec2();
        self.time = extra.elapsed_secs();
    }
}

#[derive(Component)]
struct Cube;

fn setup(
    mut cmd: Commands,
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MyMaterial>>,
) {
    cmd.spawn((
        Camera3d::default(),
        Transform::default()
            .with_translation(vec3(0.0, 3.0, 5.0))
            .looking_at(vec3(0.0, 0.0, 0.0), Dir3::Y),
    ));
    cmd.spawn((
        AseAnimation {
            aseprite: server.load("player.aseprite"),
            animation: Animation::tag("walk-down"),
        },
        Transform {
            translation: vec3(-1.0, 0.0, 0.0),
            ..default()
        },
        Mesh3d(meshes.add(Mesh::from(Cuboid::default()))),
        MeshMaterial3d(materials.add(MyMaterial::default())),
        Cube,
    ));
    cmd.spawn((
        AseSlice {
            name: "ghost_red".into(),
            aseprite: server.load("ghost_slices.aseprite"),
        },
        Mesh3d(meshes.add(Mesh::from(Cuboid::default()))),
        MeshMaterial3d(materials.add(MyMaterial::default())),
        Transform {
            translation: vec3(1.0, 0.0, 0.0),
            ..default()
        },
        SliceCycle {
            current: 0,
            slices: vec!["ghost_red".into(), "ghost_blue".into()],
        },
        Cube,
    ));
}

#[derive(Component)]
pub struct SliceCycle {
    current: usize,
    slices: Vec<String>,
}

fn change_slice(mut slices: Query<(&mut AseSlice, &mut SliceCycle)>) {
    slices.iter_mut().for_each(|(mut slice, mut cycle)| {
        cycle.current += 1;
        let index = cycle.current % cycle.slices.len();
        slice.name = cycle.slices[index].clone();
        info!("slice changed to {}", slice.name);
    });
}

fn rotate_cube(mut q: Query<&mut Transform, With<Cube>>, time: Res<Time>) {
    for mut transform in &mut q {
        transform.rotate(Quat::from_rotation_y(time.delta_secs()));
    }
}
