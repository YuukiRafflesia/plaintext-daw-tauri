use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    value: String,
    start: f32,
    length: f32,
}

impl Note {
    pub fn beats_to_samples(beats: f32, bpm: f32, sample_rate: u32) -> u32 {
        (beats * (60.0 / bpm)) as u32 * sample_rate
    }

    pub fn start_sample(&self, bpm: f32, sample_rate: u32) -> u32 {
        Self::beats_to_samples(self.start(), bpm, sample_rate)
    }
    
    pub fn end_sample(&self, bpm: f32, sample_rate: u32) -> u32 {
        let beat = self.start() + self.length();
        Self::beats_to_samples(beat, bpm, sample_rate)
    }

    pub fn value(&self) -> &str {
        self.value.as_ref()
    }

    pub fn start(&self) -> f32 {
        self.start
    }

    pub fn length(&self) -> f32 {
        self.length
    }
}
