use crate::motor::neo550::{KT_550, KV_550, R_550};
use crate::{joint::Joint, link::Link};
use nannou::geom::Point2;
pub struct Couple {
    pub joint: Joint,
    pub link: Link,
}

impl Couple {
    pub fn state(&self) -> CoupleState {
        CoupleState {
            s: self.joint.s,
            v: self.joint.v,
            u: Some(12.0),
            ratio: self.joint.ratio,
            moi: self.link.moment(),
        }
    }

    pub fn alpha(&self) -> impl Fn(f32, f32) -> f32 + '_ {
        move |v, u| {
            (-((self.joint.ratio.powi(2) * KT_550 * v) / (KV_550 * R_550 * self.link.moment())))
                + ((u * self.joint.ratio * KT_550) / self.link.moment())
        }
    }
}

pub struct CoupleState {
    pub s: Point2,
    pub v: f32,
    pub u: Option<f32>, // Generally can be used for the current of the system
    pub ratio: f32,
    pub moi: f32, // The moment of inertia
}

impl CoupleState {
    pub fn new(s: Point2, v: f32, u: Option<f32>, ratio: f32, moi: f32) -> CoupleState {
        CoupleState {
            s,
            v,
            u,
            ratio,
            moi,
        }
    }
}
