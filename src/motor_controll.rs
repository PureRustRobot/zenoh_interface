use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct  SingleMotor
{
    power:f32,
}

#[derive(Serialize, Deserialize)]
pub struct  DoubleMotor
{
    power_0:f32,
    power_1:f32,
}

#[derive(Serialize, Deserialize)]
pub struct  TripleMotor
{
    power_0:f32,
    power_1:f32,
    power_2:f32,
}

#[derive(Serialize, Deserialize)]
pub struct  QuadMotor
{
    power_0:f32,
    power_1:f32,
    power_2:f32,
    power_3:f32
}