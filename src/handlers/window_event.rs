use crate::runtime::EventHandler;
use crate::yabai::WindowEvent;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
impl EventHandler for WindowEvent {
    async fn handle(&self) -> Result<()> {
        match self {
            WindowEvent::Created { window_id } => todo!(),
            WindowEvent::Destroyed { window_id } => todo!(),
            WindowEvent::Focused { window_id } => todo!(),
            WindowEvent::Moved { window_id } => todo!(),
            WindowEvent::Resized { window_id } => todo!(),
            WindowEvent::Minimized { window_id } => todo!(),
            WindowEvent::Deminimized { window_id } => todo!(),
        }
    }
}
