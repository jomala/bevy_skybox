//! A simple example of rendering any geometry props as a skybox.
//!
//! The prop geometry translation should be near radius 1.0, or it will
//! clip into regular geometry.
//!
//! ```
//! cargo +nightly run --release --example props
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
use bevy_skybox::{SkyboxBox, SkyboxCamera, SkyboxPlugin};
use rand::Rng;

/// Create the window, add the plugins and set up the entities.
fn main() {
    // Build the window and app.
    App::build()
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::TRACE,
            filter: "wgpu=warn,bevy_ecs=info,bevy_skybox=info".to_string(),
        })
        .insert_resource(WindowDescriptor {
            title: "Prop Skybox Board Flyover".to_string(),
            width: 800.,
            height: 800.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyCameraPlugin)
        .add_plugin(SkyboxPlugin::empty())
        .add_startup_system(setup.system())
        .run();
}

/// Set up the camera, skybox props and "board" in this example.
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

    commands
        .spawn()
        .insert_bundle(PbrBundle {
                ..Default::default()
            })
        .insert(SkyboxBox)
        .with_children(|parent| {
            // The `parent`'s transform will be manipulated by the plugin
            parent.spawn()
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 0.05,
                        subdivisions: 4,
                    })),
                    material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
                    // Props should be positioned near to a radius of 1.0
                    transform: Transform::from_translation(Vec3::new(0.02, 0.005, -1.0)),
                    ..Default::default()
                });
            parent.spawn()
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 0.015,
                        subdivisions: 3,
                    })),
                    material: materials.add(Color::rgb(1.0, 0.9, 0.9).into()),
                    transform: Transform::from_translation(Vec3::new(0.06, 0.04, -0.95)),
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
