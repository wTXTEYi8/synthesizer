mod engine;
mod synth;
mod audio;

use std::sync::{Arc, Mutex};
use std::io::{self, Write};

fn main() {
    println!("🎹 Additive + FM Synthesizer");
    println!("================================");
    
    // Initialize synthesizer
    let mut synth = synth::Synthesizer::new();
    println!("✅ Synthesizer initialized successfully!");
    
    // Test synthesizer functionality
    test_synthesizer(&mut synth);
    
    // Create thread-safe synthesizer for audio
    let synth_arc = Arc::new(Mutex::new(synth));
    
    // Initialize audio output
    match audio::AudioOutput::new(Arc::clone(&synth_arc)) {
        Ok(mut audio) => {
            println!("\n🎵 Starting audio output...");
            if let Err(e) = audio.start() {
                eprintln!("❌ Failed to start audio: {}", e);
                return;
            }
            
            // Interactive control loop
            interactive_control(Arc::clone(&synth_arc), &mut audio);
        }
        Err(e) => {
            eprintln!("❌ Failed to initialize audio: {}", e);
            println!("🔧 Audio output not available, running in test mode only");
        }
    }
}

fn test_synthesizer(synth: &mut synth::Synthesizer) {
    println!("📊 Additive Engine: 64 harmonics available");
    println!("🎛️  FM Engine: 6 operators available");
    println!("🎚️  Envelope: ADSR controls");
    println!("🔊 Filter: Low-pass with resonance");
    
    println!("\n🎵 Testing synthesizer...");
    
    // Test sample generation
    for i in 0..100 {
        if i % 20 == 0 {
            let sample = synth.next_sample();
            println!("Sample {}: {:.6}", i, sample);
        } else {
            synth.next_sample();
        }
    }
    
    println!("✅ Test completed successfully!");
    println!("Is playing: {}", synth.is_playing());
    println!("Harmonics count: {}", synth.harmonics_count());
    println!("Operators count: {}", synth.operators_count());
}

fn interactive_control(synth: Arc<Mutex<synth::Synthesizer>>, _audio: &mut audio::AudioOutput) {
    println!("\n🎮 Interactive Controls:");
    println!("Press 'n' + Enter to play Middle C");
    println!("Press 'e' + Enter to play E note");
    println!("Press 'g' + Enter to play G note");
    println!("Press 'c' + Enter to play high C");
    println!("Press 's' + Enter to stop all notes");
    println!("Press 'q' + Enter to quit");
    println!("Press '1-9' + Enter to change blend (1=Additive, 9=FM)");
    println!("Press 'a' + Enter to adjust envelope");
    println!("Press 'f' + Enter to adjust filter");
    println!("Press 'p' + Enter to show active voices");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        match input {
            "n" => {
                let mut synth = synth.lock().unwrap();
                synth.note_on(60, 0.8); // Middle C
                println!("🎵 Note ON: Middle C (60)");
            }
            "e" => {
                let mut synth = synth.lock().unwrap();
                synth.note_on(64, 0.7); // E
                println!("🎵 Note ON: E (64)");
            }
            "g" => {
                let mut synth = synth.lock().unwrap();
                synth.note_on(67, 0.6); // G
                println!("🎵 Note ON: G (67)");
            }
            "c" => {
                let mut synth = synth.lock().unwrap();
                synth.note_on(72, 0.5); // High C
                println!("🎵 Note ON: High C (72)");
            }
            "s" => {
                let mut synth = synth.lock().unwrap();
                // Stop all active notes
                let active_notes: Vec<u8> = synth.voices.keys().cloned().collect();
                for note in active_notes {
                    synth.note_off(note);
                }
                println!("🔇 All notes stopped");
            }
            "p" => {
                let synth = synth.lock().unwrap();
                let active_voices: Vec<u8> = synth.voices.iter()
                    .filter(|(_, voice)| voice.is_active())
                    .map(|(note, _)| *note)
                    .collect();
                if active_voices.is_empty() {
                    println!("📊 No active voices");
                } else {
                    println!("📊 Active voices: {:?}", active_voices);
                }
            }
            "q" => {
                println!("👋 Goodbye!");
                break;
            }
            "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                let blend = (input.parse::<f32>().unwrap() - 1.0) / 8.0;
                let mut synth = synth.lock().unwrap();
                synth.set_blend(blend);
                println!("🎛️  Blend set to: {:.2}", blend);
            }
            "a" => {
                let mut synth = synth.lock().unwrap();
                synth.set_attack(0.1);
                synth.set_decay(0.2);
                synth.set_sustain(0.7);
                synth.set_release(0.3);
                println!("🎚️  Envelope adjusted");
            }
            "f" => {
                let mut synth = synth.lock().unwrap();
                synth.set_cutoff(0.5);
                synth.set_resonance(0.3);
                println!("🔊 Filter adjusted");
            }
            _ => {
                println!("❓ Unknown command. Type 'n', 'e', 'g', 'c', 's', 'p', 'q', '1-9', 'a', or 'f'");
            }
        }
    }
}
