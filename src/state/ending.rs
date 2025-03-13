use bevy::{
    prelude::*,
    sprite::MeshMaterial2d,
    color::palettes::basic,
    audio,
};
use super::super::state::*;
#[derive(Component)]
pub struct EndingText;

#[derive(Component)] 
pub struct ClickText;
#[derive(Resource)] 
pub struct MoveText {
    pub up_value: f32,
    pub up_offset: f32,
    pub timer: f32,
}
impl Default for MoveText{
    fn default() -> MoveText{
        MoveText { up_value: 0.0, up_offset: 610.0, timer: 0.0 }
    }
}

pub fn spawn_system(
    mut commands: Commands,
    mut app: ResMut<MyApp>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    app.stage_count = 1;
    app.is_clear = true;

    commands.spawn((
        AudioPlayer::new(asset_server.load(assets::BGMENDING)),
        PlaybackSettings {
            mode: { audio::PlaybackMode::Loop },
            volume: audio::Volume::new(value::VOLUME),
            ..default()
        },
        ReleaseResource
    ));

    commands.spawn((
        Camera2d,
        Camera::default(),
        ReleaseResource
    ));

    commands.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)));
    commands.insert_resource(MoveText::default());
    let time = if app.timer >= 60.0{
        let min = (app.timer / 60.0) as u32;
        let sec = (app.timer - (min as f32 * 60.0)) as u32;
        format!("{} minutes {} seconds",min, sec)
    }else{
        format!("{} seconds", app.timer as u32)
    };
    commands.spawn((
        Text2d::new("thank you for playing!"),
        TextFont {
            font: asset_server.load(assets::DEFAULTFONT),
            font_size: 50.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        EndingText,
        ReleaseResource,
    )).with_children(|parent|{
        parent.spawn((
            Text2d::new(format!("time: {}", time)),
            TextFont {
                font: asset_server.load(assets::DEFAULTFONT),
                font_size: 30.0,
                ..default()
            },
            MeshMaterial2d(materials.add(Color::from(basic::WHITE))),
            Transform::from_translation(Vec3::new(0.0, -60.0, 0.0)),
        ));
        parent.spawn((
            Text2d::new(format!("jump: {}", app.jump_count)),
            TextFont {
                font: asset_server.load(assets::DEFAULTFONT),
                font_size: 30.0,
                ..default()
            },
            MeshMaterial2d(materials.add(Color::from(basic::WHITE))),
            Transform::from_translation(Vec3::new(0.0, -100.0, 0.0)),
        ));
        parent.spawn((
            Text2d::new(format!("music: 魔王魂")),
            TextFont {
                font: asset_server.load(assets::DEFAULTFONT),
                font_size: 20.0,
                ..default()
            },
            MeshMaterial2d(materials.add(Color::from(basic::WHITE))),
            Transform::from_translation(Vec3::new(0.0, -170.0, 0.0)),
        ));
        parent.spawn((
            Text2d::new(format!("sound: FC音工場")),
            TextFont {
                font: asset_server.load(assets::DEFAULTFONT),
                font_size: 20.0,
                ..default()
            },
            MeshMaterial2d(materials.add(Color::from(basic::WHITE))),
            Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
        ));
    });
    commands.spawn((
        Text2d::new("(click to go to next game)"),
        TextFont {
            font: asset_server.load(assets::DEFAULTFONT),
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::srgba(1.0,1.0,1.0,0.0)),
        Transform::from_translation(Vec3::new(0.0, -120.0, 0.0)),
        ClickText,
        ReleaseResource,
    ));
}

pub fn update_debug(
    mut app_state: ResMut<NextState<AppState>>,
    keyboard_input:  Res<ButtonInput<KeyCode>>,
) {
    if !value::ISDEBUG{return;}
    if keyboard_input.just_pressed(KeyCode::F2){
        app_state.set(AppState::Game);
    }
}

pub fn update_player(
    app: Res<MyApp>, 
    mut app_state: ResMut<NextState<AppState>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if mouse_button_input.just_released(MouseButton::Left) && app.is_ending_end{
        app_state.set(AppState::Game);
    }
}

pub fn update_move_text(
    mut app: ResMut<MyApp>, 
    time: Res<Time>, 
    mut query: Query<&mut Transform, With<EndingText>>,
    mut mt: ResMut<MoveText>,
) {
    mt.up_value = mt.up_value + time.delta_secs();
    let mut transform = query.single_mut();
    let mut val = (80.0 * mt.up_value as f32) - mt.up_offset ;
    if val > value::ENDINGTEXTMOVE {val = value::ENDINGTEXTMOVE;}
    transform.translation.y = val;
    if val == value::ENDINGTEXTMOVE {mt.timer += time.delta_secs();}
    if mt.timer > 1.0{
        app.is_ending_end = true;
    }
}

pub fn update_click_text_animation(
    app: Res<MyApp>, 
    time: Res<Time>, 
    mut text_query: Query<&mut TextColor, With<ClickText>>,
) {
    if !app.is_ending_end{return;}
    let mut text = text_query.single_mut();
    let wave_size = 2.0;
	let elapsed = time.elapsed().as_secs_f32();
    let sin_wave = (2.0 * std::f32::consts::PI * elapsed / wave_size).sin() * 0.5 + 0.5;
    text.0 = Color::srgba(1.0, 1.0, 1.0, sin_wave);
}