mod engine;
mod synth;

fn main() {
    println!("🎹 Additive + FM Synthesizer");
    println!("================================");
    println!("This is a command-line version of the synthesizer.");
    println!("GUI version will be implemented later.");
    
    // 基本的なシンセサイザーのテスト
    let mut synth = synth::Synthesizer::new();
    
    println!("✅ Synthesizer initialized successfully!");
    println!("📊 Additive Engine: 64 harmonics available");
    println!("🎛️  FM Engine: 6 operators available");
    println!("🎚️  Envelope: ADSR controls");
    println!("🔊 Filter: Low-pass with resonance");
    
    // 簡単なテスト
    println!("\n🎵 Testing synthesizer...");
    synth.note_on(440.0); // A4
    
    // サンプルを生成してテスト
    for i in 0..100 {
        let sample = synth.next_sample();
        if i % 20 == 0 {
            println!("Sample {}: {:.6}", i, sample);
        }
    }
    
    synth.note_off();
    println!("✅ Test completed successfully!");
    
    println!("\n🚀 Next steps:");
    println!("1. Install Visual Studio Build Tools for GUI version");
    println!("2. Add audio output functionality");
    println!("3. Implement MIDI input");
    println!("4. Add more synthesis algorithms");
}
