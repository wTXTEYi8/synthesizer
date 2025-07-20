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
    println!("\n🎮 インタラクティブ制御:");
    println!("'n' + Enter で中央のC音を再生");
    println!("'e' + Enter でE音を再生");
    println!("'g' + Enter でG音を再生");
    println!("'c' + Enter で高いC音を再生");
    println!("'s' + Enter で全ての音を停止");
    println!("'q' + Enter で終了");
    println!("'1-9' + Enter でブレンド比率変更 (1=Additive, 9=FM)");
    println!("'a' + Enter でエンベロープ調整");
    println!("'f' + Enter でフィルター調整");
    println!("'p' + Enter でアクティブな音を表示");
    println!("\n⏱️  カスタム持続時間:");
    println!("'C <秒数>' で中央のC音を指定時間再生 (例: 'C 2.5')");
    println!("'E <秒数>' でE音を指定時間再生 (例: 'E 1.8')");
    println!("'G <秒数>' でG音を指定時間再生 (例: 'G 0.3')");
    println!("'H <秒数>' で高いC音を指定時間再生 (例: 'H 4.2')");
    println!("'CHORD <秒数>' でC-E-G和音を指定時間再生 (例: 'CHORD 5.0')");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        // カスタム持続時間の処理
        if let Some((note, duration_str)) = parse_custom_duration(input) {
            match duration_str.parse::<f32>() {
                Ok(duration) if duration > 0.0 => {
                    let mut synth = synth.lock().unwrap();
                    match note {
                        "C" => {
                            synth.note_on_with_duration(60, 0.8, duration);
                            println!("🎵 Note ON: Middle C (60) for {:.1} seconds", duration);
                        }
                        "E" => {
                            synth.note_on_with_duration(64, 0.7, duration);
                            println!("🎵 Note ON: E (64) for {:.1} seconds", duration);
                        }
                        "G" => {
                            synth.note_on_with_duration(67, 0.6, duration);
                            println!("🎵 Note ON: G (67) for {:.1} seconds", duration);
                        }
                        "H" => {
                            synth.note_on_with_duration(72, 0.5, duration);
                            println!("🎵 Note ON: High C (72) for {:.1} seconds", duration);
                        }
                        "CHORD" => {
                            synth.note_on_with_duration(60, 0.8, duration);
                            synth.note_on_with_duration(64, 0.7, duration);
                            synth.note_on_with_duration(67, 0.6, duration);
                            println!("🎵 Chord ON: C-E-G for {:.1} seconds", duration);
                        }
                        _ => {
                            println!("❓ Unknown note: {}", note);
                        }
                    }
                }
                Ok(_) => {
                    println!("❌ Duration must be greater than 0");
                }
                Err(_) => {
                    println!("❌ Invalid duration format. Use numbers like 2.5, 1.8, etc.");
                }
            }
            continue;
        }
        
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
                println!("❓ Unknown command. Type 'n', 'e', 'g', 'c', 's', 'p', 'q', '1-9', 'a', 'f', or custom duration like 'C 2.5'");
            }
        }
    }
}

// カスタム持続時間のパース関数
fn parse_custom_duration(input: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() == 2 {
        Some((parts[0], parts[1]))
    } else {
        None
    }
}
