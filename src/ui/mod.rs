mod main_menu;
mod system_button;
use bevy::prelude::Plugin;
use main_menu::MainMenuPlugin;
use system_button::SystemButtonPlugin;


pub struct UIPlugin;


impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugins((MainMenuPlugin, SystemButtonPlugin));
    }
}