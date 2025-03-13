use bevy::{
    prelude::*,
    sprite::*, 
    sprite::MeshMaterial2d,
    color::palettes::basic,
};
use rand::{thread_rng, Rng};
use rand::distributions::{Distribution, Uniform};

use super::state::*;
use super::state::game::*;
use super::define::*;

pub fn create_block(
    mut app: ResMut<MyApp>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
){
    let font = TextFont{
        font: asset_server.load(assets::DEFAULTFONT),
        ..default()
    };
    let mut rng = thread_rng();
    let height = 10*(app.stage_count*2);
    let cvt_stage_count = match app.stage_count{
        1 => 1,
        2 => 1,
        3 => 2,
        _ => 1,
    };
    let block_color = Color::srgb(value::DARKBLOCKCOLOR, value::DARKBLOCKCOLOR,value::DARKBLOCKCOLOR);
    let vl_range:Uniform<i32> = Uniform::new(-cvt_stage_count, cvt_stage_count+1);
    for x in 2..49{
        for y in 2..height{
            let v: u32 = rng.gen();
            if v % (2+(app.stage_count*2)) != 0{continue;}
            let xx = x as f32;
            let yy = y as f32;

            
            let mut rng = rand::thread_rng();
            let vl = match app.is_clear{
                true => {vl_range.sample(&mut rng)},
                _ =>    {0},
            };

            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                Transform::default().with_translation(Vec3::new(xx*20.0,yy*20.0,0.0)).with_scale(Vec3::splat(20.0)),
                MeshMaterial2d(materials.add(Color::from(basic::GRAY))),
                BGBlock{count: vl},
                ReleaseResource
            )).with_children(|parent| {
                parent.spawn((
                    Mesh2d(meshes.add(Rectangle::default())),
                    Transform::default().with_translation(Vec3::new(0.0,0.0,1.0)).with_scale(Vec3::splat(0.9)),
                    MeshMaterial2d(materials.add(block_color)),
                ));
                parent.spawn((
                    Text2d::new(get_number_string(vl)),
                    TextFont {
                        font: asset_server.load(assets::DEFAULTFONT),
                        font_size: 70.0,
                        ..default()
                    },
                    Transform::default().with_translation(Vec3::new(0.0,0.05,10.0)).with_scale(Vec3::splat(0.010)),
                    BGText{count: vl},
                ));
            });
        }
    }
        
    let vl = 0;
    let sb = match app.is_clear{
        true => {2},
        _ => {1}
    };
    
    for i in sb..50{//下面ブロック
        let v = i as f32;
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            Transform::default().with_translation(Vec3::new(v*20.0,0.0,0.0)).with_scale(Vec3::splat(20.0)),
            MeshMaterial2d(materials.add(Color::from(basic::GRAY))),
            BGBlock{count: vl},
            ReleaseResource
        )).with_children(|parent| {
            parent.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                Transform::default().with_translation(Vec3::new(0.0,0.0,1.0)).with_scale(Vec3::splat(0.9)),
                MeshMaterial2d(materials.add(block_color)),
            ));
            parent.spawn((
                Text2d::new(get_number_string(vl)),
                TextFont {
                    font: asset_server.load(assets::DEFAULTFONT),
                    font_size: 100.0,
                    ..default()
                },
                MeshMaterial2d(materials.add(Color::from(basic::GRAY))),
                Transform::default().with_translation(Vec3::new(0.0,0.05,10.0)).with_scale(Vec3::splat(0.0090)),
                BGText{count: vl},
            ));
        });
    }

    for i in 1..50{//上面ブロック
        let v = i as f32;
        commands.spawn((
            MeshMaterial2d(materials.add(Color::from(basic::GRAY))),
            Transform::default().with_translation(Vec3::new(v*value::BLOCKSIZE,(height+5) as f32 * value::BLOCKSIZE,0.0)).with_scale(Vec3::splat(20.0)),
            Mesh2d(meshes.add(Rectangle::default())),
            BGBlock{count: vl},
            ReleaseResource
        )).with_children(|parent| {
            parent.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(block_color)),
                Transform::default().with_translation(Vec3::new(0.0,0.0,1.0)).with_scale(Vec3::splat(0.9)),
            ));
            parent.spawn((
                Text2d::new(get_number_string(vl)),
                TextFont {
                    font: asset_server.load(assets::DEFAULTFONT),
                    font_size: 100.0,
                    ..default()
                },
                MeshMaterial2d(materials.add(Color::from(basic::GRAY))),
                Transform::default().with_translation(Vec3::new(0.0,0.05,10.0)).with_scale(Vec3::splat(0.0090)),
                BGText{count: vl},
            ));
        });
    }

    for i in 0..(height+1+5){
        let v = i as f32;
        commands.spawn((
            MeshMaterial2d(materials.add(Color::from(basic::GRAY))),
            Transform::default().with_translation(Vec3::new(0.0,v*value::BLOCKSIZE,0.0)).with_scale(Vec3::splat(20.0)),
            Mesh2d(meshes.add(Rectangle::default())),
            BGBlock{count: vl},
            ReleaseResource
        )).with_children(|parent| {
            parent.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                Transform::default().with_translation(Vec3::new(0.0,0.0,1.0)).with_scale(Vec3::splat(0.9)),
                MeshMaterial2d(materials.add(block_color)),
            ));
            parent.spawn((
                Text2d::new(get_number_string(vl)),
                font.clone(),
                MeshMaterial2d(materials.add(Color::from(basic::GRAY))),
                Transform::default().with_translation(Vec3::new(0.0,0.05,10.0)).with_scale(Vec3::splat(0.0090)),
                BGText{count: vl},
            ));
        });

        commands.spawn((
            MeshMaterial2d(materials.add(Color::from(basic::GRAY))),
            Transform::default().with_translation(Vec3::new(1000.0,v*20.0,0.0)).with_scale(Vec3::splat(20.0)),
            Mesh2d(meshes.add(Rectangle::default())),
            BGBlock{count: vl},
            ReleaseResource
        )).with_children(|parent| {
            parent.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                Transform::default().with_translation(Vec3::new(0.0,0.0,1.0)).with_scale(Vec3::splat(0.9)),
                MeshMaterial2d(materials.add(block_color)),
            ));
            parent.spawn((
                Text2d::new(get_number_string(vl)),
                TextFont {
                    font: asset_server.load(assets::DEFAULTFONT),
                    font_size: 100.0,
                    ..default()
                },
                MeshMaterial2d(materials.add(Color::from(basic::GRAY))),
                Transform::default().with_translation(Vec3::new(0.0,0.05,10.0)).with_scale(Vec3::splat(0.0090)),
                BGText{count: vl},
            ));
        });
    }

    let range = Uniform::new(2,49);
    let mut rng = rand::thread_rng();
    let x = range.sample(&mut rng);
    let goal_or_next = match app.stage_count == value::MAXSTAGE{
        true => {"GOAL!"},
        _ => {"NEXT!"},
    };
    let gm = GoalMarker{color: Color::srgb(0.0, 1.0, 0.0)};
    app.goal_pos.x = x as f32 * 20.0;
    app.goal_pos.y = height as f32 * 20.0;
    let number_visibility = match app.is_clear{
        true => { Visibility::Visible },
        _ =>    { Visibility::Hidden },
    };
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        Transform::default().with_translation(Vec3::new(app.goal_pos.x, app.goal_pos.y,0.0)).with_scale(Vec3::splat(20.0)),
        MeshMaterial2d(materials.add(gm.color)),
        gm,
        GoalBlock,
        ReleaseResource
    )).with_children(|parent| {
        parent.spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            Transform::default().with_translation(Vec3::new(0.0,0.0,1.0)).with_scale(Vec3::splat(0.9)),
            MeshMaterial2d(materials.add(Color::from(basic::BLACK))),
        ));

        parent.spawn((
            Text2d::new("0"),
            TextFont {
                font: asset_server.load(assets::DEFAULTFONT),
                font_size: 100.0,
                ..default()
            },
            MeshMaterial2d(materials.add(Color::from(basic::GRAY))),
            Transform::default().with_translation(Vec3::new(0.0,0.05,10.0)).with_scale(Vec3::splat(0.0090)),
            number_visibility,
        ));

        for (u, c) in goal_or_next.chars().enumerate(){
            parent.spawn((
                Text2d::new(c.to_string()),
                TextFont {
                    font: asset_server.load(assets::DEFAULTFONT),
                    font_size: 75.0,
                    ..default()
                },
                MeshMaterial2d(materials.add(Color::from(basic::GRAY))),
                Transform::default()
                    .with_translation(Vec3::new((u as f32 * 0.4) - 0.60, 0.5,2.0))
                    .with_scale(Vec3::new(0.01,0.01,0.01)),
                Anchor::BottomCenter,
                GoalText,
                ReleaseResource,
            ));
        }
    });
}

pub fn get_number_string(v: i32) -> String{
    let res = match v{
        0 => {String::new()},
        _ => {
            match 0 < v{
                true => format!("{}",v),
                _ => format!("{}",v)
            }
        },
    };
    return res;
}