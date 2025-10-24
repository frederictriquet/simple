use macroquad::prelude::*;
use super::GenericVisualEffect;

// Zoom feedback effect using ping-pong render targets
//
// Uses two render targets to avoid circular texture dependency:
// 1. Read from texture A, render zoomed to screen
// 2. Draw new effects on screen
// 3. Copy screen contents to texture B
// 4. Swap A and B for next frame
pub struct ZoomFeedback {
    texture_a: Option<RenderTarget>,
    texture_b: Option<RenderTarget>,
    use_a_as_source: bool,  // true = read from A, write to B; false = read from B, write to A
    opacity: f32,      // 0.0-1.0, how much previous frame persists
    zoom_factor: f32,  // how much to zoom per frame (e.g., 0.02 = 2%)
}

impl ZoomFeedback {
    pub fn new(opacity: f32, zoom_factor: f32) -> Self {
        Self {
            texture_a: None,
            texture_b: None,
            use_a_as_source: true,
            opacity,
            zoom_factor,
        }
    }

    fn ensure_render_targets(&mut self) {
        if self.texture_a.is_none() {
            self.texture_a = Some(render_target(800, 600));
            self.texture_a.as_ref().unwrap().texture.set_filter(FilterMode::Linear);
        }
        if self.texture_b.is_none() {
            self.texture_b = Some(render_target(800, 600));
            self.texture_b.as_ref().unwrap().texture.set_filter(FilterMode::Linear);
        }
    }

    // Call this at the START to switch rendering to the destination texture
    pub fn begin_frame(&mut self) {
        self.ensure_render_targets();

        // Set up rendering to the destination texture
        push_camera_state();

        let dest_rt = if self.use_a_as_source {
            self.texture_b.as_ref().unwrap()
        } else {
            self.texture_a.as_ref().unwrap()
        };

        let camera = Camera2D {
            zoom: vec2(1.0 / 400.0, -1.0 / 300.0),
            target: vec2(400.0, 300.0),
            render_target: Some(dest_rt.clone()),
            ..Default::default()
        };
        set_camera(&camera);

        // Clear with slight fade
        let fade_alpha = 1.0 - self.opacity;
        clear_background(Color::new(0.0, 0.0, 0.0, fade_alpha));

        // Draw the previous frame zoomed
        let source_texture = if self.use_a_as_source {
            &self.texture_a.as_ref().unwrap().texture
        } else {
            &self.texture_b.as_ref().unwrap().texture
        };

        let scale = 1.0 + self.zoom_factor;
        let w = 800.0;
        let h = 600.0;
        let offset_x = -w * self.zoom_factor / 2.0;
        let offset_y = -h * self.zoom_factor / 2.0;

        draw_texture_ex(
            source_texture,
            offset_x,
            offset_y,
            Color::new(1.0, 1.0, 1.0, self.opacity),
            DrawTextureParams {
                dest_size: Some(vec2(w * scale, h * scale)),
                flip_y: true,  // Flip texture vertically for correct orientation
                ..Default::default()
            },
        );

        // Now all subsequent drawing (text, effects) will go to the destination texture
    }

    // Call this at the END to restore screen rendering and display result
    pub fn end_frame(&mut self) {
        // Restore screen rendering
        pop_camera_state();

        // Swap buffers for next frame
        self.use_a_as_source = !self.use_a_as_source;

        // Draw the result texture to screen
        let source_texture = if self.use_a_as_source {
            &self.texture_a.as_ref().unwrap().texture
        } else {
            &self.texture_b.as_ref().unwrap().texture
        };

        draw_texture_ex(
            source_texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,  // Flip texture vertically for correct orientation
                ..Default::default()
            },
        );
    }
}

impl GenericVisualEffect for ZoomFeedback {
    fn draw(&self, _counter: f32) {
        // This effect is managed via begin_frame/end_frame
    }

    fn name(&self) -> &str {
        "Zoom Feedback"
    }
}
