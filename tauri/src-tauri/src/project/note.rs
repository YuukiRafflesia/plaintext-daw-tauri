use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    value: String,
    start: f32,
    length: f32,
}

impl Note {
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
