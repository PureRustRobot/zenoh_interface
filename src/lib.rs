use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Joy
{
    joy_left_x:u8,
    joy_right_x:u8,
    joy_left_y:u8,
    joy_right_y:u8,
}