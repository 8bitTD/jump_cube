use bevy::prelude::*;
use super::define::*;
use super::collision;
pub mod game;
pub mod ending;
#[derive(Debug, Resource, Default, Copy, Clone)] 
pub struct Pos{
    pub x: f32,
    pub y: f32,
}

#[derive(Resource)] 
pub struct MyApp{
    pub is_ground: bool,
    pub is_rising: bool,
    pub is_jump: bool,
    pub is_block_hit: bool,
    pub side_hit_sound_interval: f32,
    pub is_reset_game: bool,
    pub stage_count: u32,
    pub jump_count: usize,
    pub timer: f32,
    pub text_stage_alpha: f32,
    pub player_pos: Pos,
    pub mouse_pos: Pos,
    pub is_ending_end: bool,
    pub old_velocity_y: f32,
}
impl Default for MyApp{
    fn default() -> Self{   
        MyApp{
            is_ground: false,
            is_rising: true,
            is_jump: true,
            is_block_hit: false,
            side_hit_sound_interval: 0.0,
            is_reset_game: false,
            stage_count: 1,
            jump_count: 0,
            timer: 0.0,
            text_stage_alpha: value::DEFAULTTEXTSTAGEALPHA,
            player_pos: Pos::default(),
            mouse_pos: Pos::default(),
            is_ending_end: false,
            old_velocity_y: 0.0,
        }
    }
}

#[derive(Resource)] 
pub struct OneSecondTimer(pub Timer);

#[derive(Component)]
pub struct ReleaseResource;//リソース開放用

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState{
    #[default]
    Game,
    Ending,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build (&self, app: &mut App){
        app
        .add_state::<AppState>()
        .insert_resource(MyApp::default())
        .insert_resource(OneSecondTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_event::<game::JumpEvent>()
        .add_event::<game::LandingEvent>()
        .add_event::<game::SideLandingEvent>()
        .add_systems(OnEnter(AppState::Game), game::setup_asset)
        .add_systems(Update, 
            (
                game::update_player,
                game::update_debug,
                game::update_fade_stage_text,
                game::update_check_out_of_range,
                game::update_goal_animation,

                game::update_check_for_collisions,
                game::update_apply_velocity,

                game::update_play_sound,
                game::update_reset_game,  
                game::update_check_goal,
                game::update_camera_move,
            ).chain().run_if(in_state(AppState::Game)),
        )
        .add_systems(OnExit(AppState::Game), despawn)
        
        .add_systems(OnEnter(AppState::Ending), ending::spawn_system)
        .add_systems(Update, 
            (
                ending::update_debug,
                ending::update_player,
                ending::update_move_text,
                ending::update_click_text_animation,
            ).run_if(in_state(AppState::Ending)),
        )
        .add_systems(OnExit(AppState::Ending), despawn);

    }
}   

pub fn despawn(
    mut commands: Commands, 
    query: Query<Entity, With<ReleaseResource>>,
){
    for entity in &mut query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}


