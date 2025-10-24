use macroquad::prelude::*;
use super::GenericVisualEffect;

// Wave effect
pub struct WaveEffect {
    amplitude: f32,
    frequency: f32,
    color: Color,
    y_offset: f32,
}

impl WaveEffect {
    pub fn new(amplitude: f32, frequency: f32, y_offset: f32, color: Color) -> Self {
        Self {
            amplitude,
            frequency,
            y_offset,
            color,
        }
    }
}

impl GenericVisualEffect for WaveEffect {
    fn draw(&self, counter: f32) {
        let phase = counter * self.frequency;

        // Draw a sine wave using line segments
        let num_points = 800;
        for i in 0..num_points - 1 {
            let x1 = i as f32;
            let x_normalized1 = x1 / 100.0;
            let y1 = self.y_offset + self.amplitude * ((x_normalized1 + phase).sin());

            let x2 = (i + 1) as f32;
            let x_normalized2 = x2 / 100.0;
            let y2 = self.y_offset + self.amplitude * ((x_normalized2 + phase).sin());

            draw_line(x1, y1, x2, y2, 2.0, self.color);
        }
    }

    fn name(&self) -> &str {
        "Wave Effect"
    }
}
