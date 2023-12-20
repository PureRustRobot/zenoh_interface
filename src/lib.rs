use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Joy
{
    joy_left_x:u8,
    joy_right_x:u8,
    joy_left_y:u8,
    joy_right_y:u8,
}

#[derive(Serialize, Deserialize)]
pub struct CmdVel
{
    x:f32,
    y:f32,
    rotation_power:f32,
}