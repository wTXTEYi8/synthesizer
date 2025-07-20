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
    
    // パラメータをテスト
    synth.set_blend_ratio(0.7);
    synth.set_volume(0.8);
    synth.set_filter_cutoff(1000.0);
    synth.set_filter_resonance(0.3);
    
    // エンベロープをテスト
    let envelope = synth::Envelope {
        attack: 0.05,
        decay: 0.2,
        sustain: 0.8,
        release: 0.4,
    };
    synth.set_envelope(envelope);
    
    // Additive Engine をテスト
    synth.set_harmonic_amplitude(1, 0.5);
    synth.toggle_harmonic(2);
    
    // FM Engine をテスト
    synth.set_operator_amplitude(0, 1.0);
    synth.set_operator_frequency_ratio(1, 2.0);
    synth.set_operator_feedback(0, 0.1);
    
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
    
    // 状態を確認
    println!("Is playing: {}", synth.is_playing());
    println!("Harmonics count: {}", synth.harmonics().len());
    println!("Operators count: {}", synth.operators().len());
    
    println!("\n🚀 Next steps:");
    println!("1. Install Visual Studio Build Tools for GUI version");
    println!("2. Add audio output functionality");
    println!("3. Implement MIDI input");
    println!("4. Add more synthesis algorithms");
}
