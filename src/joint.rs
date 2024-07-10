use nannou::geom::{pt2, Point2};

#[derive(Debug)]
pub enum MotorType {
    NEO550, // The default motor
}

#[derive(Debug)]
pub struct Joint {
    pub motor_type: MotorType,
    pub s: Point2,
    pub v: f32,
    pub ratio: f32,
    pub angle: f32,
}

impl Joint {
    pub fn new(s: Point2, motor_type: MotorType, ratio: f32) -> Joint {
        Joint {
            motor_type,
            s,
            v: 0.0,
            ratio,
            angle: 0.0,
        }
    }
}

impl Default for Joint {
    fn default() -> Self {
        Joint::new(pt2(0.0, 0.0), MotorType::NEO550, 3.0)
    }
}
