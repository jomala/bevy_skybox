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

use bevy::{
    prelude::*,
    core_pipeline::Skybox,
    image::CompressedImageFormats,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
    render::renderer::RenderDevice,
};

/// Create a secondary camera with a longer draw distance than the main camera.
fn create_skybox(
    mut commands: Commands,
    mut plugin: ResMut<SkyboxPlugin>,
    render_device: Res<RenderDevice>,
    mut images: ResMut<Assets<Image>>,
    camera_query: Query<Entity, With<SkyboxCamera>>,
) {
    if let Some(image) = &plugin.image {
        // Check that the uncompressed format is supported.
        assert!(CompressedImageFormats::from_features(render_device.features()).contains(CompressedImageFormats::NONE));

        // Get the skybox image for the image given.
        let mut skybox_image = image::get_skybox(image).expect("Good image");

        assert_eq!(skybox_image.texture_descriptor.array_layer_count(), 1);
        skybox_image.reinterpret_stacked_2d_as_array(6);
        assert_eq!(skybox_image.texture_descriptor.array_layer_count(), 6);

        skybox_image.texture_view_descriptor = Some(TextureViewDescriptor {
            dimension: Some(TextureViewDimension::Cube),
            ..default()
        });

        let skybox_handle = images.add(skybox_image);
        plugin.handle = Some(skybox_handle.clone());

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

fn new_camera(
    mut commands: Commands,
    plugin: Res<SkyboxPlugin>,
    camera_query: Query<Entity, (Added<Camera3d>, With<SkyboxCamera>)>,
) {
    if let Some(skybox_handle) = &plugin.handle {
        for cam in camera_query.iter() {
            println!("Add camera after");
            commands.entity(cam).insert(Skybox {
                image: skybox_handle.clone(),
                brightness: 1000.0,
                ..default()
            });
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
    image: Option<String>,
    handle: Option<Handle<Image>>,
}

impl SkyboxPlugin {
    pub fn from_image_file(image: &str) -> SkyboxPlugin {
        Self {
            image: Some(image.to_owned()),
            handle: None,
        }
    }

    /// Does not create an image cube, props must then be added to SkyboxCamera
    /// with a `Skybox` component.
    pub fn empty() -> SkyboxPlugin {
        Self { image: None, handle: None }
    }
}

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.clone())
            .add_systems(Startup, create_skybox)
            .add_systems(Update, new_camera);
    }
}
