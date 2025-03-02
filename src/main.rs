use bevy::prelude::*;
use resources::ResourcesPlugin;
use tiles::TilesPlugin;

mod resources;
mod tiles;
mod components;
mod systems;

fn main() {
    App::new()
      .add_plugins(DefaultPlugins)
      .add_plugins(ResourcesPlugin)
      .add_plugins(TilesPlugin)
    ;
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum GameState {
    TitleScreen,
    Game,
    Result
}