#[derive(Debug)]
pub enum MotorType {
    NEO550, // The default motor
}

#[derive(Debug)]
pub struct Joint {
    pub motor_type: MotorType,
    pub v: f32,
    pub ratio: f32,
}

impl Joint {
    pub fn new(motor_type: MotorType, ratio: f32) -> Joint {
        Joint {
            motor_type,
            v: 0.0,
            ratio,
        }
    }
}

impl Default for Joint {
    fn default() -> Self {
        Joint::new(MotorType::NEO550, 3.0)
    }
}
