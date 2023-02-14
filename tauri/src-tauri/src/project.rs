use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub bpm: f32,
    pub sample_rate: u32,
    pub clips: BTreeMap<String, SampleClip>,
    pub instruments: BTreeMap<String, Instrument>,
    pub patterns: BTreeMap<String, PatternClip>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SampleClip {
    pub path: String,
    pub start: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstrumentClip {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instrument {
    #[serde(rename = "type")]
    pub ty: String,
    pub clips: BTreeMap<String, InstrumentClip>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatternClip {
    pub instrument: String,
    pub start: f32,
    pub repeat: f32,
    pub notes: Vec<Note>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub value: String,
    pub start: f32,
    pub length: f32,
}
