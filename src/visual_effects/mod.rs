use sdl3::render::Canvas;
use sdl3::video::Window;

// Generic trait for visual effects
pub trait GenericVisualEffect {
    fn draw(&self, canvas: &mut Canvas<Window>, counter: f32);
    fn name(&self) -> &str;
}

// Re-export all visual effect implementations
pub mod beat_bars;
pub mod pulsing_circle;
pub mod wave;
pub mod spiral;
pub mod composite;

pub use beat_bars::BeatBarsEffect;
pub use pulsing_circle::PulsingCircle;
pub use wave::WaveEffect;
pub use spiral::SpiralEffect;
pub use composite::VisualEffectComposite;
