use crate::yabai::Socket;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TargetKind {
    Title,
    App,
}

impl TargetKind {
    pub fn is_title(&self) -> bool {
        matches!(self, Self::Title)
    }
    pub fn is_app(&self) -> bool {
        matches!(self, Self::App)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scratchpad {
    pub tag: String,
    pub kind: TargetKind,
    pub target: String,
    pub command: String,
    pub timeout: Option<u8>,
    pub space: Option<u8>,
}

pub struct ScratchpadService;

impl ScratchpadService {
    pub async fn handle(yabai: &Socket, args: Vec<String>) -> Result<()> {
        todo!()
    }
}
