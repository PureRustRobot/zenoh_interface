pub mod dual_shock_4;
pub mod motor_controll;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct CmdVel
{
    x:f32,
    y:f32,
    rotation_power:f32,
}