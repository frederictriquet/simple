use sdl3::render::Canvas;
use sdl3::video::Window;
use super::GenericVisualEffect;

// Container for multiple effects
pub struct VisualEffectComposite {
    effects: Vec<Box<dyn GenericVisualEffect>>,
}

impl VisualEffectComposite {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }

    pub fn add_effect(&mut self, effect: Box<dyn GenericVisualEffect>) {
        self.effects.push(effect);
    }

    pub fn draw_all(&self, canvas: &mut Canvas<Window>, counter: f32) {
        for effect in &self.effects {
            effect.draw(canvas, counter);
        }
    }
}
