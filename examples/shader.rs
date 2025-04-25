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
        .add_plugins(MaterialAnimationPlugin::<MyAnimation>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, sync_mat_and_update_time)
        .run();
}

// create our own aseprite animation component
#[derive(Default, Component)]
#[require(AnimationState, MyMaterial, Transform)]
pub struct MyAnimation {
    ase: Handle<Aseprite>,
    animation: Animation,
}

impl AseAnimation for MyAnimation {
    type Target = MyMaterial; // each time the frame changes, a system will mut this Component in
                              // the render function below.

    fn aseprite(&self) -> &Handle<Aseprite> {
        &self.ase
    }

    fn animation(&self) -> &Animation {
        &self.animation
    }

    fn animation_mut(&mut self) -> &mut Animation {
        &mut self.animation
    }

    fn render(
        &self,
        target: &mut Self::Target,
        frame: u16,
        aseprite: &Aseprite,
        layout: &TextureAtlasLayout,
    ) {
        let id = aseprite.get_atlas_index(frame as usize);
        let Some(rect) = layout.textures.get(id) else {
            return;
        };
        target.image = aseprite.atlas_image.clone();
        target.texture_min = rect.min;
        target.texture_max = rect.max;
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
        MyAnimation {
            ase: server.load("player.aseprite"),
            animation: Animation::tag("walk-down"),
        },
        Mesh2d(meshes.add(Mesh::from(Rectangle::from_size(Vec2::splat(100.0))))),
        MeshMaterial2d(materials.add(MyMaterial::default())),
    ));
}

fn sync_mat_and_update_time(
    comps: Query<(&MeshMaterial2d<MyMaterial>, &MyMaterial)>,
    mut materials: ResMut<Assets<MyMaterial>>,
    time: Res<Time>,
) {
    comps.iter().for_each(|(h, mat)| {
        if let Some(m) = materials.get_mut(h.id()) {
            m.texture_min = mat.texture_min;
            m.texture_max = mat.texture_max;
            m.time = time.elapsed().as_secs_f32();
            m.image = mat.image.clone();
        }
    });
}
