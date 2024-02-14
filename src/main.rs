#![cfg_attr(not(release_assertions), windows_subsystem = "windows")]
use bevy::{
    prelude::*,
    window::*,
};
use define::value;

mod collision;
mod define;
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
                fit_canvas_to_parent: true,
                prevent_default_event_handling: true,
                ..default()
            }),
            exit_condition: bevy::window::ExitCondition::OnAllClosed,
            close_when_requested: true,
            ..default()
        }))
        .add_plugins(state::StatePlugin)
        .add_plugins(bevy_framepace::FramepacePlugin)
        .run();
}