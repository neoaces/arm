use nannou::math::num_traits::clamp;

use super::Motor;

pub const KT_550: f32 = 0.0097; // T_stall / I_stall
pub const KV_550: f32 = 917.0; // From datasheet
pub const R_550: f32 = 12.0 / 100.0; // Nominal voltage / I_stall

// *NOTE moment of inertia of a rod about the end is 1/3 * M * L^2
// where M is the mass and L is the radius from the axis
// *NOTE also just remember, the voltage is ideally constant,
// but the current is used to control the speed.
struct NEO550 {
    omega: f32,
}

impl Motor for NEO550 {
    /// Given a constant velocity of 12...
    fn move_motor(self, current: f32, j: f32, omega: f32) -> f32 {
        let i = clamp(current, 0.0, 100.0);
        // The equation is V = t/kt * r + w / kv
        (12.0 - i * R_550) * KV_550
    }
}

impl Default for NEO550 {
    fn default() -> Self {
        NEO550 { omega: 0.0 }
    }
}
