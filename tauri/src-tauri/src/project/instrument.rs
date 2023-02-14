use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InstrumentClip {
    path: String,
}

impl InstrumentClip {
    pub fn path(&self) -> &str {
        self.path.as_ref()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instrument {
    #[serde(rename = "type")]
    ty: String,
    clips: BTreeMap<String, InstrumentClip>,
}

impl Instrument {
    pub fn ty(&self) -> &str {
        self.ty.as_ref()
    }

    pub fn clips(&self) -> &BTreeMap<String, InstrumentClip> {
        &self.clips
    }
}
