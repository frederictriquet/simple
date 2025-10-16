use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::video::Window;
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
    fn draw(&self, canvas: &mut Canvas<Window>, counter: f32) {
        canvas.set_draw_color(self.color);
        let radius = self.base_radius + (counter.fract() * self.pulse_amplitude);

        // Draw a pulsing circle
        for angle in 0..360 {
            let rad = (angle as f32).to_radians();
            let x = self.center_x + radius * rad.cos();
            let y = self.center_y + radius * rad.sin();
            canvas.draw_point((x as i32, y as i32)).unwrap();
        }
    }

    fn name(&self) -> &str {
        "Pulsing Circle"
    }
}
