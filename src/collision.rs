use bevy::prelude::*;

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