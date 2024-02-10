use bevy::{
    prelude::*,
    sprite::*, 
    render::mesh::*,
    sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
};
use rand::{thread_rng, Rng};
use rand::distributions::{Distribution, Uniform};
use super::common;
use super::collision;
use super::super::state::*;

#[derive(Component)]
pub struct CameraMarker;

#[derive(Component)]
pub struct BGBlock;

#[derive(Component)]
pub struct GoalBlock;

#[derive(Component, Copy, Clone)]
pub struct GoalMarker{pub color: Color }

#[derive(Component)]
pub struct PlayerBlock;

#[derive(Component)]
pub struct PlayerAvatar;

#[derive(Debug, Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

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


pub fn update_camera_move(
    mut app: ResMut<MyApp>, 
    mut camera_query: Query<(&Camera, &mut Transform, &GlobalTransform), With<CameraMarker>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let (camera, mut camera_transform, camera_global_transform) = camera_query.single_mut();
    camera_transform.translation.x += (app.player_pos.x - camera_transform.translation.x) * 0.01;
    camera_transform.translation.y += (app.player_pos.y - camera_transform.translation.y) * 0.01;
    if app.is_reset_game{
        camera_transform.translation.x = value::DEFAULTCAMERAPOSX;
        camera_transform.translation.y = value::DEFAULTCAMERAPOSY;
    }
    let window = q_window.single();
    if  window.cursor_position().is_none(){return;}
    let wcp = window.cursor_position().unwrap();
    let res = camera.viewport_to_world(camera_global_transform, wcp).map(|ray| ray.origin.truncate());
    if res.is_none(){return;}
    app.mouse_pos.x = res.unwrap().x;
    app.mouse_pos.y = res.unwrap().y;
}

pub fn setup_asset(
    mut app: ResMut<MyApp>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    if app.stage_count == 1{ *app = MyApp::default(); }
    commands.insert_resource(ClearColor(Color::rgb(0.15, 0.15, 0.15)));

    let mut cam = Camera2dBundle::default();
    cam.transform = Transform::from_xyz(500.0, 0.0, 1.0);
    commands.spawn((cam, ReleaseResource, CameraMarker));
    
    commands.spawn((AudioBundle {
        source: asset_server.load(common::BGM),
        settings: PlaybackSettings{
            mode: bevy::audio::PlaybackMode::Loop,
            volume: bevy::audio::Volume::Relative(bevy::audio::VolumeLevel::new(0.05)),
            ..default()
        },
        ..default()
        },
        ReleaseResource
    ));
    commands.insert_resource(JumpSound(asset_server.load(common::SOUNDJUMP)));
    commands.insert_resource(LandingSound(asset_server.load(common::SOUNDLANDING)));

    let font = asset_server.load(common::DEFAULTFONT);
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
        mesh: meshes.add(shape::Quad::default().into()).into(),
        transform: Transform::default().with_translation(Vec3::new(value::DEFAULTPOSX,value::DEFAULTPOSY,5.0)).with_scale(Vec3::splat(20.0)),
        material: materials.add(Color::BLACK.into()),
        visibility: Visibility::Visible,
        ..default()
        },
        PlayerBlock,
        ReleaseResource,
        Velocity(Vec2::new(0.0, 0.0)),
    )).with_children(|parent| {
        parent.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::default().into()).into(),
            transform: Transform::default().with_translation(Vec3::new(0.0,0.0,1.0)).with_scale(Vec3::splat(0.9)),
            material: materials.add(Color::ORANGE.into()),
            ..default()
        });   

        parent.spawn(MaterialMesh2dBundle {//左目
            mesh: meshes.add(shape::Circle::default().into()).into(),
            transform: Transform::default().with_translation(Vec3::new(-0.2,0.15,2.0)).with_scale(Vec3::splat(0.15)),
            material: materials.add(Color::BLACK.into()),
            ..default()
        });    
        parent.spawn(MaterialMesh2dBundle {//右目
            mesh: meshes.add(shape::Circle::default().into()).into(),
            transform: Transform::default().with_translation(Vec3::new(0.2,0.15,2.0)).with_scale(Vec3::splat(0.15)),
            material: materials.add(Color::BLACK.into()),
            ..default()
        });
        let mut cps = shape::Capsule::default();
        cps.radius = 1.0;
        cps.depth = 0.1;

        let mut triangle = Mesh::new(PrimitiveTopology::TriangleList);
        triangle.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
             vec![[-0.5, -0.4, 0.0], [0.5, -0.4, 0.0], [0.0, 0.3, 0.0]],
        );
        triangle.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![[0.0, 0.0, 0.0, 1.0]; 3]);
        triangle.set_indices(Some(Indices::U32(vec![0, 1, 2])));

        parent.spawn(MaterialMesh2dBundle {//口
            mesh: meshes.add(triangle.into()).into(),
            transform: Transform::default().with_translation(Vec3::new(0.0,-0.15,2.0)).with_scale(Vec3::splat(0.3)),
            material: materials.add(Color::BLACK.into()),
            ..default()
        }); 
    });

    create_block(app.stage_count, commands, meshes, materials, asset_server);
}

pub fn update_fade_stage_text(
    mut app: ResMut<MyApp>, 
    time: Res<Time>,
    mut text_query: Query<&mut Text, With<StageText>>,
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
    /*
    let mut text = text_query.single_mut();
    if app.text_stage_alpha == value::DEFAULTTEXTSTAGEALPHA{
        text.sections[0].value =  match app.stage_count == value::MAXSTAGE{
            true => {"Last Stage".into()},
            _ => {format!("Stage {}",app.stage_count)},
        };
    }
    let alpha = match app.text_stage_alpha > 1.0{
        true => 1.0,
        _ => app.text_stage_alpha,
    };
    text.sections[0].style.color = Color::rgba(1.0,1.0,1.0, alpha);
    */
    app.text_stage_alpha -= time.delta_seconds();
}

pub fn create_block(
    stage_count: u32,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
){
    let mut rng = thread_rng();
    let height = 10*(stage_count*2);
    for x in 2..49{
        for y in 2..height{
            let v: u32 = rng.gen();
            if v % (2+(stage_count*2)) != 0{continue;}
            let xx = x as f32;
            let yy = y as f32;
            commands.spawn((
                MaterialMesh2dBundle {
                mesh: meshes.add(shape::Quad::default().into()).into(),
                transform: Transform::default().with_translation(Vec3::new(xx*20.0,yy*20.0,0.0)).with_scale(Vec3::splat(20.0)),
                material: materials.add(Color::GRAY.into()),
                ..default()
                },
                BGBlock,
                ReleaseResource
            )).with_children(|parent| {
                parent.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Quad::default().into()).into(),
                    transform: Transform::default().with_translation(Vec3::new(0.0,0.0,1.0)).with_scale(Vec3::splat(0.9)),
                    material: materials.add(Color::DARK_GRAY.into()),
                    ..default()
                });
            });
        }
    }
    
    let range = Uniform::new(2,49);
    let mut rng = rand::thread_rng();
    let x = range.sample(&mut rng);
    let goal_or_next = match stage_count == value::MAXSTAGE{
        true => {"GOAL!"},
        _ => {"NEXT!"},
    };
    let gm = GoalMarker{color: Color::rgb(0.0, 1.0, 0.0)};
    commands.spawn((
        MaterialMesh2dBundle {
        mesh: meshes.add(shape::Quad::default().into()).into(),
        transform: Transform::default().with_translation(Vec3::new(x as f32*20.0, height as f32 * 20.0,0.0)).with_scale(Vec3::splat(20.0)),
        material: materials.add(gm.color.clone().into()),
        ..default()
        },
        gm,
        GoalBlock,
        ReleaseResource
    )).with_children(|parent| {
        parent.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::default().into()).into(),
            transform: Transform::default().with_translation(Vec3::new(0.0,0.0,1.0)).with_scale(Vec3::splat(0.9)),
            material: materials.add(Color::BLACK.into()),
            ..default()
        });

        for (u, c) in goal_or_next.chars().enumerate(){
            parent.spawn((
                Text2dBundle {
                    text: Text::from_section(c.to_string(), TextStyle {
                        font: asset_server.load(common::DEFAULTFONT),
                        font_size: 100.0,
                        color: Color::WHITE,
                    }),
                    text_anchor: Anchor::BottomCenter,
                    transform: Transform{
                        translation: Vec3::new((u as f32 * 0.4) - 0.60,-1.2,2.0),
                        scale: Vec3::new(0.01,0.01,0.01),
                        ..default()
                    },
                    ..default()
                },
                GoalText,
                ReleaseResource,
            ));
        }
    });
    
    for i in 1..50{
        let v = i as f32;
        commands.spawn((
            MaterialMesh2dBundle {//下面ブロック
            mesh: meshes.add(shape::Quad::default().into()).into(),
            transform: Transform::default().with_translation(Vec3::new(v*20.0,0.0,0.0)).with_scale(Vec3::splat(20.0)),
            material: materials.add(Color::GRAY.into()),
            ..default()
            },
            BGBlock,
            ReleaseResource
        )).with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {//下面ブロック
                mesh: meshes.add(shape::Quad::default().into()).into(),
                transform: Transform::default().with_translation(Vec3::new(0.0,0.0,1.0)).with_scale(Vec3::splat(0.9)),
                material: materials.add(Color::DARK_GRAY.into()),
                ..default()
            });
        });
    }
    for i in 0..(height+1+30){
        let v = i as f32;
        commands.spawn((MaterialMesh2dBundle {//左面ブロック
            mesh: meshes.add(shape::Quad::default().into()).into(),
            transform: Transform::default().with_translation(Vec3::new(0.0,v*20.0,0.0)).with_scale(Vec3::splat(20.0)),
            material: materials.add(Color::GRAY.into()),
            ..default()
            },
            BGBlock,
            ReleaseResource
        )).with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Quad::default().into()).into(),
                transform: Transform::default().with_translation(Vec3::new(0.0,0.0,1.0)).with_scale(Vec3::splat(0.9)),
                material: materials.add(Color::DARK_GRAY.into()),
                ..default()
            });
        });
        commands.spawn((MaterialMesh2dBundle {//右面ブロック
            mesh: meshes.add(shape::Quad::default().into()).into(),
            transform: Transform::default().with_translation(Vec3::new(1000.0,v*20.0,0.0)).with_scale(Vec3::splat(20.0)),
            material: materials.add(Color::GRAY.into()),
            ..default()
            },
            BGBlock,
            ReleaseResource
        )).with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Quad::default().into()).into(),
                transform: Transform::default().with_translation(Vec3::new(0.0,0.0,1.0)).with_scale(Vec3::splat(0.9)),
                material: materials.add(Color::DARK_GRAY.into()),
                ..default()
            });
        });
    }
}

pub fn update_goal_animation(
    mut text_query: Query<(&mut Text, &mut Transform), With<GoalText>>,
    mut goal_query: Query<&mut Handle<ColorMaterial>, With<GoalBlock>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>
) {
    let elapsed = time.elapsed().as_secs_f32();
    let mut goal_material = goal_query.single_mut();
    let r_wave = (2.0 * std::f32::consts::PI * elapsed  / 1.24 as f32).sin() + 0.8;
    let g_wave = (2.0 * std::f32::consts::PI * elapsed  / 0.77 as f32).sin() + 0.8;
    let b_wave = (2.0 * std::f32::consts::PI * elapsed  / 1.03 as f32).sin() + 0.8;
    *goal_material =  materials.add(Color::rgb(r_wave, g_wave, b_wave).into());
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
    mut player_query: Query<&mut Transform, With<PlayerBlock>>,
    keyboard_input:  Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut timer: ResMut<OneSecondTimer>,
    mut app_state: ResMut<NextState<AppState>>,
    time: Res<Time>
) {
    let mut player_transform = player_query.single_mut();
    if keyboard_input.just_pressed(KeyCode::F2){
        app.stage_count += 1;
        if app.stage_count > value::MAXSTAGE{app_state.set(AppState::Ending);}
        app.is_reset_game = true;
    }
    if mouse_button_input.just_released(MouseButton::Right){
        player_transform.translation.x = app.mouse_pos.x;
        player_transform.translation.y = app.mouse_pos.y;
    }
    if !timer.0.tick(time.delta()).just_finished() { return; }
    //println!("FPS: {}", (1.0 / time.delta_seconds()) as i32);//フレームレート表示
}

pub fn update_check_out_of_range(
    mut app: ResMut<MyApp>, 
    player_query: Query<&Transform, With<PlayerBlock>>,
){
    let player_transform = player_query.single();
    if player_transform.translation.y < -1000.0{
        app.is_reset_game = true;
    }
}

pub fn update_player(
    mut app: ResMut<MyApp>, 
    mut player_query: Query<(&mut Velocity, &mut Transform), With<PlayerBlock>>,
    mouse_button_input: Res<Input<MouseButton>>,
    time: Res<Time>,
    mut jump_events: EventWriter<JumpEvent>,
) {
    app.timer += time.delta_seconds();

    let (mut player_velocity, mut player_transform) = player_query.single_mut();
    player_velocity.y += -15.0 * time.delta_seconds();
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

    if player_transform.scale.y > value::BLOCKSIZE{ player_transform.scale.y -= time.delta_seconds() * 40.0;}
    if player_transform.scale.x < value::BLOCKSIZE{
        player_transform.scale.x += time.delta_seconds() * 20.0;
    }
    if !app.is_ground && player_transform.scale.x > value::BLOCKSIZE{ player_transform.scale.x = value::BLOCKSIZE; }

    if mouse_button_input.just_pressed(MouseButton::Left){
        if app.is_ground{
            player_transform.scale.x = value::BLOCKSIZE;
            player_transform.scale.y = value::BLOCKSIZE;
        }
    }
    if mouse_button_input.pressed(MouseButton::Left) {
        if app.is_ground{
            player_transform.scale.y -= 20.0 * time.delta_seconds();
            player_transform.scale.x += 10.0 * time.delta_seconds();
            if player_transform.scale.y < 10.0 { player_transform.scale.y = 10.0; }
            if player_transform.scale.x > 25.0 { player_transform.scale.x = 25.0; }
        }
    }
    if mouse_button_input.just_released(MouseButton::Left){
        if app.is_ground{
            app.is_ground = false;
            let xv = -cnv_angle;
            let yv = 90.0 - xv.abs() + 1.0;
            let val = (value::BLOCKSIZE - player_transform.scale.y) * yv * time.delta_seconds() + 1.0;
            player_velocity.y += val;
            let jump_ratio = value::BLOCKSIZE - player_transform.scale.y;
            player_velocity.x += xv * 5.0 * jump_ratio * time.delta_seconds();
            player_transform.scale.y = value::BLOCKSIZE + (jump_ratio * 0.5);
            player_transform.scale.x = value::BLOCKSIZE - (jump_ratio * 1.0);
            app.jump_count += 1;
            jump_events.send_default();
        }
    }

    if player_velocity.x < -value::MAXSPEED{player_velocity.x = -value::MAXSPEED;}
    if player_velocity.x > value::MAXSPEED {player_velocity.x = value::MAXSPEED;}
    if player_velocity.y < -value::MAXSPEED{player_velocity.y = -value::MAXSPEED;}
    if player_velocity.y > value::MAXSPEED * 2.0 {player_velocity.y = value::MAXSPEED * 2.0;}
}

pub fn update_check_for_collisions(
    mut app: ResMut<MyApp>, 
    mut player_query: Query<(&mut Velocity, &Transform), With<PlayerBlock>>,
    block_query: Query<&Transform, With<BGBlock>>,
    mut landing_events: EventWriter<LandingEvent>
) {
    let (mut player_velocity, player_transform) = player_query.single_mut();
    //let player_size = player_transform.scale.truncate();
    let player_size = Vec2::new(value::BLOCKSIZE, player_transform.scale.y);
    //let player_size = value::BLOCKSIZE;
    let offset = 2.0;
    let op_min = player_transform.translation.truncate() - player_size * 0.5 + offset;
    let op_max = player_transform.translation.truncate() + player_size * 0.5 - offset;
    let p_min = op_min + **player_velocity;
    let p_max = op_max + **player_velocity;
    let rto = 2.0;//左右の跳ね返り倍率
    
    let mut hit_blocks = Vec::new();
    for  transform in block_query.iter() {//接触しているブロックのtransformを取得、接触しているブロックの色替え
        let b_min = transform.translation.truncate() - transform.scale.truncate() / 2.0;
        let b_max = transform.translation.truncate() + transform.scale.truncate() / 2.0; 
        if !collision::is_in(p_min, p_max, b_min, b_max){continue;}
        hit_blocks.push(transform);
    }
    if hit_blocks.is_empty() { return;}
    let mut is_hit = false;
    for t in &hit_blocks{//上下左右の4面の接触判定
        let b_min = t.translation.truncate() - t.scale.truncate() / 2.0;
        let b_max = t.translation.truncate() + t.scale.truncate() / 2.0; 
        if player_velocity.y < 0.0{
            let p_pos = Vec2::new((p_min.x + p_max.x)*0.5, p_min.y);//pの下チェック
            let ry = collision::check_bottom_collide(p_pos, b_min, b_max);
            if ry != 0.0{
                player_velocity.y += ry;
                is_hit = true;
                if !app.is_ground {
                    app.is_ground = true;
                    landing_events.send_default();
                }
            }
        }
        
        let p_pos = Vec2::new((p_min.x + p_max.x)*0.5, p_max.y);//pの上チェック
        let ry = collision::check_top_collide(p_pos, b_min, b_max);
        if ry != 0.0{
            player_velocity.y += ry;
            is_hit = true;
        }
        let p_pos = Vec2::new(p_min.x, (p_min.y+p_max.y)*0.5);//pの左チェック
        let rx = collision::check_left_collide(p_pos, b_min, b_max);
        if rx != 0.0{
            if app.is_ground{ player_velocity.x += rx; }
            else            { player_velocity.x += rx*rto; }
            is_hit = true;
        }
        let p_pos = Vec2::new(p_max.x, (p_min.y+p_max.y)*0.5);//pの右チェック
        let rx = collision::check_right_collide(p_pos, b_min, b_max);
        if rx != 0.0{
            if app.is_ground{ player_velocity.x += rx; }
            else            { player_velocity.x += rx*rto; }
            is_hit = true;
        }
    }
    if is_hit { return; }
    for t in &hit_blocks {//各頂点の接触判定
        let b_min = t.translation.truncate() - t.scale.truncate() / 2.0;
        let b_max = t.translation.truncate() + t.scale.truncate() / 2.0; 
        let (rx, ry) = collision::check_left_bottom_collide(op_min, op_max,**player_velocity, b_min, b_max);
        if ry > 0.0 && !app.is_ground && player_velocity.y < 0.0{
            app.is_ground = true;
            landing_events.send_default();
        }
        if app.is_ground{ player_velocity.x += rx; }
        else            { player_velocity.x += rx*rto; }
        player_velocity.y += ry;
        let (rx, ry) = collision::check_right_bottom_collide(op_min, op_max,**player_velocity, b_min, b_max);
        if ry > 0.0 && !app.is_ground && player_velocity.y < 0.0{
            app.is_ground = true;
            landing_events.send_default();
        }
        if app.is_ground{ player_velocity.x += rx; }
        else            { player_velocity.x += rx*rto; }
        player_velocity.y += ry;
        let (rx, ry) = collision::check_left_top_collide(op_min, op_max,**player_velocity, b_min, b_max);
        if app.is_ground{ player_velocity.x += rx; }
        else            { player_velocity.x += rx*rto; }
        player_velocity.y += ry;
        let (rx, ry) = collision::check_right_top_collide(op_min, op_max,**player_velocity, b_min, b_max);
        if app.is_ground{ player_velocity.x += rx; }
        else            { player_velocity.x += rx*rto; }
        player_velocity.y += ry;
    }
}

pub fn update_play_sound(
    mut commands: Commands,
    jump_sound: Res<JumpSound>,
    mut jump_events: EventReader<JumpEvent>,
    landing_sound: Res<LandingSound>,
    mut landing_events: EventReader<LandingEvent>,
) {
    if !jump_events.is_empty() {
        jump_events.clear();
        commands.spawn(AudioBundle {
            source: jump_sound.0.clone(),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: bevy::audio::Volume::Relative(bevy::audio::VolumeLevel::new(0.05)),
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
                volume: bevy::audio::Volume::Relative(bevy::audio::VolumeLevel::new(0.05)),
                ..default()
            },
        });
    }    
}

pub fn update_check_goal(
    mut app: ResMut<MyApp>,
    player_query: Query<(&Velocity, &Transform), With<PlayerBlock>>,
    goal_query: Query<&Transform, With<GoalBlock>>,
    mut app_state: ResMut<NextState<AppState>>,
){
    let (player_velocity,player_transform) = player_query.single();
    let player_size = player_transform.scale.truncate();
    let offset = 7.5;
    let op_min = player_transform.translation.truncate() - player_size * 0.5 + offset;
    let op_max = player_transform.translation.truncate() + player_size * 0.5 - offset;
    let p_min = op_min + player_velocity.0;
    let p_max = op_max + player_velocity.0;

    let goal_transform = goal_query.single();
    let g_min = goal_transform.translation.truncate() - goal_transform.scale.truncate() / 2.0;
    let g_max = goal_transform.translation.truncate() + goal_transform.scale.truncate() / 2.0; 
    if collision::is_in(p_min, p_max, g_min, g_max){
        app.stage_count += 1;
        app.is_reset_game = true;
        if app.stage_count > value::MAXSTAGE{app_state.set(AppState::Ending);}
    }
}

pub fn update_reset_game(
    mut commands: Commands, 
    mut app: ResMut<MyApp>, 
    block_query: Query<Entity, With<BGBlock>>,
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
    for entity in &goal_query {
        commands.entity(entity).despawn_recursive();
    }
    create_block(app.stage_count, commands, meshes, materials, asset_server);
    app.is_reset_game = false;
}

pub fn update_apply_velocity(
    mut app: ResMut<MyApp>, 
    mut player_query: Query<(&mut Transform, &mut Velocity), With<PlayerBlock>>,
) {
    let (mut player_transform, mut player_velocity) = player_query.single_mut();    
    player_transform.translation.x += player_velocity.x;
    player_transform.translation.y += player_velocity.y;
    if app.is_ground{
        player_velocity.x = player_velocity.x * 0.75;
    }else{
        player_velocity.x = player_velocity.x * 0.99;
    }
    if app.is_reset_game{
        player_transform.translation.x = value::DEFAULTPOSX;
        player_transform.translation.y = value::DEFAULTPOSY;
    }
    app.player_pos.x = player_transform.translation.x;
    app.player_pos.y = player_transform.translation.y;
}