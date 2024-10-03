use bevy::{ecs::system::SystemId, prelude::*};


pub struct SystemButtonPlugin;

impl Plugin for SystemButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, handle_system_buttons);
    }
}


#[derive(Component)]
pub struct SystemButton(pub SystemId);

#[derive(Bundle)]
pub struct SystemButtonBundle{
    pub system_button: SystemButton,
    pub button: ButtonBundle
}


fn handle_system_buttons(
    mut commands: Commands,
    button_query: Query<(&SystemButton, &Interaction), (Changed<Interaction>, With<Button>)>
) {
    for (system_id, interaction) in button_query.iter() {
        match interaction {
            Interaction::Pressed => {
                commands.run_system(system_id.0);
            },
            _ => ()
        }
    }
}