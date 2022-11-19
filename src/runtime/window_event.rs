use crate::config::Config;
use crate::runtime::EventHandler;
use crate::state::SharedState;
use crate::util::window_hide_current;
use crate::yabai::{Socket, WindowEvent};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
impl EventHandler for WindowEvent {
    async fn handle(&self, state: SharedState) -> Result<()> {
        let yabai = Socket::new()?;
        match self {
            WindowEvent::Created { window_id } => created(&yabai, window_id).await,
            WindowEvent::Destroyed { window_id } | WindowEvent::Minimized { window_id } => {
                focus_last(&yabai, window_id).await
            }
            WindowEvent::Focused { window_id } => {
                focused(&yabai, window_id, &state.lock().await.config).await
            }
            WindowEvent::Moved { window_id } => moved(&yabai, window_id).await,
            WindowEvent::Resized { window_id } => resized(&yabai, window_id).await,
            WindowEvent::Deminimized { window_id } => deminimzed(&yabai, window_id).await,
        }
    }
}

async fn deminimzed(_yabai: &Socket, _window_id: &u32) -> Result<()> {
    Ok(())
}

async fn focus_last(yabai: &Socket, _window_id: &u32) -> Result<()> {
    let last_window_id = yabai.last_window().await?.id;
    yabai
        .execute(&[
            "window".to_string(),
            "--focus".into(),
            format!("{last_window_id}"),
        ])
        .await
}

async fn resized(_yabai: &Socket, _window_id: &u32) -> Result<()> {
    Ok(())
}

async fn moved(_yabai: &Socket, _window_id: &u32) -> Result<()> {
    Ok(())
}

async fn focused(yabai: &Socket, _window_id: &u32, config: &Config) -> Result<()> {
    let last_window = yabai.last_window().await?;
    let is_scratchpad = config
        .scratchpads()
        .iter()
        .find(|sp| {
            if sp.kind.is_app() {
                last_window.app == sp.target
            } else {
                last_window.title == sp.target
            }
        })
        .is_some();

    if is_scratchpad {
        window_hide_current().await?;
    }

    Ok(())
}

async fn created(_yabai: &Socket, _window_id: &u32) -> Result<()> {
    Ok(())
}
