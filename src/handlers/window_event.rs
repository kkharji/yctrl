use crate::runtime::EventHandler;
use crate::yabai::{Socket, Window, WindowEvent};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
impl EventHandler for WindowEvent {
    async fn handle(&self) -> Result<()> {
        let yabai = Socket::new()?;
        match self {
            WindowEvent::Created { window_id } => created(&yabai, window_id).await,
            WindowEvent::Destroyed { window_id } => destroyed(&yabai, window_id).await,
            WindowEvent::Focused { window_id } => focused(&yabai, window_id).await,
            WindowEvent::Moved { window_id } => moved(&yabai, window_id).await,
            WindowEvent::Resized { window_id } => resized(&yabai, window_id).await,
            WindowEvent::Minimized { window_id } => minimized(&yabai, window_id).await,
            WindowEvent::Deminimized { window_id } => deminimzed(&yabai, window_id).await,
        }
    }
}

async fn deminimzed(_yabai: &Socket, _window_id: &u32) -> Result<()> {
    Ok(())
}

async fn minimized(_yabai: &Socket, _window_id: &u32) -> Result<()> {
    Ok(())
}

async fn resized(_yabai: &Socket, _window_id: &u32) -> Result<()> {
    Ok(())
}

async fn moved(_yabai: &Socket, _window_id: &u32) -> Result<()> {
    Ok(())
}

async fn focused(_yabai: &Socket, _window_id: &u32) -> Result<()> {
    Ok(())
}

async fn destroyed(yabai: &Socket, _window_id: &u32) -> Result<()> {
    // TODO: Make this configurable
    // NOTE: This maybe better done through trying to query current window '--windows --window'?
    let windows: Vec<Window> = yabai
        .query::<Vec<Window>, _>(&["query", "--windows", "--space"])
        .await?
        .into_iter()
        .filter(|w| w.has_focus)
        .collect();

    // NOTE: this hack doesn't always works. My use case was closing a window in with hammerspoon
    // console, which for some reason is ignored. It does focus on the console, but then switch
    // focus to the app in macos title.
    if windows.is_empty() {
        tracing::info!("Focus isn't in current space, Trying to focus with cursor");
        yabai.execute(&["window", "--focus", "mouse"]).await?
    } else {
        tracing::info!("Focus is still in the same space");
    }

    Ok(())
}

async fn created(_yabai: &Socket, _window_id: &u32) -> Result<()> {
    Ok(())
}
