use bevy::prelude::*;
use bevy_skybox::{SkyboxCamera, SkyboxPlugin};

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d,
        SkyboxCamera,
    ));}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugin(SkyboxPlugin::from_image_file("sky1.png"))
        .run();
}
