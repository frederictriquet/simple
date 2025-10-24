extern crate sdl3;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::{Canvas, TextureQuery};
use sdl3::video::Window;

use sdl3::rect::Rect;

use std::time::Duration;
extern crate korg_nano_kontrol_2;
extern crate midir;

// Import modules
mod metronome;
mod visual_effects;

use metronome::Metronome;
use visual_effects::{BeatBarsEffect, PulsingCircle, WaveEffect, SpiralEffect, VisualEffectComposite};

pub fn main() {
    let (inputs, event_rx) = setup_midi();

    let metronome = Metronome::new();
    metronome.start_counter_thread();

    // Create composite visual effect with multiple effects
    let mut visual_effects = VisualEffectComposite::new();

    // Add beat bars effect (the original draw_beat visualization)
    visual_effects.add_effect(Box::new(BeatBarsEffect::new(
        4,        // 4 bars for 4 beats
        200.0,    // 200px width per bar
        600.0,    // canvas height
        Color::RGB(255, 0, 255)  // magenta
    )));

    // Add pulsing circle effect
    visual_effects.add_effect(Box::new(PulsingCircle::new(
        400.0,
        300.0,
        50.0,
        100.0,
        Color::RGB(0, 255, 255)
    )));

    // Add wave effect
    visual_effects.add_effect(Box::new(WaveEffect::new(
        50.0,
        0.5,
        100.0,
        Color::RGB(255, 255, 0)
    )));

    // Add spiral effect
    visual_effects.add_effect(Box::new(SpiralEffect::new(
        400.0,
        300.0,
        Color::RGB(0, 255, 128)
    )));

    let sdl_context = sdl3::init().unwrap();
    let ttf_context = sdl3::ttf::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Try macOS font path first, then fall back to Linux
    let font = ttf_context.load_font("/System/Library/Fonts/Helvetica.ttc", 32.0)
        .or_else(|_| ttf_context.load_font("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 32.0))
        .unwrap();

    let window = video_subsystem
        .window("rust-sdl3 demo", 800, 600)
        .position_centered()
        // .fullscreen()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    metronome.reset();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    metronome.adjust_bpm(0.1);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    metronome.adjust_bpm(-0.1);
                }
                _ => {}
            }
        }
        
        let mut new_bpm: f32 = 0.0;
        'korg_event: loop {
            match event_rx.try_recv() {
                Ok(event) => match event {
                    korg_nano_kontrol_2::Event::VerticalSlider(
                        korg_nano_kontrol_2::Strip::A,
                        value,
                    ) => {
                        new_bpm = 110.0 + (value as f32) / 5.0;
                    }
                    _ => {}
                },
                Err(_e) => break 'korg_event,
            }
        }
        metronome.set_bpm(new_bpm);

        let counter_copy = metronome.get_counter();
        let current_bpm = metronome.get_bpm();

        render_frame(&mut canvas, &font, &visual_effects, counter_copy, current_bpm);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    for input in inputs {
        input.close();
    }
}

fn setup_midi() -> (Vec<midir::MidiInputConnection<()>>, std::sync::mpsc::Receiver<korg_nano_kontrol_2::Event>) {
    let midi_in = midir::MidiInput::new("Korg Nano Kontrol 2").unwrap();
    let (event_tx, event_rx) = std::sync::mpsc::channel();
    let mut inputs = Vec::new();

    // For each point used by the nano kontrol 2, check for events.
    for i in 0..midi_in.port_count() {
        let name = midi_in.port_name(i).unwrap();
        let event_tx = event_tx.clone();
        let midi_in = midir::MidiInput::new(&name).unwrap();
        let input = midi_in
            .connect(
                i,
                "nanoKONTROL2 SLIDER/KNOB",
                move |_stamp, msg, _| {
                    if let Some(event) = korg_nano_kontrol_2::Event::from_midi(msg) {
                        event_tx.send(event).unwrap();
                    }
                },
                (),
            )
            .unwrap();
        inputs.push(input);
    }
    
    (inputs, event_rx)
}

fn render_frame(canvas: &mut Canvas<Window>, font: &sdl3::ttf::Font, visual_effects: &VisualEffectComposite, counter: f32, bpm: f32) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // Render BPM text
    let surface = font
        .render(&format!("BPM: {:.1}", bpm))
        .blended(Color::RGB(255, 255, 255))
        .unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    let TextureQuery { width, height, .. } = texture.query();
    let target = Rect::new(10, 10, width, height);
    canvas.copy(&texture, None, target).unwrap();

    // Render all visual effects
    visual_effects.draw_all(canvas, counter);
}
