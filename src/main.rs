use bevy::prelude::*;
use resources::ResourcesPlugin;
use game::tiles::TilesPlugin;
use title_screen::TitleScreenPlugin;

#[allow(unused)]
mod resources;
#[allow(unused)]
mod game;
#[allow(unused)]
mod title_screen;
#[allow(unused)]
mod systems;
#[allow(unused)]
mod constants;
fn main() {
    App::new()
      .add_plugins(DefaultPlugins)
      .add_plugins(ResourcesPlugin)
      .add_plugins(TilesPlugin)
      .add_plugins(TitleScreenPlugin)
      .init_state::<GameState>()
      .add_systems(Startup, setup)
      .run()
    ;
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub enum GameState {
    #[default]
    TitleScreen,
    Game,
    Result
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2d::default());
}