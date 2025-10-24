use macroquad::prelude::*;
use super::GenericVisualEffect;

// Spiral effect
pub struct SpiralEffect {
    center_x: f32,
    center_y: f32,
    color: Color,
}

impl SpiralEffect {
    pub fn new(center_x: f32, center_y: f32, color: Color) -> Self {
        Self {
            center_x,
            center_y,
            color,
        }
    }
}

impl GenericVisualEffect for SpiralEffect {
    fn draw(&self, counter: f32) {
        let rotation = counter * 2.0;

        // Draw a spiral that rotates with the beat
        for i in 0..499 {
            let t1 = i as f32 / 50.0;
            let radius1 = t1 * 20.0;
            let angle1 = t1 + rotation;
            let x1 = self.center_x + radius1 * angle1.cos();
            let y1 = self.center_y + radius1 * angle1.sin();

            let t2 = (i + 1) as f32 / 50.0;
            let radius2 = t2 * 20.0;
            let angle2 = t2 + rotation;
            let x2 = self.center_x + radius2 * angle2.cos();
            let y2 = self.center_y + radius2 * angle2.sin();

            draw_line(x1, y1, x2, y2, 2.0, self.color);
        }
    }

    fn name(&self) -> &str {
        "Spiral Effect"
    }
}
