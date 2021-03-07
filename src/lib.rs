//! A skybox plugin for processing skybox images and projecting them
//! relative to the camera.
//!
//! # Example
//!
//! ```rust
//! use bevy::prelude::*;
//! use bevy_skybox::{SkyboxPlugin, SkyboxCamera};
//!
//! fn setup(commands: &mut Commands) {
//!		commands
//! 		.spawn(Camera3dBundle::default())
//! 		.with(SkyboxCamera);
//! }
//!
//! fn main() {
//!		App::build()
//! 		.add_plugins(DefaultPlugins)
//! 		.add_startup_system(setup.system())
//! 		.add_plugin(SkyboxPlugin::from_image_file("sky1.png"))
//! 		.run();
//! }
//! ```

mod image;

use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;

/// Create a secondary camera with a longer draw distance than the main camera.
fn create_pipeline(
    commands: &mut Commands,
    camera_query: Query<(Entity, &PerspectiveProjection, &SkyboxCamera)>,
    skybox_query: Query<(Entity, &SkyboxBox)>,
    mut active_cameras: ResMut<bevy::render::camera::ActiveCameras>,
    plugin: Res<crate::SkyboxPlugin>,
) {
    // If more than one SkyboxCamera is defined then only one is used.
    if let Some((cam, cam_proj, _)) = camera_query.iter().next() {
        // Add a secondary camera as a child of the main camera but a longer draw distance.
        //
        // Assumes that the perspective projection of the main camera does not change.
        let far_proj = PerspectiveProjection {
            near: cam_proj.far * 1.5,
            far: cam_proj.far * 10.0,
            ..cam_proj.clone()
        };
        let child_entity = commands
            .spawn(Camera3dBundle {
                perspective_projection: far_proj,
                ..Default::default()
            })
            .current_entity()
            .expect("Child camera");
        commands.push_children(cam, &[child_entity]);

        // Make the secondary camera active.
        active_cameras.add(&plugin.camera_name);

        // Assign the skybox to the secondary camera.
        for s in skybox_query.iter() {
            active_cameras.set(&plugin.camera_name, s.0);
        }
    }
}

/// Translate (but don't rotate) the `SkyboxBox` with the camera (or any entity it is attached
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
                // I'm not sure how the scale is working with respect to the draw distances
                // but it does seem to be.
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
    /// The filename of the image in the assets folder.
    pub image: Option<String>,
    /// The identifying name of the secondary camera and pipeline for rendering the skybox
    pub camera_name: String,
}

impl SkyboxPlugin {
    pub fn from_image_file(image: &str) -> SkyboxPlugin {
        Self {
            image: Some(image.to_owned()),
            camera_name: "Skybox".to_owned(),
        }
    }
    /// Does not create an image cube, props must then be added to SkyboxCamera
    /// with a `SkyboxBox` component.
    pub fn empty() -> SkyboxPlugin {
        Self {
            image: None,
            camera_name: "Skybox".to_owned(),
        }
    }
}

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(self.clone());
        app.add_startup_system(image::create_skybox.system());
        app.add_startup_system(create_pipeline.system());
        app.add_system(move_skybox.system());
    }
}
