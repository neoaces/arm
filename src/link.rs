use nannou::{
    color::{BLUE, RED, WHITE},
    geom::{pt2, Point2},
    Draw,
};

use crate::{
    constants::SCALE_FACTOR,
    joint::{Joint, MotorType},
    motor::neo550::{KT_550, KV_550, R_550},
};

#[derive(Debug)]
pub struct Link {
    pub m: f32,
    pub l: f32,
    pub start: Point2,
    pub end: Point2,
    pub angle: f32,
    pub joint: Joint,
}

impl Link {
    pub fn new(
        m: f32,
        l: f32,
        start: Point2,
        end: Point2,
        motor_type: Option<MotorType>,
        ratio: f32,
    ) -> Link {
        Link {
            m,
            l,
            start,
            end,
            angle: 0.0,
            joint: Joint {
                motor_type: motor_type.unwrap_or(MotorType::NEO550),
                v: 0.0,
                ratio,
            },
        }
    }

    pub fn draw_link(&self, draw: &Draw) {
        draw.line()
            .start(self.start)
            .end(self.end)
            .weight(10.0)
            .caps_round()
            .color(WHITE);
    }

    pub fn draw_trig(&self, draw: &Draw) {
        let intersect = pt2(self.start.x + (self.len() * self.angle.cos()), self.start.y);

        // First line: horizontal
        draw.line()
            .start(self.start)
            .end(intersect)
            .weight(3.0)
            .color(BLUE);

        // Second line: vertical
        draw.line()
            .start(intersect)
            .end(self.end)
            .weight(3.0)
            .color(RED);
    }

    pub fn alpha(&'_ self) -> impl Fn(f32, f32) -> f32 + '_ {
        move |v, u| {
            (-((self.joint.ratio.powi(2) * KT_550 * v) / (KV_550 * R_550 * self.moment())))
                + ((u * self.joint.ratio * KT_550) / self.moment())
        }
    }

    pub fn len(&self) -> f32 {
        f32::sqrt((self.end.x - self.start.x).powi(2) + (self.end.y - self.start.y).powi(2))
    }

    pub fn moment(&self) -> f32 {
        // Returns the moment of inertia for the rod at its end
        // Given that the formula for a rod on its end is J = 1/3 M * L^2
        (1.0 / 3.0) * self.m * self.l.powi(2)
    }
}
