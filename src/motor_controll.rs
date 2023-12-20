use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct  SingleMotor
{
    pub power:f32,
}

#[derive(Serialize, Deserialize)]
pub struct  DoubleMotor
{
    pub power_0:f32,
    pub power_1:f32,
}

#[derive(Serialize, Deserialize)]
pub struct  TripleMotor
{
    pub power_0:f32,
    pub power_1:f32,
    pub power_2:f32,
}

#[derive(Serialize, Deserialize)]
pub struct  QuadMotor
{
    pub power_0:f32,
    pub power_1:f32,
    pub power_2:f32,
    pub power_3:f32
}
