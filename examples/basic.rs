use bevy::prelude::*;
use bevy_skybox::{SkyboxPlugin, SkyboxCamera};

fn setup(commands: &mut Commands) {
	commands
		.spawn(Camera3dBundle::default())
		.with(SkyboxCamera);
}

fn main() {
	App::build()
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup.system())
		.add_plugin(SkyboxPlugin::from_image_file("sky1.png"))
		.run();
}
