use bevy::prelude::*;
use super::state;

pub fn is_in(p_min: Vec2, p_max: Vec2, b_min: Vec2, b_max: Vec2) -> bool{
    if p_min.x < b_max.x && p_max.x > b_min.x && p_min.y < b_max.y && p_max.y > b_min.y { true }
    else{ false }
}

pub fn check_top_collide(p_pos: Vec2, b_min: Vec2, b_max: Vec2) -> f32{//pの上チェック
    let mut res_y = 0.0;
    if p_pos.x > b_min.x && p_pos.x < b_max.x && p_pos.y > b_min.y && p_pos.y <= b_max.y{
        res_y = b_min.y - p_pos.y;        
    }
    return res_y
}
pub fn check_left_collide(p_pos: Vec2, b_min: Vec2, b_max: Vec2) -> f32{//pの左チェック
    let mut res_x = 0.0;
    if p_pos.x > b_min.x && p_pos.x <= b_max.x && p_pos.y > b_min.y && p_pos.y < b_max.y{
        res_x = b_max.x - p_pos.x;      
    }
    return res_x
}
pub fn check_right_collide(p_pos: Vec2, b_min: Vec2, b_max: Vec2) -> f32{//pの右チェック
    let mut res_x = 0.0;
    if p_pos.x >= b_min.x && p_pos.x < b_max.x && p_pos.y > b_min.y && p_pos.y < b_max.y{
        res_x = b_min.x - p_pos.x;        
    }
    return res_x
}
pub fn check_bottom_collide(p_pos: Vec2, b_min: Vec2, b_max: Vec2) -> f32{//pの下チェック
    let mut res_y = 0.0;
    if p_pos.x > b_min.x && p_pos.x < b_max.x && p_pos.y >= b_min.y && p_pos.y < b_max.y{
        res_y = b_max.y - p_pos.y;        
    }
    return res_y
}

pub fn check_left_bottom_collide(op_min: Vec2, op_max: Vec2, p_vec: Vec2, b_min: Vec2, b_max: Vec2) -> (f32, f32){//pの左下チェック
    let (mut res_x, mut res_y) = (0.0, 0.0);
    let p_min = op_min + p_vec;
    let _p_max = op_max + p_vec;
    if p_min.x < b_max.x && p_min.x > b_min.x && p_min.y > b_min.y && p_min.y < b_max.y{
        if op_max.y < b_min.y { res_x = b_max.x - p_min.x; }
        else                  { res_y = b_max.y - p_min.y; }
    }
    return (res_x, res_y);
}

pub fn check_right_bottom_collide(op_min: Vec2, op_max: Vec2, p_vec: Vec2, b_min: Vec2, b_max: Vec2) -> (f32, f32){//pの右下チェック
    let (mut res_x, mut res_y) = (0.0, 0.0);
    let p_min = op_min + p_vec;
    let p_max = op_max + p_vec;
    if p_max.x < b_max.x && p_max.x > b_min.x && p_min.y > b_min.y && p_min.y < b_max.y{
        if op_max.y < b_min.y { res_x = b_min.x - p_max.x; }
        else                  { res_y = b_max.y - p_min.y; }
    }
    return (res_x, res_y);
}

pub fn check_left_top_collide(op_min: Vec2, op_max: Vec2, p_vec: Vec2, b_min: Vec2, b_max: Vec2) -> (f32, f32){//pの左上チェック
    let (mut res_x, mut res_y) = (0.0, 0.0);
    let p_min = op_min + p_vec;
    let p_max = op_max + p_vec;
    if p_min.x < b_max.x && p_min.x > b_min.x && p_max.y > b_min.y && p_max.y < b_max.y{
        if op_max.y <= b_min.y { res_y = b_min.y - p_max.y; }
        else                   { res_x = b_max.x - p_min.x; }
    }
    return (res_x, res_y);
}

pub fn check_right_top_collide(op_min: Vec2, op_max: Vec2, p_vec: Vec2, b_min: Vec2, b_max: Vec2) -> (f32, f32){//pの右上チェック
    let (mut res_x, mut res_y) = (0.0, 0.0);
    let _p_min = op_min + p_vec;
    let p_max = op_max + p_vec;
    if p_max.x < b_max.x && p_max.x > b_min.x && p_max.y > b_min.y && p_max.y < b_max.y{
        if op_max.y <= b_min.y { res_y = b_min.y - p_max.y; }
        else                   { res_x = b_min.x - p_max.x; }
    }
    return (res_x, res_y);
}

pub fn check_for_collisions(
    is_side_hit: &mut bool,
    is_top_hit: &mut bool,
    is_rising: bool,
    is_ground: &mut bool,
    block_query: &Query<&Transform, With<state::game::BGBlock>>,
    player_adjustment: &mut Vec2,
    player_velocity: &mut Vec2,
    player_velocity_delta: &mut Vec2,
    p_min: Vec2,
    p_max: Vec2,
    op_min: Vec2,
    op_max: Vec2,
) {
    let mut hit_blocks = Vec::new();
    for  transform in block_query.iter() {//接触しているブロックのtransformを取得、接触しているブロックの色替え
        let b_min = transform.translation.truncate() - transform.scale.truncate() / 2.0;
        let b_max = transform.translation.truncate() + transform.scale.truncate() / 2.0; 
        if !is_in(p_min, p_max, b_min, b_max){continue;}
        hit_blocks.push(transform);
    }
    if hit_blocks.is_empty() { return;}
    let mut is_hit = false;
    for t in &hit_blocks{//上下左右の4面の接触判定
        let b_min = t.translation.truncate() - t.scale.truncate() / 2.0;
        let b_max = t.translation.truncate() + t.scale.truncate() / 2.0; 
        if !is_rising{
            let p_pos = Vec2::new((p_min.x + p_max.x)*0.5, p_min.y);//pの下チェック
            let ry = check_bottom_collide(p_pos, b_min, b_max);
            if ry != 0.0 && !is_rising{
                player_adjustment.y += ry;
                is_hit = true;
                if !*is_ground {
                    *is_ground = true;
                }
            }
        }
        
        let p_pos = Vec2::new((p_min.x + p_max.x)*0.5, p_max.y);//pの上チェック
        let ry = check_top_collide(p_pos, b_min, b_max);
        if ry != 0.0{
            *is_top_hit = true;
            player_adjustment.y += ry;
            is_hit = true;
        }
        let p_pos = Vec2::new(p_min.x, (p_min.y+p_max.y)*0.5);//pの左チェック
        let rx = check_left_collide(p_pos, b_min, b_max);
        if rx != 0.0{
            player_adjustment.x += rx;
            if !*is_ground{ 
                player_velocity.x = -player_velocity.x;
                player_velocity_delta.x = -player_velocity_delta.x;
            }
            *is_side_hit = true;
            is_hit = true;
        }
        let p_pos = Vec2::new(p_max.x, (p_min.y+p_max.y)*0.5);//pの右チェック
        let rx = check_right_collide(p_pos, b_min, b_max);
        if rx != 0.0{
            player_adjustment.x += rx;
            if !*is_ground{ 
                player_velocity.x = -player_velocity.x;
                player_velocity_delta.x = -player_velocity_delta.x;
            }
            *is_side_hit = true;
            is_hit = true;
        }
    }
    
    if is_hit { return; }
    for t in &hit_blocks {//各頂点の接触判定
        let b_min = t.translation.truncate() - t.scale.truncate() / 2.0;
        let b_max = t.translation.truncate() + t.scale.truncate() / 2.0; 
        let (rx, ry) = check_left_bottom_collide(op_min, op_max,*player_velocity_delta+*player_adjustment, b_min, b_max);
        if ry > 0.0 && !*is_ground && player_velocity.y < 0.0{ *is_ground = true; }
        if rx != 0.0{
            player_adjustment.x += rx;
            if !*is_ground{ 
                player_velocity.x = -player_velocity.x;
                player_velocity_delta.x = -player_velocity_delta.x;
            }
            *is_side_hit = true;
        }
        if !is_rising {player_adjustment.y += ry;}
        let (rx, ry) = check_right_bottom_collide(op_min, op_max,*player_velocity_delta+*player_adjustment, b_min, b_max);
        if ry > 0.0 && !*is_ground && player_velocity.y < 0.0{ *is_ground = true; }
        if rx != 0.0{
            player_adjustment.x += rx;
            if !*is_ground{ 
                player_velocity.x = -player_velocity.x;
                player_velocity_delta.x = -player_velocity_delta.x;
            }
            *is_side_hit = true;
        }
        if !is_rising {player_adjustment.y += ry;}
        let (rx, ry) = check_left_top_collide(op_min, op_max,*player_velocity_delta+*player_adjustment, b_min, b_max);
        if rx != 0.0{
            player_adjustment.x += rx;
            if !*is_ground{ 
                player_velocity.x = -player_velocity.x;
                player_velocity_delta.x = -player_velocity_delta.x;
            }
            *is_side_hit = true;
        }
        if ry != 0.0{ *is_top_hit = true; }
        player_adjustment.y += ry;
        let (rx, ry) = check_right_top_collide(op_min, op_max,*player_velocity_delta+*player_adjustment, b_min, b_max);
        if rx != 0.0{
            player_adjustment.x += rx;
            if !*is_ground{ 
                player_velocity.x = -player_velocity.x;
                player_velocity_delta.x = -player_velocity_delta.x;
            }
            *is_side_hit = true;
        }
        if ry != 0.0{ *is_top_hit = true; }
        player_adjustment.y += ry;
    }
}