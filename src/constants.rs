use std::marker::PhantomData;

use nannou::color::Srgb;

pub const DEFAULT_TIMESTEP: f32 = 0.025;
pub const INACTIVE_GREY: Srgb<u8> = Srgb::<u8> {
    red: 90,
    green: 90,
    blue: 90,
    standard: PhantomData,
};
pub const SCALE_FACTOR: f32 = 1000.0;
