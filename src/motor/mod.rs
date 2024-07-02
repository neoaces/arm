pub mod neo550;

pub trait Motor {
    /// Gives the angular velocity of the motor.
    fn move_motor(self, current: f32, moi: f32, omega: f32) -> f32;
}
