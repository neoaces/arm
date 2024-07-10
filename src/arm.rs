use std::borrow::{Borrow, BorrowMut};

use log::debug;
use nannou::{
    color::{white_point::A, WHITE},
    geom::{pt2, Point2},
    Draw,
};

use crate::{
    constants::SCALE_FACTOR,
    couple::Couple,
    joint::{Joint, MotorType},
    link::Link,
    utils::rk4::solve_rk4,
};

/// The structs and stuff required to define an arm.
/// The basic arm consists of two things:
/// 1. The start joint
/// 2. The link
/// 3. The end joint (optional)
///
/// The link does not have coordinates, however the joints themselves have positions and angles.

#[derive(Debug)]
pub struct MissingLinkError;

pub struct Arm {
    couples: Vec<Couple>,
}

impl Arm {
    pub fn new(start: Point2, motor_type: MotorType, ratio: f32, m: f32, l: f32) -> Arm {
        let joint = Joint::new(start, motor_type, ratio);

        Arm {
            couples: vec![Couple {
                link: Link::new(m, l),
                joint,
            }],
        }
    }

    pub fn add_link(&mut self, m: f32, l: f32, s: f32, ratio: f32) {
        let s = self.couples.last().unwrap().joint.s;
        let link = Link { m, l };

        self.couples.push(Couple {
            joint: Joint {
                motor_type: MotorType::NEO550,
                s,
                v: 0.0,
                ratio,
                angle: 0.0,
            },
            link,
        })
    }

    pub fn calc(&mut self, current: f32, timestep: f32) {
        for (i, couple) in self.couples.iter_mut().enumerate() {
            debug!("Link {}. Angle {:?}", i + 1, couple.joint.angle);

            let f = couple.alpha();
            let alpha = solve_rk4(couple.joint.v, current, timestep, f);
            couple.joint.v = alpha * timestep;
            couple.joint.angle += couple.joint.v * timestep;
        }
    }

    pub fn draw_links(&self, draw: &Draw) {
        for couple in self.couples.iter() {
            let dim = pt2(
                (couple.joint.s.x + couple.link.l * couple.joint.angle.cos()) * SCALE_FACTOR,
                (couple.joint.s.y + couple.link.l * couple.joint.angle.sin()) * SCALE_FACTOR,
            );

            debug!("{:#?}", dim);

            draw.line()
                .start(couple.joint.s)
                .end(dim)
                .weight(10.0)
                .caps_round()
                .color(WHITE);
        }
    }

    pub fn set_link(&mut self, index: usize, length: f32) -> Result<(), MissingLinkError> {
        if let Some(a) = self.couples.get_mut(index) {
            a.link.l = length;
            Ok(())
        } else {
            Err(MissingLinkError {})
        }
    }

    // pub fn draw_trig(&self, draw: &Draw) {
    //     let intersect = pt2(
    //         self.start.x + (self.len() * self.joint.angle.cos()),
    //         self.start.y,
    //     );

    //     // First line: horizontal
    //     draw.line()
    //         .start(self.start)
    //         .end(intersect)
    //         .weight(3.0)
    //         .color(BLUE);

    //     // Second line: vertical
    //     draw.line()
    //         .start(intersect)
    //         .end(self.end)
    //         .weight(3.0)
    //         .color(RED);
    // }
}
