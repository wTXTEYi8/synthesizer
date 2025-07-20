use crate::engine::{EngineBlender, Harmonic, Operator};
use std::collections::HashMap;

// エンベロープ
#[derive(Debug, Clone, Copy)]
pub struct Envelope {
    pub attack: f32,   // 秒
    pub decay: f32,    // 秒
    pub sustain: f32,  // 0.0-1.0
    pub release: f32,  // 秒
}

impl Default for Envelope {
    fn default() -> Self {
        Self {
            attack: 0.01,
            decay: 0.1,
            sustain: 0.7,
            release: 0.2,
        }
    }
}

pub struct EnvelopeGenerator {
    envelope: Envelope,
    sample_rate: f32,
    current_stage: EnvelopeStage,
    current_time: f32,
    current_value: f32,
    gate: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum EnvelopeStage {
    Attack,
    Decay,
    Sustain,
    Release,
    Idle,
}

impl EnvelopeGenerator {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            envelope: Envelope::default(),
            sample_rate,
            current_stage: EnvelopeStage::Idle,
            current_time: 0.0,
            current_value: 0.0,
            gate: false,
        }
    }
    
    pub fn set_envelope(&mut self, envelope: Envelope) {
        self.envelope = envelope;
    }
    
    pub fn note_on(&mut self) {
        self.gate = true;
        self.current_stage = EnvelopeStage::Attack;
        self.current_time = 0.0;
    }
    
    pub fn note_off(&mut self) {
        self.gate = false;
        self.current_stage = EnvelopeStage::Release;
        self.current_time = 0.0;
    }
    
    pub fn next_sample(&mut self) -> f32 {
        match self.current_stage {
            EnvelopeStage::Attack => {
                self.current_time += 1.0 / self.sample_rate;
                if self.current_time >= self.envelope.attack {
                    self.current_stage = EnvelopeStage::Decay;
                    self.current_time = 0.0;
                    self.current_value = 1.0;
                } else {
                    self.current_value = self.current_time / self.envelope.attack;
                }
            }
            EnvelopeStage::Decay => {
                self.current_time += 1.0 / self.sample_rate;
                if self.current_time >= self.envelope.decay {
                    self.current_stage = EnvelopeStage::Sustain;
                    self.current_value = self.envelope.sustain;
                } else {
                    let decay_progress = self.current_time / self.envelope.decay;
                    self.current_value = 1.0 - (1.0 - self.envelope.sustain) * decay_progress;
                }
            }
            EnvelopeStage::Sustain => {
                if !self.gate {
                    self.current_stage = EnvelopeStage::Release;
                    self.current_time = 0.0;
                }
                self.current_value = self.envelope.sustain;
            }
            EnvelopeStage::Release => {
                self.current_time += 1.0 / self.sample_rate;
                if self.current_time >= self.envelope.release {
                    self.current_stage = EnvelopeStage::Idle;
                    self.current_value = 0.0;
                } else {
                    let release_progress = self.current_time / self.envelope.release;
                    self.current_value = self.envelope.sustain * (1.0 - release_progress);
                }
            }
            EnvelopeStage::Idle => {
                self.current_value = 0.0;
            }
        }
        
        self.current_value
    }
}

// フィルター
pub struct LowPassFilter {
    cutoff_frequency: f32,
    resonance: f32,
    sample_rate: f32,
    buffer: [f32; 2],
}

impl LowPassFilter {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            cutoff_frequency: 20000.0,
            resonance: 0.0,
            sample_rate,
            buffer: [0.0; 2],
        }
    }
    
    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff_frequency = cutoff.clamp(20.0, self.sample_rate / 2.0);
    }
    
    pub fn set_resonance(&mut self, resonance: f32) {
        self.resonance = resonance.clamp(0.0, 1.0);
    }
    
    pub fn process(&mut self, input: f32) -> f32 {
        let freq = self.cutoff_frequency / self.sample_rate;
        let q = 1.0 + self.resonance * 10.0;
        
        let w0 = 2.0 * std::f32::consts::PI * freq;
        let alpha = w0.sin() / (2.0 * q);
        
        let b0 = (1.0 - alpha.cos()) / 2.0;
        let b1 = 1.0 - alpha.cos();
        let b2 = (1.0 - alpha.cos()) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * alpha.cos();
        let a2 = 1.0 - alpha;
        
        let output = (b0 * input + b1 * self.buffer[0] + b2 * self.buffer[1] 
                     - a1 * self.buffer[0] - a2 * self.buffer[1]) / a0;
        
        self.buffer[1] = self.buffer[0];
        self.buffer[0] = output;
        
        output
    }
}

// 個別の音声（ボイス）
pub struct Voice {
    engine_blender: EngineBlender,
    envelope: EnvelopeGenerator,
    filter: LowPassFilter,
    frequency: f32,
    velocity: f32,
    note: u8,
    is_active: bool,
}

impl Voice {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            engine_blender: EngineBlender::new(sample_rate),
            envelope: EnvelopeGenerator::new(sample_rate),
            filter: LowPassFilter::new(sample_rate),
            frequency: 440.0,
            velocity: 0.5,
            note: 60,
            is_active: false,
        }
    }
    
    pub fn note_on(&mut self, note: u8, velocity: f32) {
        let frequency = 440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0);
        self.frequency = frequency;
        self.note = note;
        self.velocity = velocity.clamp(0.0, 1.0);
        self.engine_blender.set_frequency(frequency);
        self.envelope.note_on();
        self.is_active = true;
    }
    
    pub fn note_off(&mut self) {
        self.envelope.note_off();
        self.is_active = false;
    }
    
    pub fn next_sample(&mut self) -> f32 {
        if !self.is_active {
            return 0.0;
        }
        
        let raw_sample = self.engine_blender.next_sample();
        let envelope_value = self.envelope.next_sample();
        let filtered_sample = self.filter.process(raw_sample * envelope_value);
        
        filtered_sample * self.velocity
    }
    
    pub fn is_active(&self) -> bool {
        self.is_active
    }
    
    pub fn is_released(&self) -> bool {
        !self.is_active && self.envelope.current_stage == EnvelopeStage::Idle
    }
    
    pub fn get_note(&self) -> u8 {
        self.note
    }
    
    // パラメータ設定
    pub fn set_blend(&mut self, blend: f32) {
        self.engine_blender.set_blend_ratio(blend);
    }
    
    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.filter.set_cutoff(cutoff * 20000.0);
    }
    
    pub fn set_resonance(&mut self, resonance: f32) {
        self.filter.set_resonance(resonance);
    }
    
    pub fn set_attack(&mut self, attack: f32) {
        self.envelope.envelope.attack = attack;
    }
    
    pub fn set_decay(&mut self, decay: f32) {
        self.envelope.envelope.decay = decay;
    }
    
    pub fn set_sustain(&mut self, sustain: f32) {
        self.envelope.envelope.sustain = sustain;
    }
    
    pub fn set_release(&mut self, release: f32) {
        self.envelope.envelope.release = release;
    }
    
    // Additive Engine パラメータ
    pub fn set_harmonic_amplitude(&mut self, harmonic_index: usize, amplitude: f32) {
        self.engine_blender.additive_engine().set_harmonic_amplitude(harmonic_index, amplitude);
    }
    
    pub fn toggle_harmonic(&mut self, harmonic_index: usize) {
        self.engine_blender.additive_engine().toggle_harmonic(harmonic_index);
    }
    
    // FM Engine パラメータ
    pub fn set_operator_amplitude(&mut self, operator_index: usize, amplitude: f32) {
        self.engine_blender.fm_engine().set_operator_amplitude(operator_index, amplitude);
    }
    
    pub fn set_operator_frequency_ratio(&mut self, operator_index: usize, ratio: f32) {
        self.engine_blender.fm_engine().set_operator_frequency_ratio(operator_index, ratio);
    }
    
    pub fn set_operator_feedback(&mut self, operator_index: usize, feedback: f32) {
        self.engine_blender.fm_engine().set_operator_feedback(operator_index, feedback);
    }
    
    // Volume control
    pub fn set_volume(&mut self, volume: f32) {
        self.velocity = volume.clamp(0.0, 1.0);
    }
    
    // Envelope control
    pub fn set_envelope(&mut self, envelope: Envelope) {
        self.envelope.set_envelope(envelope);
    }
}

// メインシンセサイザー
pub struct Synthesizer {
    pub voices: HashMap<u8, Voice>,
    sample_rate: f32,
    current_note: Option<u8>,
    current_velocity: Option<f32>,
}

impl Synthesizer {
    pub fn new() -> Self {
        let sample_rate = 44100.0;
        
        Self {
            voices: HashMap::new(),
            sample_rate,
            current_note: None,
            current_velocity: None,
        }
    }
    
    pub fn note_on(&mut self, note: u8, velocity: f32) {
        let voice = self.voices.entry(note).or_insert_with(|| Voice::new(self.sample_rate));
        voice.note_on(note, velocity);
        self.current_note = Some(note);
        self.current_velocity = Some(velocity);
    }
    
    pub fn note_off(&mut self, note: u8) {
        if let Some(voice) = self.voices.get_mut(&note) {
            voice.note_off();
        }
        self.current_note = None;
        self.current_velocity = None;
    }
    
    pub fn next_sample(&mut self) -> f32 {
        let mut sample = 0.0;
        for voice in self.voices.values_mut() {
            sample += voice.next_sample();
        }
        sample / self.voices.len() as f32 // Average voices for polyphony
    }
    
    // パラメータ設定
    pub fn set_blend_ratio(&mut self, ratio: f32) {
        for voice in self.voices.values_mut() {
            voice.set_blend(ratio);
        }
    }
    
    pub fn set_blend(&mut self, blend: f32) {
        for voice in self.voices.values_mut() {
            voice.set_blend(blend);
        }
    }
    
    pub fn set_volume(&mut self, volume: f32) {
        for voice in self.voices.values_mut() {
            voice.set_volume(volume); // Assuming set_volume exists on Voice
        }
    }
    
    pub fn set_filter_cutoff(&mut self, cutoff: f32) {
        for voice in self.voices.values_mut() {
            voice.set_cutoff(cutoff);
        }
    }
    
    pub fn set_cutoff(&mut self, cutoff: f32) {
        for voice in self.voices.values_mut() {
            voice.set_cutoff(cutoff * 20000.0);
        }
    }
    
    pub fn set_filter_resonance(&mut self, resonance: f32) {
        for voice in self.voices.values_mut() {
            voice.set_resonance(resonance);
        }
    }
    
    pub fn set_resonance(&mut self, resonance: f32) {
        for voice in self.voices.values_mut() {
            voice.set_resonance(resonance);
        }
    }
    
    pub fn set_envelope(&mut self, envelope: Envelope) {
        for voice in self.voices.values_mut() {
            voice.set_envelope(envelope);
        }
    }
    
    pub fn set_attack(&mut self, attack: f32) {
        for voice in self.voices.values_mut() {
            voice.set_attack(attack);
        }
    }
    
    pub fn set_decay(&mut self, decay: f32) {
        for voice in self.voices.values_mut() {
            voice.set_decay(decay);
        }
    }
    
    pub fn set_sustain(&mut self, sustain: f32) {
        for voice in self.voices.values_mut() {
            voice.set_sustain(sustain);
        }
    }
    
    pub fn set_release(&mut self, release: f32) {
        for voice in self.voices.values_mut() {
            voice.set_release(release);
        }
    }
    
    // Additive Engine パラメータ
    pub fn set_harmonic_amplitude(&mut self, harmonic_index: usize, amplitude: f32) {
        for voice in self.voices.values_mut() {
            voice.set_harmonic_amplitude(harmonic_index, amplitude);
        }
    }
    
    pub fn toggle_harmonic(&mut self, harmonic_index: usize) {
        for voice in self.voices.values_mut() {
            voice.toggle_harmonic(harmonic_index);
        }
    }
    
    // FM Engine パラメータ
    pub fn set_operator_amplitude(&mut self, operator_index: usize, amplitude: f32) {
        for voice in self.voices.values_mut() {
            voice.set_operator_amplitude(operator_index, amplitude);
        }
    }
    
    pub fn set_operator_frequency_ratio(&mut self, operator_index: usize, ratio: f32) {
        for voice in self.voices.values_mut() {
            voice.set_operator_frequency_ratio(operator_index, ratio);
        }
    }
    
    pub fn set_operator_feedback(&mut self, operator_index: usize, feedback: f32) {
        for voice in self.voices.values_mut() {
            voice.set_operator_feedback(operator_index, feedback);
        }
    }
    
    // ゲッター
    pub fn harmonics(&self) -> &[Harmonic] {
        // This needs to be adapted to return harmonics from all voices
        // For now, it will return the harmonics of the first voice
        if let Some(voice) = self.voices.values().next() {
            &voice.engine_blender.additive_engine.harmonics
        } else {
            &[]
        }
    }
    
    pub fn harmonics_count(&self) -> usize {
        // This needs to be adapted to return the total count of harmonics across all voices
        // For now, it will return the count of harmonics from the first voice
        if let Some(voice) = self.voices.values().next() {
            voice.engine_blender.additive_engine.harmonics.len()
        } else {
            0
        }
    }
    
    pub fn operators(&self) -> &[Operator] {
        // This needs to be adapted to return operators from all voices
        // For now, it will return the operators of the first voice
        if let Some(voice) = self.voices.values().next() {
            &voice.engine_blender.fm_engine.operators
        } else {
            &[]
        }
    }
    
    pub fn operators_count(&self) -> usize {
        // This needs to be adapted to return the total count of operators across all voices
        // For now, it will return the count of operators from the first voice
        if let Some(voice) = self.voices.values().next() {
            voice.engine_blender.fm_engine.operators.len()
        } else {
            0
        }
    }
    
    pub fn is_playing(&self) -> bool {
        // This needs to be adapted to check if any voice is active
        self.voices.values().any(|v| v.is_active())
    }
} 