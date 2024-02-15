use bevy::{
    prelude::*,
    sprite::*, 
    sprite::MaterialMesh2dBundle,
};
use rand::{thread_rng, Rng};
use rand::distributions::{Distribution, Uniform};

use super::state::*;
use super::state::game::*;
use super::define::*;

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
                        font: asset_server.load(assets::DEFAULTFONT),
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

    for i in 1..50{//上面ブロック
        let v = i as f32;
        commands.spawn((
            MaterialMesh2dBundle {//上面ブロック
            mesh: meshes.add(shape::Quad::default().into()).into(),
            transform: Transform::default().with_translation(Vec3::new(v*value::BLOCKSIZE,(height+5) as f32 * value::BLOCKSIZE,0.0)).with_scale(Vec3::splat(20.0)),
            material: materials.add(Color::GRAY.into()),
            ..default()
            },
            BGBlock,
            ReleaseResource
        )).with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {//上面ブロック
                mesh: meshes.add(shape::Quad::default().into()).into(),
                transform: Transform::default().with_translation(Vec3::new(0.0,0.0,1.0)).with_scale(Vec3::splat(0.9)),
                material: materials.add(Color::DARK_GRAY.into()),
                ..default()
            });
        });
    }

    for i in 0..(height+1+5){
        let v = i as f32;
        commands.spawn((MaterialMesh2dBundle {//左面ブロック
            mesh: meshes.add(shape::Quad::default().into()).into(),
            transform: Transform::default().with_translation(Vec3::new(0.0,v*value::BLOCKSIZE,0.0)).with_scale(Vec3::splat(20.0)),
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