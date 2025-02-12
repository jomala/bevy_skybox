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
use bevy::render::{
    camera::PerspectiveProjection, pipeline::PipelineDescriptor, render_graph::RenderGraph,
};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_skybox::{SkyMaterial, SkyboxBox, SkyboxCamera, SkyboxPlugin};
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
        .add_systems(Startup, setup)
        .run();
}

/// Set up the camera, skybox props and "board" in this example.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    pipelines: ResMut<Assets<PipelineDescriptor>>,
    shaders: ResMut<Assets<Shader>>,
    render_graph: ResMut<RenderGraph>,
    mut sky_materials: ResMut<Assets<bevy_skybox::SkyMaterial>>,
) {
    // Add a camera with a the `FlyCamera` controls and a `Skybox` centred on it.
    commands
        .spawn((
            Camera3d,
            Transform::from_matrix(Mat4::from_translation(Vec3::new(0.0, 2.0, -4.0)))
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
            PerspectiveProjection {
                far: 200.0,
                ..Default::default()
            },
            FlyCamera::default(),
            SkyboxCamera,
        ))
        .with_children(|parent| {
            // Add a light source for the board that moves with the camera.
            parent.spawn(LightBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 30.0, 0.0)),
                ..Default::default()
            });
        });

    commands
        .spawn((
            PbrBundle::default(),
            SkyboxBox,
        ))
        .with_children(|parent| {
            let render_pipelines = SkyMaterial::pipeline(pipelines, shaders, render_graph);
            let texture_handle: Handle<Texture> = asset_server.load("bevy_logo_dark.png");
            let sky_material = sky_materials.add(SkyMaterial {
                texture: texture_handle,
            });
            // The `parent`'s transform will be manipulated by the plugin
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad {
                        flip: false,
                        size: Vec2::new(20., 7.),
                    })),
                    render_pipelines: render_pipelines.clone(),
                    // Props should be positioned near to a radius of 1.0
                    transform: Transform::from_translation(Vec3::new(-5., 0.1, -20.0)),
                    ..Default::default()
                },
                sky_material.clone(),
            ));
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 3. })),
                    render_pipelines: render_pipelines.clone(),
                    transform: Transform::from_translation(Vec3::new(5.2, 3.15, -15.0)),
                    ..Default::default()
                },
                sky_material,
            ));
        });

    // Add a static "board" as some foreground to show camera movement.
    let mut rng = rand::thread_rng();
    for i in -20..=20 {
        for j in -20..=20 {
            // Each square is a random shade of green.
            let br = rng.gen::<f32>() * 0.4 + 0.6;
            let col = Color::rgb(0.6 * br, 1. * br, 0.6 * br);

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
                    material: materials.add(col.into()),
                    transform: Transform::from_translation(Vec3::new(i as f32, 0.0, j as f32)),
                    ..Default::default()
                },
            ));
        }
    }
}
