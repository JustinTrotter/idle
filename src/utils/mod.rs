use bevy::prelude::*;
use dexterous_developer::{ReloadableElementsSetup, dexterous_developer_setup, ReloadableAppContents, ReloadableApp};

pub mod fps_counter;

use crate::{utils::fps_counter::*, GameState};

pub struct FpsCounterPlugin;


impl Plugin for FpsCounterPlugin {
    fn build(&self, app: &mut App) {
        app
            .setup_reloadable_elements::<reloadable>();
    }
}

#[dexterous_developer_setup]
fn reloadable(app: &mut ReloadableAppContents) {
    app
        .add_systems(OnEnter(GameState::Playing), setup_fps_counter)
            .add_systems(
                Update,
                (
                    fps_text_update_system.run_if(in_state(GameState::Playing)),
                    fps_counter_showhide.run_if(in_state(GameState::Playing)),
                ),
            );
}

