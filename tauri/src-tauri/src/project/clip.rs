use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SampleClip {
    path: String,
    start: f32,
}

impl SampleClip {
    pub fn path(&self) -> &str {
        self.path.as_ref()
    }

    pub fn start(&self) -> f32 {
        self.start
    }
}
