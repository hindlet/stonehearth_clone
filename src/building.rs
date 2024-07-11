use bevy::prelude::*;


pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        
    }
}




#[derive(Bundle)]
pub struct BuildingBundle {
    tag: BuildingTag
}

impl Default for BuildingBundle {
    fn default() -> Self {
        BuildingBundle {
            tag: BuildingTag
        }
    }
}

#[derive(Component)]
pub struct BuildingTag;


#[derive(Bundle)]
pub struct BuildingComponentBundle {
    tag: BuildingComponentTag
}

impl Default for BuildingComponentBundle {
    fn default() -> Self {
        BuildingComponentBundle {
            tag: BuildingComponentTag
        }
    }
}

#[derive(Component)]
pub struct BuildingComponentTag;