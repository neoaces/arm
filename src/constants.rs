use std::marker::PhantomData;

use nannou::color::Srgb;

pub const ARM_LENGTH: f32 = 200.0;
pub const DEFAULT_TIMESTEP: f32 = 0.025;
pub const INACTIVE_GREY: Srgb<u8> = Srgb::<u8> {
    red: 90,
    green: 90,
    blue: 90,
    standard: PhantomData,
};
