use macroquad::prelude::*;
use super::GenericVisualEffect;

// Pulsing circle effect
pub struct PulsingCircle {
    center_x: f32,
    center_y: f32,
    base_radius: f32,
    pulse_amplitude: f32,
    color: Color,
}

impl PulsingCircle {
    pub fn new(center_x: f32, center_y: f32, base_radius: f32, pulse_amplitude: f32, color: Color) -> Self {
        Self {
            center_x,
            center_y,
            base_radius,
            pulse_amplitude,
            color,
        }
    }
}

impl GenericVisualEffect for PulsingCircle {
    fn draw(&self, counter: f32) {
        let radius = self.base_radius + (counter.fract() * self.pulse_amplitude);
        draw_circle(self.center_x, self.center_y, radius, self.color);
    }

    fn name(&self) -> &str {
        "Pulsing Circle"
    }
}
