mod clip;
mod instrument;
mod pattern;
mod note;

use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};

use self::{clip::SampleClip, instrument::Instrument, pattern::PatternClip};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    bpm: f32,
    sample_rate: usize,
    clips: BTreeMap<String, SampleClip>,
    instruments: BTreeMap<String, Instrument>,
    patterns: BTreeMap<String, PatternClip>,
}

impl Project {
    pub fn bpm(&self) -> f32 {
        self.bpm
    }

    pub fn sample_rate(&self) -> usize {
        self.sample_rate
    }

    pub fn clips(&self) -> &BTreeMap<String, SampleClip> {
        &self.clips
    }

    pub fn instruments(&self) -> &BTreeMap<String, Instrument> {
        &self.instruments
    }

    pub fn patterns(&self) -> &BTreeMap<String, PatternClip> {
        &self.patterns
    }
}
