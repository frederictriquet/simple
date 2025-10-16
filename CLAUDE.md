# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based metronome application with a visual SDL3 interface and MIDI control support via Korg Nano Kontrol 2. The application uses SDL3 for graphics and event handling, SDL3-TTF for text rendering, and midir for MIDI input processing.

## Common Commands

### Build and Run
- Build: `cargo build`
- Run: `cargo run`
- Build for release: `cargo build --release`

### Development
- Check code: `cargo check`
- Run with full optimization: `cargo run --release`

## Architecture

### Core Components

1. **Metronome Struct** ([src/main.rs:19-80](src/main.rs#L19-L80))
   - Thread-safe counter using `Arc<Mutex<>>` for shared state
   - Runs a background thread that continuously updates the counter based on elapsed time and BPM
   - Supports BPM adjustment (clamped between 30-200 BPM) and reset functionality
   - Counter represents the current beat position (fractional value)

2. **MIDI Integration** ([src/main.rs:167-193](src/main.rs#L167-L193))
   - `setup_midi()` initializes MIDI connections and returns a channel receiver for events
   - Connects to all available MIDI ports to listen for Korg Nano Kontrol 2 events
   - Uses a multi-producer, single-consumer channel to communicate MIDI events to the main loop
   - Vertical Slider A on the controller maps to BPM (110-135 BPM range based on slider position)

3. **Main Event Loop** ([src/main.rs:107-161](src/main.rs#L107-L161))
   - Processes SDL keyboard events (Space to reset, Up/Down arrows to adjust BPM, Escape to quit)
   - Polls MIDI events via the channel without blocking
   - Renders at 60 FPS
   - Updates metronome BPM from MIDI slider input

4. **Rendering** ([src/main.rs:195-224](src/main.rs#L195-L224))
   - `draw_beat()`: Visualizes the current beat with a rising magenta bar (4 bars for 4 beats)
   - `render_frame()`: Clears screen, renders BPM text, and draws beat visualization
   - Uses SDL3 TTF to render text with system Helvetica font
   - Beat visualization uses fractional counter value to create smooth animations

### Concurrency Model

The metronome uses a separate thread for time tracking to ensure accurate timing independent of frame rate. The main thread handles:
- SDL event processing
- MIDI event polling
- Rendering

All shared state uses `Arc<Mutex<>>` for thread-safe access between the counter thread and main thread.

### Dependencies Note

- Uses SDL3 (version 0.x) with TTF feature enabled
- Uses `edition = "2024"` which is a newer Rust edition
- External git dependency for Korg Nano Kontrol 2 library
