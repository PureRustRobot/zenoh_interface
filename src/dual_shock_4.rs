use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Axis
{
    pub joy_left_x:f32,
    pub joy_right_x:f32,
    pub joy_left_y:f32,
    pub joy_right_y:f32,
}

#[derive(Serialize, Deserialize)]
pub struct Buttons
{
    pub joy_left:f32,
    pub joy_right:f32,
    pub circle:f32,
    pub cross:f32,
    pub square:f32,
    pub triangle:f32,
    pub up:f32,
    pub down:f32,
    pub right:f32,
    pub left:f32,
    pub _l1_:f32,
    pub _l2_:f32,
    pub _r1_:f32,
    pub _r2_:f32,
}
