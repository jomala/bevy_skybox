//! A very simple example of a skybox and a very wide FOV (and nothing else).
use bevy::prelude::*;
use bevy_skybox::{SkyboxCamera, SkyboxPlugin};

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            // 120 degree field-of-view.
            fov: 120.0_f32.to_radians(),
            ..default()
        }),
        SkyboxCamera,
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(SkyboxPlugin::from_image_file("sky1.png"))
        .run();
}
