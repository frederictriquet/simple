use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::video::Window;
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
    fn draw(&self, canvas: &mut Canvas<Window>, counter: f32) {
        canvas.set_draw_color(self.color);
        let rotation = counter * 2.0;

        // Draw a spiral that rotates with the beat
        for i in 0..500 {
            let t = i as f32 / 50.0;
            let radius = t * 20.0;
            let angle = t + rotation;
            let x = self.center_x + radius * angle.cos();
            let y = self.center_y + radius * angle.sin();
            canvas.draw_point((x as i32, y as i32)).unwrap();
        }
    }

    fn name(&self) -> &str {
        "Spiral Effect"
    }
}
