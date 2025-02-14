//! A skybox plugin for processing skybox images and projecting them
//! relative to the camera.
//!
//! # Example
//!
//! ```rust
//! use bevy::prelude::*;
//! use bevy_skybox::{SkyboxCamera, SkyboxPlugin};
//!
//! fn main() {
//!    App::build()
//!        .add_plugins(DefaultPlugins)
//!        .add_systems(Startup, setup)
//!        .add_plugin(SkyboxPlugin::from_image_file("sky1.png"))
//!        .run();
//!
//! fn setup(mut commands: Commands) {
//!    commands.spawn((
//!        Camera3d,
//!        SkyboxCamera,
//!    ));}
//!
//! ```

mod image;

use bevy::core_pipeline::Skybox;
use bevy::prelude::*;

/// Create a secondary camera with a longer draw distance than the main camera.
fn create_skybox(
    mut commands: Commands,
    plugin: Res<SkyboxPlugin>,
    mut images: ResMut<Assets<Image>>,
    camera_query: Query<Entity, With<SkyboxCamera>>,
) {
    if let Some(image) = &plugin.image {
        // Get the kybox image for the image given.
        let skybox_image = image::get_skybox(image).expect("Good image");
        let skybox_handle = images.add(skybox_image);

        for cam in camera_query.iter() {
            commands.entity(cam).insert(Skybox {
                image: skybox_handle.clone(),
                brightness: 1000.0,
                ..default()
            });
        }
    } else {
        for cam in camera_query.iter() {
            commands.entity(cam).remove::<Skybox>();
        }
    }
}

/// The `SkyboxCamera` tag attached to the camera (Translation) entity that
/// triggers the skybox to move with the camera.
#[derive(Component)]
pub struct SkyboxCamera;

/// The `SkyboxPlugin` object acts as both the plugin and the resource providing the image name.
#[derive(Clone, Resource)]
pub struct SkyboxPlugin {
    /// The filename of the image in the assets folder.
    pub image: Option<String>,
}

impl SkyboxPlugin {
    pub fn from_image_file(image: &str) -> SkyboxPlugin {
        Self {
            image: Some(image.to_owned()),
        }
    }

    /// Does not create an image cube, props must then be added to SkyboxCamera
    /// with a `Skybox` component.
    pub fn empty() -> SkyboxPlugin {
        Self { image: None }
    }
}

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.clone())
            .add_systems(Startup, create_skybox);
    }
}
