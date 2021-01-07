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

/// Move the `SkyboxBox` with the camera (or any entity it is attached
/// to with a Transform property). If it is not attached to such an
/// entity then it will not move.
fn move_skybox(
    mut skybox_query: Query<(&mut Transform, &SkyboxBox)>,
    camera_query: Query<(&PerspectiveProjection, &Transform, &SkyboxCamera)>,
) {
    if let Some((cam_proj, cam_trans, _)) = camera_query.iter().next() {
        for (mut pbr_trans, _) in skybox_query.iter_mut() {
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
///
/// This can also be used to tag any other `Transform` that you want to translate with
/// the `SkyboxCamera`, e.g. a light source.
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
        app.add_startup_system(image::create_skybox.system());
        app.add_system(move_skybox.system());
    }
}
