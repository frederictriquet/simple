use macroquad::prelude::*;
extern crate korg_nano_kontrol_2;
extern crate midir;

// Import modules
mod metronome;
mod visual_effects;

use metronome::Metronome;
use visual_effects::{BeatBarsEffect, PulsingCircle, WaveEffect, SpiralEffect, VisualEffectComposite, ZoomFeedback};

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Metronome".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
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
        Color::from_rgba(255, 0, 255, 255)  // magenta
    )));

    // Add pulsing circle effect
    visual_effects.add_effect(Box::new(PulsingCircle::new(
        400.0,
        300.0,
        50.0,
        100.0,
        Color::from_rgba(0, 255, 255, 255)
    )));

    // Add wave effect
    visual_effects.add_effect(Box::new(WaveEffect::new(
        50.0,
        0.5,
        100.0,
        Color::from_rgba(255, 255, 0, 255)
    )));

    // Add spiral effect
    visual_effects.add_effect(Box::new(SpiralEffect::new(
        400.0,
        300.0,
        Color::from_rgba(0, 255, 128, 255)
    )));

    // Create zoom feedback effect
    // opacity: 0.92 = 92% persistence (creates trails and fade)
    // zoom_factor: 0.015 = 1.5% zoom per frame (creates visible zoom tunnel)
    let mut zoom_feedback = ZoomFeedback::new(0.99999, 0.1);

    loop {
        // Handle keyboard input
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }

        if is_key_pressed(KeyCode::Space) {
            metronome.reset();
        }

        if is_key_down(KeyCode::Up) {
            metronome.adjust_bpm(0.1);
        }

        if is_key_down(KeyCode::Down) {
            metronome.adjust_bpm(-0.1);
        }

        // Handle MIDI events
        let mut new_bpm: f32 = 0.0;
        loop {
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
                Err(_e) => break,
            }
        }
        metronome.set_bpm(new_bpm);

        let counter_copy = metronome.get_counter();
        let current_bpm = metronome.get_bpm();

        // Begin zoom feedback - switches to offscreen rendering
        // All subsequent drawing will be captured for the zoom effect
        zoom_feedback.begin_frame();

        // Render BPM text
        draw_text(
            &format!("BPM: {:.1}", current_bpm),
            10.0,
            30.0,
            32.0,
            WHITE
        );

        // Render all visual effects
        visual_effects.draw_all(counter_copy);

        // End zoom feedback - restores screen rendering and displays the result
        zoom_feedback.end_frame();

        next_frame().await;
    }

    // Cleanup MIDI connections
    for input in inputs {
        input.close();
    }
}

fn setup_midi() -> (Vec<midir::MidiInputConnection<()>>, std::sync::mpsc::Receiver<korg_nano_kontrol_2::Event>) {
    let midi_in = midir::MidiInput::new("Korg Nano Kontrol 2").unwrap();
    let (event_tx, event_rx) = std::sync::mpsc::channel();
    let mut inputs = Vec::new();

    // For each port used by the nano kontrol 2, check for events.
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
