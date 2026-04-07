use crate::AudioComponent;

pub struct Smooth {
    previous_output: f32,
    pole: f32,
}

impl Smooth {
    // use Option to provide default pole value
    pub fn new(pole_or_none: impl Into<Option<f32>>) -> Self {
        let pole = pole_or_none.into().unwrap_or(0.999);
        Smooth {
            previous_output: 0.0,
            pole,
        }
    }
}

impl AudioComponent for Smooth {
    fn tick(&mut self, in_frame: f32) -> f32 {
        let output = (1.0 - self.pole) * in_frame + self.pole * self.previous_output;
        self.previous_output = output;
        output
    }
}