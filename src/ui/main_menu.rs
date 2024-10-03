use bevy::prelude::*;

use crate::AppState;

use super::system_button::{SystemButton, SystemButtonBundle};



pub enum MainMenuState {
    Main,
    Settings,
    Credits
}

pub struct MainMenuPlugin;


impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (spawn_menus, hide_main_menu).chain())
            .add_systems(OnEnter(AppState::MainMenu), show_main_menu)
            .add_systems(OnExit(AppState::MainMenu), hide_main_menu);
    }
}


fn show_main_menu(
    mut main_menu_query: Query<&mut Visibility, With<MainMenuUITag>>
) {
    let mut visibility = main_menu_query.single_mut();
    *visibility = Visibility::Visible;
}

fn hide_main_menu(
    mut main_menu_query: Query<&mut Visibility, With<MainMenuUITag>>
) {
    let mut visibility = main_menu_query.single_mut();
    *visibility = Visibility::Hidden;
}



#[derive(Component)]
struct MainMenuUITag;

struct SettingsMenuUITag;



fn spawn_menus(
    world: &mut World
) {
    let play_func_id = world.register_system(play_button_system);

    world.spawn(NodeBundle{
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn(SystemButtonBundle {
            system_button: SystemButton(play_func_id),
            button: ButtonBundle {
                style: Style {
                    width: Val::Percent(20.0),
                    height: Val::Percent(15.0),
                    ..Default::default()
                },
                background_color: Color::srgb(0.0, 0.6, 1.0).into(),
                ..Default::default()
            }
        });
    })
    .insert(MainMenuUITag);
}


fn play_button_system(
    mut main_menu_vis: Query<&mut Visibility, With<MainMenuUITag>>,
    mut next_state: ResMut<NextState<AppState>>
) {

    next_state.set(AppState::GameLoading);
    println!("Play button pressed")
}