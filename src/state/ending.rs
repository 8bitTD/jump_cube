use bevy::prelude::*;
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
) {
    app.stage_count = 1;

    commands.spawn((AudioBundle {
        source: asset_server.load(assets::BGMENDING),
        settings: PlaybackSettings{
            mode: bevy::audio::PlaybackMode::Loop,
            volume: bevy::audio::Volume::Relative(bevy::audio::VolumeLevel::new(value::VOLUME)),
            ..default()
        },
        ..default()
        },
        ReleaseResource
    ));

    commands.spawn((Camera2dBundle::default(), ReleaseResource));
    let font = asset_server.load(assets::DEFAULTFONT);
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands.insert_resource(MoveText::default());
    let time = if app.timer >= 60.0{
        let min = (app.timer / 60.0) as u32;
        let sec = (app.timer - (min as f32 * 60.0)) as u32;
        format!("{} minutes {} seconds",min, sec)
    }else{
        format!("{} seconds", app.timer as u32)
    };
    
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("thank you for playing!", text_style.clone()),
            ..default()
        },
        EndingText,
        ReleaseResource,
    )).with_children(|parent|{
        parent.spawn(
            Text2dBundle {
                text: Text::from_section(format!("time: {}", time), TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                }),
                transform: Transform{
                    translation: Vec3::new(0.0, -60.0, 0.0),
                    ..default()
                },
                ..default()
            },
        );
        parent.spawn(
            Text2dBundle {
                text: Text::from_section(format!("jump: {}", app.jump_count), TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                }),
                transform: Transform{
                    translation: Vec3::new(0.0, -100.0, 0.0),
                    ..default()
                },
                ..default()
            },
        );
        parent.spawn(
            Text2dBundle {
                text: Text::from_section(format!("music: 魔王魂"), TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                }),
                transform: Transform{
                    translation: Vec3::new(0.0, -170.0, 0.0),
                    ..default()
                },
                ..default()
            },
        );
        parent.spawn(
            Text2dBundle {
                text: Text::from_section(format!("sound: FC音工場"), TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                }),
                transform: Transform{
                    translation: Vec3::new(0.0, -200.0, 0.0),
                    ..default()
                },
                ..default()
            },
        );
    });
    commands.spawn((
        Text2dBundle {
            text: Text { 
                sections: vec![TextSection{
                    value: "(click to go to next game)".into(),
                    style: TextStyle { 
                        font: font.clone(),
                        font_size: 40.0,
                        color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                    },
                }],
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -120.0, 0.0)),
            ..default()
        },
        ClickText,
        ReleaseResource,
    ));
}

pub fn update_debug(
    mut app_state: ResMut<NextState<AppState>>,
    keyboard_input:  Res<Input<KeyCode>>,
) {
    if !value::ISDEBUG{return;}
    if keyboard_input.just_pressed(KeyCode::F2){
        app_state.set(AppState::Game);
    }
}

pub fn update_player(
    app: Res<MyApp>, 
    mut app_state: ResMut<NextState<AppState>>,
    mouse_button_input: Res<Input<MouseButton>>,
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
    mt.up_value = mt.up_value + time.delta_seconds();
    let mut transform = query.single_mut();
    let mut val = (80.0 * mt.up_value as f32) - mt.up_offset ;
    if val > value::ENDINGTEXTMOVE {val = value::ENDINGTEXTMOVE;}
    transform.translation.y = val;
    if val == value::ENDINGTEXTMOVE {mt.timer += time.delta_seconds();}
    if mt.timer > 1.0{
        app.is_ending_end = true;
    }
}

pub fn update_click_text_animation(
    app: Res<MyApp>, 
    time: Res<Time>, 
    mut text_query: Query<&mut Text, With<ClickText>>,
) {
    if !app.is_ending_end{return;}
    let mut text = text_query.single_mut();
    let wave_size = 2.0;
	let elapsed = time.elapsed().as_secs_f32();
    let sin_wave = (2.0 * std::f32::consts::PI * elapsed / wave_size).sin() * 0.5 + 0.5;
    text.sections[0].style.color = Color::rgba(1.0, 1.0, 1.0, sin_wave);
}