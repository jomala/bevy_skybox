//! A skybox plugin for processing skybox images and projecting them
//! relative to the camera.
//!
//! # Example
//!
//! ```rust
//! use bevy::prelude::*;
//! use bevy_skybox::{SkyboxCamera, SkyboxPlugin};
//!
//! fn setup(mut commands: Commands) {
//!    commands.spawn((
//!        Camera3d,
//!        SkyboxCamera,
//!    ));}
//!
//! fn main() {
//!    App::build()
//!        .add_plugins(DefaultPlugins)
//!        .add_systems(Startup, setup)
//!        .add_plugin(SkyboxPlugin::from_image_file("sky1.png"))
//!        .run();
//!
//! ```

mod image;
mod material;
pub use material::SkyMaterial;

use bevy::prelude::*;
use bevy::render::view::visibility::RenderLayers;

/// Create a secondary camera with a longer draw distance than the main camera.
fn create_pipeline(
    mut commands: Commands,
    camera_query: Query<(Entity, &SkyboxCamera)>,
    skybox_query: Query<Entity, With<SkyboxBox>>,
) {
    // If more than one SkyboxCamera is defined then only one is used.
    if let Some((cam, _)) = camera_query.iter().next() {
        // Add a secondary camera as a child of the main camera
        let child_entity = commands
            .spawn((Camera3d::default(), RenderLayers::layer(1), SkyboxSecondaryCamera))
            .id();
        commands.entity(cam).add_children(&[child_entity]);

        // Assign the skybox to the secondary camera.
        for s in skybox_query.iter() {
            commands.entity(s).insert(RenderLayers::layer(1));
        }
    }
}

/// Translate (but don't rotate) the `SkyboxBox` with the camera (or any entity it is attached
/// to with a Transform property). If it is not attached to such an
/// entity then it will not move.
fn move_skybox(
    mut skybox_query: Query<(&mut Transform, &SkyboxBox), Without<SkyboxCamera>>,
    camera_query: Query<(&Transform, &SkyboxCamera)>,
) {
    if let Some((cam_trans, _)) = camera_query.iter().next() {
        for (mut pbr_trans, _) in skybox_query.iter_mut() {
            // This could also be achieved by manipulating the ViewProj matrix
            // in the SkyMaterial shader.
            pbr_trans.translation = cam_trans.translation;
            pbr_trans.rotation = Quat::IDENTITY;
        }
    }
}

/// The `SkyboxCamera` tag attached to the camera (Translation) entity that
/// triggers the skybox to move with the camera.
#[derive(Component)]
pub struct SkyboxCamera;

/// The `SkyboxSecondaryCamera` tag attached to the primary camera entity that
/// and gives the infinite view distance for the skybox.
#[derive(Component)]
pub struct SkyboxSecondaryCamera;

/// The `SkyboxBox` tag attached to the skybox mesh entity.
#[derive(Component)]
pub struct SkyboxBox;

/// The `SkyboxPlugin` object acts as both the plugin and the resource providing the image name.
#[derive(Clone, Resource)]
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
    fn build(&self, app: &mut App) {
        app.insert_resource(self.clone())
            .add_systems(Startup, (image::create_skybox, create_pipeline))
            .add_systems(PostUpdate, move_skybox);
    }
}
