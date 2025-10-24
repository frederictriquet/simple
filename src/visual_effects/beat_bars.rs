use macroquad::prelude::*;
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
    fn draw(&self, counter: f32) {
        let i = (counter.floor() as u64) % self.num_bars as u64;
        let x = self.bar_width * (i as f32);
        let height = self.canvas_height * counter.fract();
        let y = self.canvas_height - height;

        draw_rectangle(x, y, self.bar_width, height, self.color);
    }

    fn name(&self) -> &str {
        "Beat Bars"
    }
}
