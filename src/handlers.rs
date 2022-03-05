mod space_event;
mod window_event;
use crate::runtime::EventHandler;
use crate::yabai::Event;
use anyhow::{bail, Result};

use async_trait::async_trait;

#[async_trait]
impl EventHandler for Event {
    async fn handle(&self) -> Result<()> {
        match self {
            Event::Window(e) => e.handle().await,
            Event::Space(s) => s.handle().await,
            _ => {
                bail!("{:?} is not supported.", self)
            }
        }
    }
}
