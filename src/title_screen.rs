use bevy::{ecs::system::SystemId, prelude::*};

use crate::{constants::*, GameState};

pub struct TitleScreenPlugin;
impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app
          .add_systems(OnEnter(GameState::TitleScreen), setup_title_screen)
          .add_systems(OnExit(GameState::TitleScreen), clean_title_screen)
          .add_systems(Update, (update_title_screen).run_if(in_state(GameState::TitleScreen)))
        ;
    }
}

#[derive(Resource)]
struct TitleScreenUI(Entity);

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct QuitButton;

fn setup_title_screen(mut commands: Commands) {
    let entity = commands.spawn((
        Node{
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        BackgroundColor(MAIN_COLOR)
    ))
        .with_child(
            
            
            (
                Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                BackgroundColor(SCND_COLOR)
            )).with_children(|parent| {


                parent.spawn((
                    Text::new(GAME_NAME),
                    TextColor(TITLE_COLOR),
                    TextFont {
                        font_size: TITLE_FONT_SIZE,
                        ..Default::default()
                    },
                    Node {
                        margin: UiRect::all(Val::Px(3.)),
                        ..Default::default()
                    }
                ));


                parent.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                })
                    .with_children(|parent| {

                        
                        
                        parent.spawn((
                            PlayButton,
                            Button,
                            Node {
                                margin: UiRect::all(Val::Px(1.)),
                                align_content: AlignContent::Center,
                                border: UiRect::all(Val::Px(4.)),
                                ..Default::default()
                            },
                            BorderColor(TRIM_COLOR),
                            BorderRadius::all(Val::Px(5.))
                        ))
                            .with_child((
                                Text::new("Play"),
                                TextColor(TEXT1_COLOR),
                            ));
                    
                    
                        parent.spawn((
                            QuitButton,
                            Button,
                            Node {
                                margin: UiRect::all(Val::Px(1.)),
                                align_content: AlignContent::Center,
                                border: UiRect::all(Val::Px(4.)),
                                ..Default::default()
                            },
                            BorderColor(TRIM_COLOR),
                            BorderRadius::all(Val::Px(5.))
                        ))
                            .with_child((
                                Text::new("Quit"),
                                TextColor(TEXT1_COLOR),
                            ));
            });
            }).id()
            ;
    
    commands.insert_resource(TitleScreenUI(entity));
    commands.init_resource::<TitleScreenOneShots>();
}

fn clean_title_screen(mut commands: Commands,ui: Res<TitleScreenUI>) {
    commands.get_entity(ui.0).unwrap().despawn();
    commands.remove_resource::<TitleScreenUI>();
}

fn update_title_screen(
    mut commands: Commands,
    one_shots: Res<TitleScreenOneShots>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&PlayButton>),
        Changed<Interaction>
    >,
) {
    for (interaction, mut bgcolor, is_play) in button_query.iter_mut() {
        
        bgcolor.0 = match *interaction {
            Interaction::Pressed => {
                if is_play.is_some() {
                    commands.run_system(one_shots.go_game);
                } else {
                    commands.run_system(one_shots.quit);

                }
                BUTTON_PRESSED_COLOR
            },
            Interaction::Hovered => BUTTON_HOWER_COLOR,
            Interaction::None => BUTTON_NONE_COLOR,
        };
    }
}





#[derive(Resource)]
struct TitleScreenOneShots {
    go_game: SystemId,
    quit: SystemId
}
impl FromWorld for TitleScreenOneShots {
    fn from_world(world: &mut World) -> Self {
        Self {
            go_game: world.register_system(go_to_game_screen),
            quit: world.register_system(quit_game)
        }
    }
}

fn go_to_game_screen(mut next: ResMut<NextState<GameState>>) {
    next.set(GameState::Game);
}

fn quit_game(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
}