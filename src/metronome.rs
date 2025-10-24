use sdl3::timer;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Metronome {
    counter: Arc<Mutex<f32>>,
    t0: Arc<Mutex<u64>>,
    bpm: Arc<Mutex<f32>>,
}

impl Metronome {
    pub fn new() -> Self {
        Metronome {
            counter: Arc::new(Mutex::new(0.0)),
            t0: Arc::new(Mutex::new(timer::ticks())),
            bpm: Arc::new(Mutex::new(120.0)),
        }
    }

    pub fn start_counter_thread(&self) {
        let counter = Arc::clone(&self.counter);
        let t0 = Arc::clone(&self.t0);
        let bpm = Arc::clone(&self.bpm);

        thread::spawn(move || {
            loop {
                let t0_value = *t0.lock().unwrap();
                let bpm_value = *bpm.lock().unwrap();
                let now = timer::ticks();
                let new_counter_value = (now - t0_value) as f32 / 1000.0 / 60.0 * bpm_value;

                *counter.lock().unwrap() = new_counter_value;

                thread::sleep(Duration::from_millis(10));
            }
        });
    }

    pub fn reset(&self) {
        let now = timer::ticks();
        let mut counter = self.counter.lock().unwrap();
        let mut t0 = self.t0.lock().unwrap();
        *counter = 0.0;
        *t0 = now;
    }

    pub fn adjust_bpm(&self, delta: f32) {
        let mut bpm = self.bpm.lock().unwrap();
        *bpm = (*bpm + delta).clamp(30.0, 200.0);
    }

    pub fn set_bpm(&self, new_bpm: f32) {
        if new_bpm > 100.0 {
            let mut bpm = self.bpm.lock().unwrap();
            *bpm = new_bpm;
        }
    }

    pub fn get_counter(&self) -> f32 {
        *self.counter.lock().unwrap()
    }

    pub fn get_bpm(&self) -> f32 {
        *self.bpm.lock().unwrap()
    }
}
