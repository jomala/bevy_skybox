use bevy::prelude::*;
use bevy_skybox::{SkyboxPlugin, SkyboxCamera};

fn setup(mut commands: Commands) {
	commands
		.spawn()
		.insert_bundle(PerspectiveCameraBundle::default())
		.insert(SkyboxCamera);
}

fn main() {
	App::build()
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup.system())
		.add_plugin(SkyboxPlugin::from_image_file("sky1.png"))
		.run();
}
