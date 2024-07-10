#[derive(Debug)]
pub struct Link {
    pub m: f32,
    pub l: f32,
}

impl Link {
    pub fn new(m: f32, l: f32) -> Link {
        Link { m, l }
    }

    pub fn moment(&self) -> f32 {
        // Returns the moment of inertia for the rod at its end
        // Given that the formula for a rod on its end is J = 1/3 M * L^2
        (1.0 / 3.0) * self.m * self.l.powi(2)
    }
}
