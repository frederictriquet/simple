use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::video::Window;
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
    fn draw(&self, canvas: &mut Canvas<Window>, counter: f32) {
        canvas.set_draw_color(self.color);
        let phase = counter * self.frequency;

        // Draw a sine wave
        for x in 0..800 {
            let x_normalized = x as f32 / 100.0;
            let y = self.y_offset + self.amplitude * ((x_normalized + phase).sin());
            canvas.draw_point((x, y as i32)).unwrap();
        }
    }

    fn name(&self) -> &str {
        "Wave Effect"
    }
}
