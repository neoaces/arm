use nannou::{
    color::{BLUE, RED, WHITE},
    geom::{pt2, Point2},
    Draw,
};

#[derive(Debug)]
pub struct Link {
    pub start: Point2,
    pub end: Point2,
    pub angle: f32,
}

impl Link {
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

    pub fn len(&self) -> f32 {
        f32::sqrt((self.end.x - self.start.x).powi(2) + (self.end.y - self.start.y).powi(2))
    }
}
