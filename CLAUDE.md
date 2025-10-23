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

1. **Metronome Struct** (src/main.rs:22-83)
   - Thread-safe counter using `Arc<Mutex<>>` for shared state
   - Runs a background thread that continuously updates the counter based on elapsed time and BPM
   - Supports BPM adjustment (clamped between 30-200 BPM) and reset functionality
   - Counter represents the current beat position (fractional value)

2. **Visual Effects System** (src/visual_effects/)
   - Uses trait-based architecture with `GenericVisualEffect` trait for pluggable effects
   - `VisualEffectComposite` manages multiple effects that render simultaneously
   - Each effect implements `draw()` method that takes canvas and counter value
   - Current effects: Beat Bars, Pulsing Circle, Wave, and Spiral
   - Effects are composed in main.rs (lines 92-124) before the main loop

3. **MIDI Integration** (src/main.rs:205-231)
   - `setup_midi()` initializes MIDI connections and returns a channel receiver for events
   - Connects to all available MIDI ports to listen for Korg Nano Kontrol 2 events
   - Uses a multi-producer, single-consumer channel to communicate MIDI events to the main loop
   - Vertical Slider A on the controller maps to BPM (110-135 BPM range based on slider position)

4. **Main Event Loop** (src/main.rs:145-199)
   - Processes SDL keyboard events (Space to reset, Up/Down arrows to adjust BPM, Escape to quit)
   - Polls MIDI events via the channel without blocking
   - Renders at 60 FPS
   - Updates metronome BPM from MIDI slider input

5. **Rendering** (src/main.rs:233-250)
   - `render_frame()`: Clears screen, renders BPM text, and delegates to visual effects
   - Uses SDL3 TTF to render text with system Helvetica font
   - All visual effects render through `visual_effects.draw_all()` which iterates over registered effects

### Visual Effects Architecture

The visual effects system uses a trait-based plugin architecture:
- `GenericVisualEffect` trait defines the interface for all effects
- Each effect is a separate module in `src/visual_effects/`
- Effects receive the metronome counter value and render based on beat position
- `VisualEffectComposite` acts as a container that renders multiple effects in sequence
- New effects can be added by implementing the `GenericVisualEffect` trait

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
