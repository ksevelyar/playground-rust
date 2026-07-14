use bevy::prelude::*;
use bevy_3d::game_plugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, game_plugin))
        .run();
}
