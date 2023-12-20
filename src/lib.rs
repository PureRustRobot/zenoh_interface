pub mod dual_shock_4;
pub mod motor_controll;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct CmdVel
{
    pub x:f32,
    pub y:f32,
    pub rotation_power:f32,
}
