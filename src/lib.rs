use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct DS4JoyAxis
{
    joy_left_x:u8,
    joy_right_x:u8,
    joy_left_y:u8,
    joy_right_y:u8,
}

#[derive(Serialize, Deserialize)]
pub struct DS4Buttons
{
    joy_left:f32,
    joy_right:f32,
    circle:f32,
    cross:f32,
    square:f32,
    triangle:f32,
    up:f32,
    down:f32,
    right:f32,
    left:f32,
    _l1_:f32,
    _l2_:f32,
    _r1_:f32,
    _r2_:f32,
}

#[derive(Serialize, Deserialize)]
pub struct CmdVel
{
    x:f32,
    y:f32,
    rotation_power:f32,
}