#![windows_subsystem = "windows"]
use bevy::{
    prelude::*,
    window::*,
};
use define::value;

mod collision;
mod define;
pub mod block;
pub mod state;

fn main() {
    set_exec();
}

fn set_exec(){
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: define::common::TOOLNAME.into(),
                position: WindowPosition::new(IVec2::new(value::DEFAULTWINDOWPOSX, 0)),
                resolution: (1100.0, 1000.0).into(),
                transparent: true,
                decorations: true,
                present_mode: PresentMode::AutoNoVsync,
                prevent_default_event_handling: false,
                ..default()
            }),
            exit_condition: bevy::window::ExitCondition::OnAllClosed,
            close_when_requested: true,
            ..default()
        }).set(bevy::log::LogPlugin{
            level: bevy::log::Level::WARN,
            ..default()
        }).set(AssetPlugin {
	        meta_check: bevy::asset::AssetMetaCheck::Never,
	        ..default()
	    }))
        .add_plugins(state::StatePlugin)
        
        .run();
}