extern crate sdl3;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::timer;
use sdl3::video::Window;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
extern crate korg_nano_kontrol_2;
extern crate midir;

pub fn main() {
    let midi_in = midir::MidiInput::new("Korg Nano Kontrol 2").unwrap();
    // A channel for sending events to the main thread.
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

    let counter: Arc<Mutex<f32>> = Arc::new(Mutex::new(0.0));
    let t0: Arc<Mutex<u64>> = Arc::new(Mutex::new(timer::ticks()));
    let bpm = Arc::new(Mutex::new(120.0));
    {
        let counter = Arc::clone(&counter);
        let t0 = Arc::clone(&t0);
        let bpm = Arc::clone(&bpm);
        thread::spawn(move || {
            loop {
                {
                    let t0 = *t0.lock().unwrap();
                    let bpm = *bpm.lock().unwrap();
                    // println!("thread bpm {bpm}");
                    let now = timer::ticks();
                    let mut counter = counter.lock().unwrap();

                    *counter = (now - t0) as f32 / 1000.0 / 60.0 * bpm;
                }
                thread::sleep(Duration::from_millis(10));
            }
        });
    }

    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl3 demo", 800, 600)
        .position_centered()
        // .fullscreen()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // let i: u8;
        let counter_copy;
        {
            counter_copy = *counter.lock().unwrap();
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        draw_beat(counter_copy, &mut canvas);
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
                    *counter.lock().unwrap() = 0.0;
                    *t0.lock().unwrap() = timer::ticks();
                }
                _ => {}
            }
        }

        // for event in &event_rx.try_recv() {
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
        {
            let mut bpm = bpm.lock().unwrap();
            if new_bpm > 100.0 && *bpm != new_bpm {
                println!("---{}", *bpm);
                *bpm = new_bpm;
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    for input in inputs {
        input.close();
    }
}

fn draw_beat(counter: f32, canvas: &mut Canvas<Window>) {
    let i = (counter.floor() as u64) % 4;
    // unsafe {
    //     SDL_RenderDebugText(SDL_GetRenderer(window),50.0, 50.0, );
    // }
    canvas.set_draw_color(Color::RGB(255, 0, 255));
    let rect = sdl3::render::FRect::new(
        200_f32 * (i as f32),
        599_f32,
        200_f32,
        -600_f32 * (1.0 - counter.fract()),
    );
    canvas.fill_rect(rect).unwrap();
}
