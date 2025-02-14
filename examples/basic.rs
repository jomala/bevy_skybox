use bevy::prelude::*;
use bevy_skybox::{SkyboxCamera, SkyboxPlugin};

fn setup(mut commands: Commands) {
    commands.spawn((Camera3d::default(), SkyboxCamera));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(SkyboxPlugin::from_image_file("sky1.png"))
        .run();
    println!("End");
}
