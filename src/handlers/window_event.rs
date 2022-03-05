use crate::runtime::EventHandler;
use crate::yabai::WindowEvent;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
impl EventHandler for WindowEvent {
    async fn handle(&self) -> Result<()> {
        tracing::info!("Handling {:?} Event", self);
        match self {
            WindowEvent::Created { window_id } => created(window_id).await,
            WindowEvent::Destroyed { window_id } => destroyed(window_id).await,
            WindowEvent::Focused { window_id } => focused(window_id).await,
            WindowEvent::Moved { window_id } => moved(window_id).await,
            WindowEvent::Resized { window_id } => resized(window_id).await,
            WindowEvent::Minimized { window_id } => minimized(window_id).await,
            WindowEvent::Deminimized { window_id } => deminimzed(window_id).await,
        }
    }
}

async fn deminimzed(_window_id: &u32) -> Result<()> {
    Ok(())
}

async fn minimized(_window_id: &u32) -> Result<()> {
    Ok(())
}

async fn resized(_window_id: &u32) -> Result<()> {
    Ok(())
}

async fn moved(_window_id: &u32) -> Result<()> {
    Ok(())
}

async fn focused(_window_id: &u32) -> Result<()> {
    Ok(())
}

async fn destroyed(_window_id: &u32) -> Result<()> {
    Ok(())
}

async fn created(_window_id: &u32) -> Result<()> {
    Ok(())
}
