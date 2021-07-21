//! A simple example of how to create a `bevy_skybox` and attach it to a camera.
//!
//! An optional positional argument can be used to give the filename of the
//! skybox image to test (in the `assets` folder, including the suffix).
//!
//! The camera has a deliberately limited draw distance (roughly twice the width
//! of the board) to show that the skybox is not affected by it.
//!
//! ```
//! cargo +nightly run --release --example board_flyover -- sky2.png
//! ```
//!
//! The controls are:
//! - W / A / S / D - Move along the horizontal plane
//! - Shift - Move downward
//! - Space - Move upward
//! - Mouse - Look around

use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_skybox::{SkyboxCamera, SkyboxPlugin};
use rand::Rng;
use std::env;

/// Create the window, add the plugins and set up the entities.
fn main() {
    // Get the skybox image.
    let image = env::args().nth(1).unwrap_or("sky1.png".to_owned());
    // Build the window and app.
    App::build()
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::INFO,
            filter: "wgpu=warn,bevy_ecs=info,bevy_skybox=info".to_string(),
        })
        .insert_resource(WindowDescriptor {
            title: "Skybox Board Flyover".to_string(),
            width: 800.,
            height: 800.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyCameraPlugin)
        .add_plugin(SkyboxPlugin::from_image_file(&image))
        .add_startup_system(setup.system())
        .run();
}

/// Set up the camera, skybox and "board" in this example.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add a camera with a the `FlyCamera` controls and a `Skybox` centred on it.
    let cam = PerspectiveCameraBundle {
        transform: Transform::from_matrix(Mat4::from_translation(Vec3::new(0.0, 2.0, -4.0)))
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
        perspective_projection: PerspectiveProjection {
            far: 200.0,
            ..Default::default()
        },
        ..Default::default()
    };

    commands
        .spawn()
        .insert_bundle(cam)
        .insert(FlyCamera::default())
        .insert(SkyboxCamera)
        .with_children(|parent| {
            // Add a light source for the board that moves with the camera.
            parent.spawn()
                .insert_bundle(LightBundle {
                    transform: Transform::from_translation(Vec3::new(0.0, 30.0, 0.0)),
                    ..Default::default()
                });
        });


    // Add a static "board" as some foreground to show camera movement.
    let mut rng = rand::thread_rng();
    for i in -20..=20 {
        for j in -20..=20 {
            // Each square is a random shade of green.
            let br = rng.gen::<f32>() * 0.4 + 0.6;
            let col = Color::rgb(0.6 * br, 1. * br, 0.6 * br);
            commands.spawn()
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
                    material: materials.add(col.into()),
                    transform: Transform::from_translation(Vec3::new(i as f32, 0.0, j as f32)),
                    ..Default::default()
                });
        }
    }
}
