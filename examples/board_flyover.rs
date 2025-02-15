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
use bevy_flycam::prelude::*;
use bevy_skybox::{SkyboxCamera, SkyboxPlugin};
use rand::Rng;
use std::env;

/// Create the window, add the plugins and set up the entities.
fn main() {
    // Get the skybox image.
    let image = env::args().nth(1).unwrap_or("sky1.png".to_owned());
    // Build the window and app.
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NoCameraPlayerPlugin)
        .add_plugins(SkyboxPlugin::from_image_file(&image))
        .add_systems(Startup, setup)
        .run();
}

/// Set up the camera, skybox and "board" in this example.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // The bevy_flycam::NoCameraPlayerPlugin assumes it'll find one camera.
    commands
        .spawn((
            Camera3d::default(),
            Transform::from_translation(Vec3::new(0.0, 2.0, -4.0))
                 .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            PerspectiveProjection {
                 far: 200.0,
                 ..Default::default()
            },
            SkyboxCamera,
            FlyCam,
        ))
        .with_children(|parent| {
            parent.spawn((
                DirectionalLight {
                    illuminance: 5000.0,
                    shadows_enabled: true,
                    ..default()
                },
                Transform::default().looking_at(Vec3::ZERO, Vec3::Y),
            ));
        });

    // Add a static "board" as some foreground to show camera movement.
    let mut rng = rand::rng();
    for i in -20..=20 {
        for j in -20..=20 {
            // Each square is a random shade of green.
            let br = rng.random::<f32>() * 0.4 + 0.6;
            let col = Color::srgb(0.6 * br, 1. * br, 0.6 * br);
            commands.spawn((
                Mesh3d(meshes.add(Mesh::from(Plane3d {
                    normal: Dir3::Y,
                    half_size: Vec2::new(0.5, 0.5),
                }))),
                MeshMaterial3d(materials.add(col)),
                Transform::from_translation(Vec3::new(i as f32, 0.0, j as f32)),
            ));
        }
    }
}
