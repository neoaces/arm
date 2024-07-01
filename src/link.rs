use nannou::{color::WHITE, geom::Point2, Draw};

#[derive(Debug)]
pub struct Link {
    pub start: Point2,
    pub end: Point2,
    pub angle: f32
}

impl Link {
    pub fn draw_link(&self, draw: &Draw) {
        draw.line()
            .start(self.start)
            .end(self.end)
            .weight(3.0)
            .color(WHITE);
    }
}


