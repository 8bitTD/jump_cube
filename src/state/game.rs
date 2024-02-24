use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
};

use super::collision;
use super::super::state::*;
use super::super::block::*;

#[derive(Component)]
pub struct CameraMarker;

#[derive(Component)]
pub struct BGBlock{
    pub count: i32,
}

#[derive(Component)]
pub struct BGText{
    pub count: i32,
}

#[derive(Component)]
pub struct GoalBlock;

#[derive(Component, Copy, Clone)]
pub struct GoalMarker{pub color: Color }

#[derive(Component)]
pub struct PlayerBlock;

#[derive(Component)]
pub struct Enemy;

#[derive(Debug, Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Debug, Component, Deref, DerefMut)]
pub struct Adjustment(Vec2);

#[derive(Debug, Component)]
pub struct PlayerInfo{
    pub is_ground: bool,
    pub is_rising: bool,
    pub is_block_hit: bool,
    pub is_bend: bool,
    pub angle: f32,
}
impl Default for PlayerInfo{
    fn default() -> PlayerInfo{
        PlayerInfo { is_ground: false, is_rising: false, is_block_hit: false, is_bend: false, angle: 0.0 }
    }
}

#[derive(Component)]
pub struct PlayerText{
    pub count: i32,
}

#[derive(Component)]
pub struct BlackRectangle;

#[derive(Component)]
pub struct StageText;

#[derive(Component)]
pub struct StageTextSub;

#[derive(Component)]
pub struct GoalText;

#[derive(Event, Default)]
pub struct JumpEvent;
#[derive(Resource)]
pub struct JumpSound(Handle<AudioSource>);

#[derive(Event, Default)]
pub struct LandingEvent;
#[derive(Resource)]
pub struct LandingSound(Handle<AudioSource>);

#[derive(Event, Default)]
pub struct SideLandingEvent;
#[derive(Resource)]
pub struct SideLandingSound(Handle<AudioSource>);

#[derive(Event, Default)]
pub struct GetNumberEvent;
#[derive(Resource)]
pub struct GetNumberSound(Handle<AudioSource>);


pub fn update_camera_move(
    mut app: ResMut<MyApp>, 
    mut camera_query: Query<(&Camera, &mut Transform, &GlobalTransform), With<CameraMarker>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    //if app.game_state != GameState::Play{return;}
    let (camera, mut camera_transform, camera_global_transform) = camera_query.single_mut();
    if app.game_state == GameState::In{
        camera_transform.translation.x = app.goal_pos.x;
        camera_transform.translation.y = app.goal_pos.y;
    }else{
        if app.is_reset_game{
            camera_transform.translation.x = value::DEFAULTCAMERAPOSX;
            camera_transform.translation.y = value::DEFAULTCAMERAPOSY;
        }
        camera_transform.translation.x += (app.player_pos.x - camera_transform.translation.x) * 0.075 * (time.delta_seconds() / value::PER60FPS);
        camera_transform.translation.y += (app.player_pos.y - camera_transform.translation.y) * 0.075 * (time.delta_seconds() / value::PER60FPS);
    }
    
    
    let window = q_window.single();
    if  window.cursor_position().is_none(){return;}
    let wcp = window.cursor_position().unwrap();
    let res = camera.viewport_to_world(camera_global_transform, wcp).map(|ray| ray.origin.truncate());
    if res.is_none(){ return; }
    app.mouse_pos.x = res.unwrap().x;
    app.mouse_pos.y = res.unwrap().y;
}

pub fn setup_asset(
    mut app: ResMut<MyApp>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
    asset_server: Res<AssetServer>,
) {
    let tmp_is_clear = app.is_clear;
    if app.stage_count == 1{ *app = MyApp::default(); }
    app.is_clear = tmp_is_clear;
    commands.insert_resource(ClearColor(Color::rgb(0.15, 0.15, 0.15)));
    settings.limiter = bevy_framepace::Limiter::Off;
    let mut cam = Camera2dBundle::default();
    cam.transform = Transform::from_xyz(500.0, 0.0, 1.0);
    commands.spawn((cam, ReleaseResource, CameraMarker));
    
    commands.spawn((AudioBundle {
        source: asset_server.load(assets::BGM),
        settings: PlaybackSettings{
            mode: bevy::audio::PlaybackMode::Loop,
            volume: bevy::audio::Volume::new(value::VOLUME),
            ..default()
        },
        ..default()
        },
        ReleaseResource
    ));

    commands.insert_resource(JumpSound(asset_server.load(assets::SOUNDJUMP)));
    commands.insert_resource(LandingSound(asset_server.load(assets::SOUNDLANDING)));
    commands.insert_resource(SideLandingSound(asset_server.load(assets::SOUNDSIDELANDING)));
    commands.insert_resource(GetNumberSound(asset_server.load(assets::SOUNDGETNUMBER)));

    let font = asset_server.load(assets::DEFAULTFONT);
    let text = match app.stage_count == value::MAXSTAGE{
        true => {"Last Stage".into()},
        _ => {format!("Stage {}",app.stage_count)},
    };

    commands.spawn((
        TextBundle::from_section(
            text,
            TextStyle {
                font: font.clone(),
                font_size: 100.0,
                ..default()
            },
        )        
        .with_style(Style {
            position_type: PositionType::Relative,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            top: Val::Px(-150.0),
            ..default()
        }),
        StageText,
        ReleaseResource,
    ));

    commands.spawn((
        TextBundle::from_section(
            format!("(Total {} Stages)", value::MAXSTAGE),
            TextStyle {
                font: font.clone(),
                font_size: 50.0,
                ..default()
            },
        )        
        .with_style(Style {
            position_type: PositionType::Relative,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            top: Val::Px(-75.0),
            //right: Val::Px(5.0),
            ..default()
        }),
        StageText,
        ReleaseResource,
    ));

    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_translation(Vec3::new(500.0,0.0,1000.0)).with_scale(Vec3::splat(20000.0)),
        material: materials.add(Color::BLACK),
        visibility: Visibility::Visible,
        ..default()
        },
        BlackRectangle,
        ReleaseResource,
    ));

    let number_visibility = match app.is_clear{
        true => { Visibility::Visible },
        _ =>    { Visibility::Hidden },
    };

    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_translation(Vec3::new(value::DEFAULTPOSX,value::DEFAULTPOSY,5.0)).with_scale(Vec3::splat(20.0)),
        material: materials.add(Color::BLACK),
        visibility: Visibility::Visible,
        ..default()
        },
        PlayerBlock,
        ReleaseResource,
        PlayerInfo::default(),
        Velocity(Vec2::new(0.0, 0.0)),
        Adjustment(Vec2::new(0.0, 0.0)),
    )).with_children(|parent| {
        
        parent.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: Transform::default().with_translation(Vec3::new(0.0,0.0,1.0)).with_scale(Vec3::splat(0.9)),
            material: materials.add(Color::ORANGE),
            ..default()
        });
        parent.spawn((Text2dBundle {
            text: Text::from_section("0", TextStyle {
                font: font.clone(),
                font_size: 100.0,
                color: Color::BLACK,
            }),
            transform: Transform::default().with_translation(Vec3::new(0.0,0.05,10.0)).with_scale(Vec3::splat(0.0080)),
            visibility: number_visibility,
            ..default()
            },
            PlayerText{count: 0},
        ));

        if !app.is_clear{
            parent.spawn(MaterialMesh2dBundle {//左目
                mesh: meshes.add(Circle::default()).into(),
                transform: Transform::default().with_translation(Vec3::new(-0.2,0.15,2.0)).with_scale(Vec3::splat(0.15)),
                material: materials.add(Color::BLACK),
                ..default()
            });    
            parent.spawn(MaterialMesh2dBundle {//右目
                mesh: meshes.add(Circle::default()).into(),
                transform: Transform::default().with_translation(Vec3::new(0.2,0.15,2.0)).with_scale(Vec3::splat(0.15)),
                material: materials.add(Color::BLACK),
                ..default()
            });
            parent.spawn(MaterialMesh2dBundle {//口
                mesh: meshes.add(Triangle2d::new(
                    Vec2::Y * 0.25,
                    Vec2::new(-0.5, -0.5),
                    Vec2::new(0.5, -0.5),
                )).into(),
                transform: Transform::default().with_translation(Vec3::new(0.0,-0.15,2.0)).with_scale(Vec3::splat(0.3)),
                material: materials.add(Color::BLACK),
                ..default()
            }); 
        }
    });

    create_block(app, commands, meshes, materials, asset_server);
}

pub fn update_fade_stage_text(
    mut app: ResMut<MyApp>, 
    time: Res<Time>,
    mut text_query: Query<&mut Text, (With<StageText>, Without<PlayerText>, Without<BGText>, Without<GoalText>)>,
){
    if app.text_stage_alpha <= -1.0{return;}
    for mut t in text_query.iter_mut(){
        if app.text_stage_alpha == value::DEFAULTTEXTSTAGEALPHA && t.sections[0].style.font_size == 100.0{
            t.sections[0].value =  match app.stage_count == value::MAXSTAGE {
                true => {"Last Stage".into()},
                _ => {format!("Stage {}",app.stage_count)},
            };
        }
        let alpha = match app.text_stage_alpha > 1.0{
            true => 1.0,
            _ => app.text_stage_alpha,
        };
        t.sections[0].style.color = Color::rgba(1.0,1.0,1.0, alpha);
    }
    app.text_stage_alpha -= time.delta_seconds();
}

pub fn update_goal_animation(
    mut text_query: Query<(&mut Text, &mut Transform), With<GoalText>>,
    player_text_query: Query<&PlayerText>,
    mut goal_query: Query<&mut Handle<ColorMaterial>, With<GoalBlock>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>
) {
    let elapsed = time.elapsed().as_secs_f32();
    let mut goal_material = goal_query.single_mut();
    let r_wave = (2.0 * std::f32::consts::PI * elapsed  / 1.24 as f32).sin() + 0.8;
    let g_wave = (2.0 * std::f32::consts::PI * elapsed  / 0.77 as f32).sin() + 0.8;
    let b_wave = (2.0 * std::f32::consts::PI * elapsed  / 1.03 as f32).sin() + 0.8;
    *goal_material =  materials.add(Color::rgb(r_wave, g_wave, b_wave));
    let player_text = player_text_query.single();
    if player_text.count != 0{//プレイヤーテキストが0じゃなかったら文字アニメーションをしない
        for (_u, (mut _text, mut transform))in text_query.iter_mut().enumerate(){
            transform.scale.y = 0.0;
        }
        return;
    }
    
    for (u, (mut text, mut transform))in text_query.iter_mut().enumerate(){
        let os = (u+1) as f32 * 0.175;
        let transform_wave_y = ((2.0 * std::f32::consts::PI * (elapsed - os)  / 1.0 as f32).sin() + 3.0) * 0.002;
        transform.scale.y = transform_wave_y;
        let r_wave = (2.0 * std::f32::consts::PI * (elapsed - os)  / 1.24 as f32).sin() + 0.8;
        let g_wave = (2.0 * std::f32::consts::PI * (elapsed - os)  / 0.77 as f32).sin() + 0.8;
        let b_wave = (2.0 * std::f32::consts::PI * (elapsed - os)  / 1.03 as f32).sin() + 0.8;
        text.sections[0].style.color = Color::rgb(r_wave, g_wave, b_wave);
    }
    
}

pub fn update_debug(
    mut app: ResMut<MyApp>, 
    mut player_query: Query<(&Velocity, &mut Transform), With<PlayerBlock>>,
    keyboard_input:  Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut timer: ResMut<OneSecondTimer>,
    //mut app_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
    mut exit: EventWriter<bevy::app::AppExit>
) {
    if app.game_state != GameState::Play{return;}
    if !value::ISDEBUG {return;}
    let (_player_velocity, mut player_transform) = player_query.single_mut();
    if keyboard_input.just_pressed(KeyCode::F2){
        app.stage_count += 1;
        app.game_state = GameState::Out;
    }
    if keyboard_input.just_pressed(KeyCode::F3){
        app.game_state = GameState::Out;
    }
    if mouse_button_input.just_released(MouseButton::Right){
        player_transform.translation.x = app.mouse_pos.x;
        player_transform.translation.y = app.mouse_pos.y;
    }
    if keyboard_input.just_pressed(KeyCode::Digit1){
        settings.limiter = bevy_framepace::Limiter::Off;
    }
    if keyboard_input.just_pressed(KeyCode::Digit2){
        settings.limiter = bevy_framepace::Limiter::from_framerate(60.0);
    }
    if keyboard_input.just_pressed(KeyCode::Digit3){
        settings.limiter = bevy_framepace::Limiter::from_framerate(30.0);
    }if keyboard_input.just_pressed(KeyCode::Digit4){
        settings.limiter = bevy_framepace::Limiter::from_framerate(15.0);
    }
    if keyboard_input.just_pressed(KeyCode::Escape){
        exit.send(bevy::app::AppExit);
    }
    if !timer.0.tick(time.delta()).just_finished() { return; }
    //println!("FPS: {}", (1.0 / time.delta_seconds()) as i32);//フレームレート表示
    
}

pub fn update_check_out_of_range(
    mut app: ResMut<MyApp>, 
    player_query: Query<&Transform, With<PlayerBlock>>,
){
    if app.game_state != GameState::Play{return;}
    let player_transform = player_query.single();
    if player_transform.translation.y < -1000.0{
        app.game_state = GameState::Out;
    }
}

pub fn update_player(
    mut app: ResMut<MyApp>, 
    mut player_query: Query<(&mut PlayerInfo, &mut Adjustment, &mut Velocity, &mut Transform), With<PlayerBlock>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut jump_events: EventWriter<JumpEvent>,
) {
    if app.game_state != GameState::Play{return;}
    let (mut player_info, mut player_adjustment, mut player_velocity, mut player_transform) = player_query.single_mut();
    app.timer += time.delta_seconds();
    player_info.is_bend = false;
    
    player_adjustment.x = 0.0;
    player_adjustment.y = 0.0;
    let gravity = -15.0 * time.delta_seconds();
    player_velocity.y += gravity; 
    let sax = app.mouse_pos.x - app.player_pos.x;
    let say = app.mouse_pos.y - app.player_pos.y;
    let val = say.atan2(sax);
    let rad = val - 1.57;
    let mut angle = rad * 180.0 / 3.1415;
    if angle < -180.0{ angle += 360.0; }
    let cnv_angle = if angle > 90.0 { 90.0 - (angle - 90.0) }
    else if angle < -90.0                { -90.0 - (angle + 90.0) }
    else                                 { angle };
    let cnv_rad = angle * 3.1415 / 180.0; 
    player_transform.rotation = Quat::from_rotation_z(cnv_rad);
    
    if player_transform.scale.y > value::BLOCKSIZE{ player_transform.scale.y -= time.delta_seconds() * 40.0; }
    if player_transform.scale.x <= value::BLOCKSIZE{ player_transform.scale.x += time.delta_seconds() * 20.0; }

    if !player_info.is_ground && player_transform.scale.x > value::BLOCKSIZE{ player_transform.scale.x = value::BLOCKSIZE; }
    
    if mouse_button_input.just_pressed(MouseButton::Left){
        if player_info.is_ground{
            player_transform.scale.x = value::BLOCKSIZE;
            player_transform.scale.y = value::BLOCKSIZE;
        }
    }
    if mouse_button_input.pressed(MouseButton::Left) {
        if player_info.is_ground {
            player_info.is_bend = true;
            player_transform.scale.y -= 20.0 * time.delta_seconds();
            player_transform.scale.x += 10.0 * time.delta_seconds();
            if player_transform.scale.y < 10.0 { player_transform.scale.y = 10.0; }
            if player_transform.scale.x > 25.0 { player_transform.scale.x = 25.0; }
        }
    }
    if mouse_button_input.just_released(MouseButton::Left){
        if player_info.is_ground{
            player_info.is_ground = false;
            player_info.is_rising = true;
            let xv = -cnv_angle;
            let jump_val = value::BLOCKSIZE - player_transform.scale.y;
            let y_val = jump_val * 0.75;
            player_velocity.y += y_val;
            let x_val = xv * jump_val * 0.02;
            player_velocity.x += x_val;
            player_transform.scale.y = value::BLOCKSIZE + (jump_val * 0.5);
            player_transform.scale.x = value::BLOCKSIZE - (jump_val * 1.0);
            app.jump_count += 1;
            jump_events.send_default();
        }
    }

    if player_velocity.x < -value::MAXSPEED { player_velocity.x = -value::MAXSPEED; }
    if player_velocity.x > value::MAXSPEED  { player_velocity.x = value::MAXSPEED; }
    if player_velocity.y < -value::MAXSPEED { player_velocity.y = -value::MAXSPEED; }
    if player_velocity.y > value::MAXSPEED  { player_velocity.y = value::MAXSPEED; }
}

pub fn update_collisions(
    app: Res<MyApp>, 
    mut player_query: Query<(&mut PlayerInfo, &mut Adjustment, &mut Velocity, &Transform), (With<PlayerBlock>, Without<Enemy>)>,
    mut block_query: Query<(&Children, &mut BGBlock, &Transform), With<BGBlock>>,
    mut block_text_query: Query<&mut Text, (With<BGText>, Without<PlayerText>)>,
    mut landing_events: EventWriter<LandingEvent>,
    mut side_landing_events: EventWriter<SideLandingEvent>,
    mut get_number_events: EventWriter<GetNumberEvent>,
    mut text_query: Query<(&mut Text, &mut PlayerText), With<PlayerText>>,
    time: Res<Time>,
) {
    if app.game_state != GameState::Play {return;}
    let (mut player_info, mut player_adjustment, mut player_velocity, player_transform) = player_query.single_mut();

    //プレイヤーとブロックの接触
    let mut is_ground = player_info.is_ground;
    let is_rising = player_info.is_rising;
    let mut is_hit_top = false;
    let mut is_hit_side = false;
    let old_is_block_hit = player_info.is_block_hit;
    let mut hit_count = 0;

    let player_size = Vec2::new(value::BLOCKSIZE, player_transform.scale.y);
    let offset = 2.0;
    let op_min = player_transform.translation.truncate() - player_size * 0.5 + (offset * 0.5);
    //let op_min = player_transform.translation.truncate() - player_size * 0.5;
    let op_max = player_transform.translation.truncate() + player_size * 0.5 - (offset * 1.5);
    //let op_max = player_transform.translation.truncate() + player_size * 0.5;
    let mut player_velocity_delta = **player_velocity * (time.delta_seconds() / value::PER60FPS) * 1.0;
    let p_min = op_min + player_velocity_delta;
    let p_max = op_max + player_velocity_delta;

    collision::check_for_collisions_player(&mut hit_count,&mut is_hit_side,&mut is_hit_top,is_rising,&mut is_ground,&mut block_query, &mut block_text_query, &mut player_adjustment,&mut player_velocity,&mut player_velocity_delta,p_min, p_max,op_min,op_max);
    
    if hit_count != 0{
        let (mut text, mut player_text) = text_query.single_mut();
        player_text.count += hit_count;
        text.sections[0].value = format!("{}", player_text.count);
        if hit_count != 0{ get_number_events.send_default(); }
    }
    if is_hit_top && player_velocity.y > 0.0 {player_velocity.y = 0.0; player_info.is_rising = false;}
    if player_adjustment.y != 0.0 || is_hit_side || is_hit_top {player_info.is_block_hit = true;}
    else{ player_info.is_block_hit = false; }
    if !old_is_block_hit && player_info.is_block_hit && player_adjustment.y > 0.0 && !is_hit_side && !is_hit_top && !player_info.is_bend { 
        landing_events.send_default();
    }
    else if !old_is_block_hit && is_hit_side  { side_landing_events.send_default(); }
    else if !old_is_block_hit && is_hit_top   { side_landing_events.send_default(); }
    player_info.is_ground = is_ground;
}

pub fn update_play_sound(
    app: Res<MyApp>, 
    mut commands: Commands,
    jump_sound: Res<JumpSound>,
    mut jump_events: EventReader<JumpEvent>,
    landing_sound: Res<LandingSound>,
    mut landing_events: EventReader<LandingEvent>,
    side_landing_sound: Res<SideLandingSound>,
    mut side_landing_events: EventReader<SideLandingEvent>,
    get_number_sound: Res<GetNumberSound>,
    mut get_number_events: EventReader<GetNumberEvent>,
) {
    if app.game_state != GameState::Play{return;}
    if !jump_events.is_empty() {
        jump_events.clear();
        commands.spawn(AudioBundle {
            source: jump_sound.0.clone(),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: bevy::audio::Volume::new(0.05),
                ..default()
            },
        });
    }
    
    if !landing_events.is_empty() {
        landing_events.clear();
        
        commands.spawn(AudioBundle {
            source: landing_sound.0.clone(),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: bevy::audio::Volume::new(0.05),
                ..default()
            },
        });
    } 

    if !side_landing_events.is_empty()  {
        side_landing_events.clear();
        commands.spawn(AudioBundle {
            source: side_landing_sound.0.clone(),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: bevy::audio::Volume::new(0.05),
                ..default()
            },
        });
    }

    if !get_number_events.is_empty()  {
        get_number_events.clear();
        commands.spawn(AudioBundle {
            source: get_number_sound.0.clone(),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: bevy::audio::Volume::new(0.05),
                ..default()
            },
        });
    }
}

pub fn update_check_goal(
    mut app: ResMut<MyApp>,
    player_query: Query<(&Velocity, &Transform), With<PlayerBlock>>,
    goal_query: Query<&Transform, With<GoalBlock>>,
    text_query: Query<&PlayerText, With<PlayerText>>,
){
    if app.game_state != GameState::Play{return;}
    let (player_velocity,player_transform) = player_query.single();
    let player_text = text_query.single();
    if player_text.count != 0{return;}
    let player_size = player_transform.scale.truncate();
    let offset = 7.0;
    let op_min = player_transform.translation.truncate() - player_size * 0.5 + offset;
    let op_max = player_transform.translation.truncate() + player_size * 0.5 - offset;
    let p_min = op_min + player_velocity.0;
    let p_max = op_max + player_velocity.0;

    let goal_transform = goal_query.single();
    let g_min = goal_transform.translation.truncate() - goal_transform.scale.truncate() / 2.0;
    let g_max = goal_transform.translation.truncate() + goal_transform.scale.truncate() / 2.0; 
    if collision::is_in(p_min, p_max, g_min, g_max){
        app.stage_count += 1;
        app.game_state = GameState::Out;
    }
}

pub fn update_game_state(
    mut app: ResMut<MyApp>,
    time: Res<Time>,
    mut app_state: ResMut<NextState<AppState>>,
    mut black_query: Query<&mut Handle<ColorMaterial>, With<BlackRectangle>>,
    mut player_query: Query<(&mut PlayerInfo, &mut Transform, &mut Velocity), With<PlayerBlock>>,
    mut player_text_query: Query<(&mut Text, &mut PlayerText), With<PlayerText>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    let mut black_color = black_query.single_mut();
    if app.game_state == GameState::In{
        app.game_state_timer += time.delta_seconds();
        *black_color = materials.add(Color::rgba(0.0,0.0,0.0, 1.0 - (app.game_state_timer*1.0)));
        if app.game_state_timer >= value::FADETIME{
            *black_color = materials.add(Color::rgba(0.0,0.0,0.0, 0.0));
            app.game_state_timer = 0.0;
            app.game_state = GameState::Play;
        }
    }
    if app.game_state == GameState::Out{
        app.game_state_timer += time.delta_seconds();
        *black_color = materials.add(Color::rgba(0.0,0.0,0.0, app.game_state_timer*1.0));
        if app.game_state_timer >= value::FADETIME{
            *black_color = materials.add(Color::rgba(0.0,0.0,0.0, 1.0));
            app.game_state_timer = 0.0;
            app.game_state = GameState::In;
            app.is_reset_game = true;
            if app.stage_count > value::MAXSTAGE{app_state.set(AppState::Ending);}
            //let (mut player_transform, mut player_velocity) = player_query.single_mut();
            //player_transform.translation.x = value::DEFAULTCAMERAPOSX;
            //player_transform.translation.y = value::DEFAULTCAMERAPOSY;
            //player_velocity.x = 0.0;
            //player_velocity.y = 0.0;
        }
    }
    if app.is_reset_game{
        let (mut player_info, mut player_transform, mut player_velocity) = player_query.single_mut();
        (player_transform.translation.x, player_transform.translation.y) = (value::DEFAULTPOSX, value::DEFAULTPOSY);
        (player_velocity.x, player_velocity.y) = (0.0, 0.0);
        player_info.is_ground = false;
        player_info.is_rising = false;
        let (mut text, mut player_text) = player_text_query.single_mut();
        player_text.count = 0;
        text.sections[0].value = format!("{}", player_text.count);
    }
}

pub fn update_reset_game(
    mut commands: Commands, 
    mut app: ResMut<MyApp>, 
    block_query: Query<Entity, With<BGBlock>>,
    enemy_query: Query<Entity, With<Enemy>>,
    goal_query: Query<Entity, With<GoalBlock>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
){
    if !app.is_reset_game{return;}
    app.text_stage_alpha = value::DEFAULTTEXTSTAGEALPHA;
    for entity in &block_query {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &enemy_query{
        commands.entity(entity).despawn_recursive();
    }
    for entity in &goal_query {
        commands.entity(entity).despawn_recursive();
    }
    app.is_reset_game = false;
    create_block(app, commands, meshes, materials, asset_server);
}

pub fn update_apply_velocity_player(
    mut app: ResMut<MyApp>, 
    mut player_query: Query<(&mut PlayerInfo, &Adjustment, &mut Transform, &mut Velocity), With<PlayerBlock>>,
    time: Res<Time>,
) {
    let (mut player_info, player_adjustment, mut player_transform, mut player_velocity) = player_query.single_mut();  
    app.player_pos.x = player_transform.translation.x;
    app.player_pos.y = player_transform.translation.y;

    if app.game_state != GameState::Play{return;}
    let delta_player_velocity = **player_velocity * (time.delta_seconds() / value::PER60FPS);
    player_transform.translation.x += player_adjustment.x;
    player_transform.translation.y += player_adjustment.y;
    player_transform.translation.x += delta_player_velocity.x;
    player_transform.translation.y += delta_player_velocity.y;
    player_transform.translation.x += app.vel.x * time.delta_seconds() * 10.0;
    player_transform.translation.y += app.vel.y * time.delta_seconds() * 10.0;

    if player_adjustment.y > 0.0 { player_velocity.y = 0.0; }

    app.vel.x = app.vel.x * (1.0 - time.delta_seconds() * 20.0);
    app.vel.y = app.vel.y * (1.0 - time.delta_seconds() * 20.0);

    if app.old_velocity_y >= 0.0 && player_velocity.y < 0.0 { player_info.is_rising = false; }
    app.old_velocity_y = player_velocity.y;
    if player_info.is_ground{ player_velocity.x = player_velocity.x * (1.0 - time.delta_seconds() * 20.0); }
    else            { player_velocity.x = player_velocity.x * (1.0 - time.delta_seconds() *  1.0); }
    
    if player_info.is_ground {player_info.is_rising = false;}
}