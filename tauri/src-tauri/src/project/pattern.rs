use serde::{Serialize, Deserialize};

use super::note::Note;

#[derive(Debug, Serialize, Deserialize)]
pub struct PatternClip {
    instrument: String,
    start: f32,
    repeat: f32,
    notes: Vec<Note>,
}

impl PatternClip {
    pub fn instrument(&self) -> &str {
        self.instrument.as_ref()
    }

    pub fn start(&self) -> f32 {
        self.start
    }

    pub fn repeat(&self) -> f32 {
        self.repeat
    }

    pub fn notes(&self) -> &[Note] {
        self.notes.as_ref()
    }
}
