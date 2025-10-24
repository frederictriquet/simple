use macroquad::prelude::*;

// Generic trait for visual effects
pub trait GenericVisualEffect {
    fn draw(&self, counter: f32);
    fn name(&self) -> &str;
}

// Re-export all visual effect implementations
pub mod beat_bars;
pub mod pulsing_circle;
pub mod wave;
pub mod spiral;
pub mod zoom_feedback;
pub mod composite;

pub use beat_bars::BeatBarsEffect;
pub use pulsing_circle::PulsingCircle;
pub use wave::WaveEffect;
pub use spiral::SpiralEffect;
pub use zoom_feedback::ZoomFeedback;
pub use composite::VisualEffectComposite;
