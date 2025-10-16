use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::video::Window;
use super::GenericVisualEffect;

// Beat bars effect - rising bars for each beat
pub struct BeatBarsEffect {
    num_bars: u32,
    bar_width: f32,
    canvas_height: f32,
    color: Color,
}

impl BeatBarsEffect {
    pub fn new(num_bars: u32, bar_width: f32, canvas_height: f32, color: Color) -> Self {
        Self {
            num_bars,
            bar_width,
            canvas_height,
            color,
        }
    }
}

impl GenericVisualEffect for BeatBarsEffect {
    fn draw(&self, canvas: &mut Canvas<Window>, counter: f32) {
        let i = (counter.floor() as u64) % self.num_bars as u64;
        canvas.set_draw_color(self.color);
        let rect = sdl3::render::FRect::new(
            self.bar_width * (i as f32),
            self.canvas_height - 1.0,
            self.bar_width,
            -self.canvas_height * (1.0 - counter.fract()),
        );
        canvas.fill_rect(rect).unwrap();
    }

    fn name(&self) -> &str {
        "Beat Bars"
    }
}
