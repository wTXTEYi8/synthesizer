// 基本的なオシレーター
pub trait Oscillator {
    fn next_sample(&mut self) -> f32;
    fn set_frequency(&mut self, freq: f32);
    fn set_amplitude(&mut self, amp: f32);
}

pub struct SineOscillator {
    frequency: f32,
    amplitude: f32,
    phase: f32,
    sample_rate: f32,
}

impl SineOscillator {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            frequency: 440.0,
            amplitude: 1.0,
            phase: 0.0,
            sample_rate,
        }
    }
}

impl Oscillator for SineOscillator {
    fn next_sample(&mut self) -> f32 {
        let sample = (self.phase * 2.0 * std::f32::consts::PI).sin() * self.amplitude;
        self.phase += self.frequency / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        sample
    }
    
    fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
    }
    
    fn set_amplitude(&mut self, amp: f32) {
        self.amplitude = amp;
    }
}

// Additive Engine
#[derive(Debug, Clone)]
pub struct Harmonic {
    pub frequency_multiplier: f32,
    pub amplitude: f32,
    pub phase: f32,
    pub enabled: bool,
}

pub struct AdditiveEngine {
    pub harmonics: Vec<Harmonic>,
    base_frequency: f32,
    sample_rate: f32,
    oscillators: Vec<SineOscillator>,
}

impl AdditiveEngine {
    pub fn new(sample_rate: f32) -> Self {
        let mut harmonics = Vec::new();
        let mut oscillators = Vec::new();
        
        // 64個の倍音を初期化
        for i in 1..=64 {
            harmonics.push(Harmonic {
                frequency_multiplier: i as f32,
                amplitude: if i == 1 { 1.0 } else { 0.0 },
                phase: 0.0,
                enabled: i == 1,
            });
            
            oscillators.push(SineOscillator::new(sample_rate));
        }
        
        Self {
            harmonics,
            base_frequency: 440.0,
            sample_rate,
            oscillators,
        }
    }
    
    pub fn set_base_frequency(&mut self, freq: f32) {
        self.base_frequency = freq;
        for (i, osc) in self.oscillators.iter_mut().enumerate() {
            let harmonic = &self.harmonics[i];
            osc.set_frequency(self.base_frequency * harmonic.frequency_multiplier);
            osc.set_amplitude(if harmonic.enabled { harmonic.amplitude } else { 0.0 });
        }
    }
    
    pub fn set_harmonic_amplitude(&mut self, harmonic_index: usize, amplitude: f32) {
        if harmonic_index < self.harmonics.len() {
            self.harmonics[harmonic_index].amplitude = amplitude;
            self.oscillators[harmonic_index].set_amplitude(amplitude);
        }
    }
    
    pub fn toggle_harmonic(&mut self, harmonic_index: usize) {
        if harmonic_index < self.harmonics.len() {
            self.harmonics[harmonic_index].enabled = !self.harmonics[harmonic_index].enabled;
            let amplitude = if self.harmonics[harmonic_index].enabled {
                self.harmonics[harmonic_index].amplitude
            } else {
                0.0
            };
            self.oscillators[harmonic_index].set_amplitude(amplitude);
        }
    }
    
    pub fn next_sample(&mut self) -> f32 {
        let mut sample = 0.0;
        for osc in &mut self.oscillators {
            sample += osc.next_sample();
        }
        sample / 64.0 // 正規化
    }
    
    pub fn harmonics(&self) -> &[Harmonic] {
        &self.harmonics
    }
}

// FM Engine
#[derive(Debug, Clone)]
pub struct Operator {
    pub frequency_ratio: f32,
    pub amplitude: f32,
    pub feedback: f32,
    pub enabled: bool,
}

pub struct FMEngine {
    pub operators: Vec<Operator>,
    base_frequency: f32,
    sample_rate: f32,
    oscillators: Vec<SineOscillator>,
    feedback_buffer: Vec<f32>,
}

impl FMEngine {
    pub fn new(sample_rate: f32) -> Self {
        let mut operators = Vec::new();
        let mut oscillators = Vec::new();
        let mut feedback_buffer = Vec::new();
        
        // 6個のオペレーターを初期化
        for i in 0..6 {
            operators.push(Operator {
                frequency_ratio: if i == 0 { 1.0 } else { 0.0 },
                amplitude: if i == 0 { 1.0 } else { 0.0 },
                feedback: 0.0,
                enabled: i == 0,
            });
            
            oscillators.push(SineOscillator::new(sample_rate));
            feedback_buffer.push(0.0);
        }
        
        Self {
            operators,
            base_frequency: 440.0,
            sample_rate,
            oscillators,
            feedback_buffer,
        }
    }
    
    pub fn set_base_frequency(&mut self, freq: f32) {
        self.base_frequency = freq;
        for (i, osc) in self.oscillators.iter_mut().enumerate() {
            let op = &self.operators[i];
            osc.set_frequency(self.base_frequency * op.frequency_ratio);
        }
    }
    
    pub fn set_operator_amplitude(&mut self, operator_index: usize, amplitude: f32) {
        if operator_index < self.operators.len() {
            self.operators[operator_index].amplitude = amplitude;
        }
    }
    
    pub fn set_operator_frequency_ratio(&mut self, operator_index: usize, ratio: f32) {
        if operator_index < self.operators.len() {
            self.operators[operator_index].frequency_ratio = ratio;
            self.oscillators[operator_index].set_frequency(self.base_frequency * ratio);
        }
    }
    
    pub fn set_operator_feedback(&mut self, operator_index: usize, feedback: f32) {
        if operator_index < self.operators.len() {
            self.operators[operator_index].feedback = feedback;
        }
    }
    
    pub fn next_sample(&mut self) -> f32 {
        let mut output = 0.0;
        
        // 各オペレーターの処理
        for i in 0..self.operators.len() {
            if !self.operators[i].enabled {
                continue;
            }
            
            let mut phase_modulation = 0.0;
            
            // フィードバック
            if self.operators[i].feedback > 0.0 {
                phase_modulation += self.feedback_buffer[i] * self.operators[i].feedback;
            }
            
            // 他のオペレーターからの変調（簡易版）
            for j in 0..self.operators.len() {
                if i != j && self.operators[j].enabled {
                    phase_modulation += self.feedback_buffer[j] * 0.1; // 簡易変調
                }
            }
            
            // オシレーターの位相を変調
            let sample = (self.oscillators[i].next_sample() + phase_modulation).sin() 
                * self.operators[i].amplitude;
            
            self.feedback_buffer[i] = sample;
            output += sample;
        }
        
        output / 6.0 // 正規化
    }
    
    pub fn operators(&self) -> &[Operator] {
        &self.operators
    }
}

// エンジンブレンダー
pub struct EngineBlender {
    pub additive_engine: AdditiveEngine,
    pub fm_engine: FMEngine,
    blend_ratio: f32, // 0.0 = Additive only, 1.0 = FM only
}

impl EngineBlender {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            additive_engine: AdditiveEngine::new(sample_rate),
            fm_engine: FMEngine::new(sample_rate),
            blend_ratio: 0.5,
        }
    }
    
    pub fn set_blend_ratio(&mut self, ratio: f32) {
        self.blend_ratio = ratio.clamp(0.0, 1.0);
    }
    
    pub fn set_frequency(&mut self, freq: f32) {
        self.additive_engine.set_base_frequency(freq);
        self.fm_engine.set_base_frequency(freq);
    }
    
    pub fn next_sample(&mut self) -> f32 {
        let additive_sample = self.additive_engine.next_sample();
        let fm_sample = self.fm_engine.next_sample();
        
        // クロスフェード
        additive_sample * (1.0 - self.blend_ratio) + fm_sample * self.blend_ratio
    }
    
    pub fn additive_engine(&mut self) -> &mut AdditiveEngine {
        &mut self.additive_engine
    }
    
    pub fn fm_engine(&mut self) -> &mut FMEngine {
        &mut self.fm_engine
    }
} 