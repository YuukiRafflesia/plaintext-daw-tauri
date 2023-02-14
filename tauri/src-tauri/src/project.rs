use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ProjectPiece {
    #[serde(rename = "bpm")]
    Bpm(f32),
    
    #[serde(rename = "sample_rate")]
    SampleRate(u32),

    #[serde(rename = "clips")]
    Clips(BTreeMap<String, SampleClip>),

    #[serde(rename = "instruments")]
    Instruments(BTreeMap<String, Instrument>),

    Patterns(BTreeMap<String, PatternClip>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SampleClip {
    path: String,
    start: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstrumentClip {
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instrument {
    #[serde(rename = "type")]
    ty: String,
    clips: BTreeMap<String, InstrumentClip>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatternClip {
    instrument: String,
    start: f32,
    repeat: f32,
    notes: Vec<Note>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    value: String,
    start: f32,
    length: f32,
}
