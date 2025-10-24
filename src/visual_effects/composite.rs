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

    pub fn draw_all(&self, counter: f32) {
        // Draw all effects sequentially
        for effect in &self.effects {
            effect.draw(counter);
        }
    }
}
