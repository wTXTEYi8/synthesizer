# 🎹 Rust Synthesizer

A real-time synthesizer built in Rust with rich sound expression using Additive + FM synthesis.

## 🎵 Features

- **Additive Synthesis**: 64 harmonics for rich, complex tones
- **FM Synthesis**: 6 operators for dynamic, evolving sounds
- **Engine Blending**: Smooth crossfading between Additive and FM engines
- **ADSR Envelope**: Attack, Decay, Sustain, Release control
- **Low-pass Filter**: Cutoff and resonance control
- **Real-time Audio Output**: Using cpal crate
- **Polyphonic Support**: Multiple simultaneous notes
- **Custom Note Duration**: Specify exact duration for each note
- **Interactive Command-line Interface**: Japanese localized controls

## 🎮 Interactive Controls

### Basic Note Controls (Continuous Play)
- **`c` + Enter**: Middle C (60)
- **`d` + Enter**: D (62)
- **`e` + Enter**: E (64)
- **`f` + Enter**: F (65)
- **`g` + Enter**: G (67)
- **`a` + Enter**: A (69)
- **`b` + Enter**: B (71)
- **`s` + Enter**: Stop all notes
- **`q` + Enter**: Quit

### Custom Duration Controls
- **`C <seconds>`**: Middle C for specified duration (e.g., `C 2.5`)
- **`D <seconds>`**: D for specified duration (e.g., `D 1.8`)
- **`E <seconds>`**: E for specified duration (e.g., `E 1.8`)
- **`F <seconds>`**: F for specified duration (e.g., `F 0.3`)
- **`G <seconds>`**: G for specified duration (e.g., `G 0.3`)
- **`A <seconds>`**: A for specified duration (e.g., `A 4.2`)
- **`B <seconds>`**: B for specified duration (e.g., `B 4.2`)
- **`H <seconds>`**: High C for specified duration (e.g., `H 4.2`)
- **`CHORD <seconds>`**: C-E-G chord for specified duration (e.g., `CHORD 5.0`)
- **`SCALE <seconds>`**: C-D-E-F-G-A-B-C scale for specified duration (e.g., `SCALE 8.0`)

### Sound Shaping Controls
- **`1-9` + Enter**: Blend ratio (1=Additive, 9=FM)
- **`env` + Enter**: Adjust envelope settings
- **`filter` + Enter**: Adjust filter settings
- **`p` + Enter**: Show active voices

## 🎼 Musical Scale

The synthesizer supports a complete octave:
```
C (60) - D (62) - E (64) - F (65) - G (67) - A (69) - B (71)
```

## 🚀 Getting Started

### Prerequisites
- Rust (latest stable version)
- Visual Studio Build Tools (Windows)

### Installation
1. Clone the repository:
```bash
git clone <repository-url>
cd synthesizer
```

2. Build and run:
```bash
cargo run
```

### Usage Examples

#### Playing a Scale
```
> c    ← Start C
> d    ← Add D
> e    ← Add E
> f    ← Add F
> g    ← Add G
> a    ← Add A
> b    ← Add B
```

#### Custom Duration
```
> C 2.5    ← Play C for 2.5 seconds
> D 1.8    ← Play D for 1.8 seconds
> CHORD 5.0 ← Play C-E-G chord for 5 seconds
> SCALE 8.0 ← Play complete scale for 8 seconds
```

#### Sound Shaping
```
> 1        ← Pure Additive synthesis
> 9        ← Pure FM synthesis
> 5        ← 50/50 blend
> env      ← Adjust envelope
> filter   ← Adjust filter
```

## 🏗️ Architecture

- **`src/main.rs`**: Interactive command-line interface
- **`src/synth.rs`**: Main synthesizer with polyphonic voice management
- **`src/engine.rs`**: Additive and FM synthesis engines
- **`src/audio.rs`**: Real-time audio output using cpal

## 🎛️ Technical Details

### Polyphonic Voice Management
- Each note is managed as a separate `Voice` instance
- Automatic note-off after specified duration
- Real-time voice allocation and deallocation

### Synthesis Engines
- **Additive**: 64 harmonics with individual amplitude control
- **FM**: 6 operators with frequency ratios and feedback
- **Blending**: Smooth crossfading between engines

### Audio Processing
- Real-time sample generation at 48kHz
- Support for multiple audio formats
- Low-latency audio output

## 🔧 Development

### Building
```bash
cargo build
```

### Running Tests
```bash
cargo test
```

### Code Structure
```
src/
├── main.rs      # Entry point and CLI
├── synth.rs     # Synthesizer core
├── engine.rs    # Synthesis engines
└── audio.rs     # Audio output
```

## 🎵 Future Enhancements

- MIDI input support
- GUI interface
- Audio effects (reverb, delay, chorus)
- Preset management
- Score playback
- MIDI file support

## 📝 License

This project is open source and available under the MIT License.
