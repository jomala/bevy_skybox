//! A skybox plugin for processing skybox images and projecting them
//! relative to the camera.
//!
//! # Example
//!
//! ```rust
//! use bevy::prelude::*;
//! use bevy_skybox::{SkyboxPlugin, SkyboxCamera};
//!
//! fn setup(mut commands: Commands) {
//!		commands
//! 		.spawn(Camera3dComponents::default())
//! 		.with(SkyboxCamera);
//! }
//!
//! fn main() {
//!		App::build()
//! 		.add_plugins(DefaultPlugins)
//! 		.add_startup_system(setup.system())
//! 		.add_plugin(SkyboxPlugin::from_image_file("sky.png"))
//! 		.run();
//! }
//! ```

mod image;

use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;

/// Create the `SkyboxBox` using settings from the `SkyboxPlugin`.
fn create_skybox(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    plugin: Res<SkyboxPlugin>,
) {
    // Get the mesh for the image given.
    let mesh = image::get_mesh(&plugin.image).expect("Good image");
    // Load image as a texture asset.
    let texture_handle = asset_server.load(plugin.image.as_str());
    // Create the PbrBundle tagged as a skybox.
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .with(SkyboxBox);
}

/// Move the `SkyboxBox` with the camera (or any entity it is attached
/// to with a Transform property). If it is not attached to such an
/// entity then it will not move.
fn move_skybox(
    mut skybox_query: Query<(&mut Transform, &SkyboxBox)>,
    camera_query: Query<(&PerspectiveProjection, &Transform, &SkyboxCamera)>,
) {
    if let Some((mut pbr_trans, _)) = skybox_query.iter_mut().next() {
        // TODO better handling
        for (cam_proj, cam_trans, _) in camera_query.iter() {
            *pbr_trans = Transform {
                translation: cam_trans.translation,
                rotation: Quat::identity(),
                scale: Vec3::new(cam_proj.far, cam_proj.far, cam_proj.far),
            };
        }
    }
}

/// The `SkyboxCamera` tag attached to the camera (Translation) entity that
/// triggers the skybox to move with the camera.
pub struct SkyboxCamera;

/// The `SkyboxBox` tag attached to the skybox mesh entity.
pub struct SkyboxBox;

/// The `SkyboxPlugin` object acts as both the plugin and the resource providing the image name.
#[derive(Clone)]
pub struct SkyboxPlugin {
    image: String,
}

impl SkyboxPlugin {
    pub fn from_image_file(image: &str) -> SkyboxPlugin {
        Self {
            image: image.to_owned(),
        }
    }
}

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(self.clone());
        app.add_startup_system(create_skybox.system());
        app.add_system(move_skybox.system());
    }
}
