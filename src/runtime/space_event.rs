use crate::runtime::EventHandler;
use crate::state::SharedState;
use crate::yabai::{self, Socket, Space, SpaceEvent, Window};
use anyhow::{bail, Result};
use async_trait::async_trait;
use std::time::Duration;
use tokio::time::sleep;

#[async_trait]
impl EventHandler for SpaceEvent {
    async fn handle(&self, state: SharedState) -> Result<()> {
        let yabai = yabai::Socket::new()?;
        let config = &state.lock().await.config;

        match self {
            SpaceEvent::Changed {
                space_id,
                recent_space_id,
            } => {
                // TODO: Make it configurable
                auto_focus_window(&yabai, space_id).await?;
                if config.auto_close_empty_spaces {
                    destory_recent_space_when_empty(&yabai, recent_space_id).await?;
                }
                Ok(())
            }
        }
    }
}

/// Switch focus to current space open window if focus is in another space window
async fn auto_focus_window(yabai: &Socket, space_id: &u32) -> Result<()> {
    // TODO: Should only work if there is no focused window in current space.
    let windows = yabai
        .windows("current")
        .await?
        .into_iter()
        .filter(|w| !w.is_minimized && !w.is_hidden)
        .collect::<Vec<Window>>();

    if windows.len() == 0 {
        bail!("No More windows in current space with {space_id}");
    }

    if !windows.iter().any(|w| w.has_focus) {
        tracing::debug!("Focus is stolen in some other space, fixing ...")
    } else {
        return Ok(());
    }

    if yabai
        .execute(&["window", "--focus", "mouse"])
        .await
        .is_err()
    {
        let focus_window = windows.first().unwrap();
        tracing::trace!("Switching focus to {}", focus_window.title);

        let focus_window_id = focus_window.id.to_string();
        let focus_args = &["window", "--focus", &focus_window_id];
        let focus_result = yabai.execute(focus_args).await;

        if let Err(e) = focus_result {
            tracing::error!(
                "Unable to change focus to {}. Cause: {e}",
                focus_window.title
            )
        }
    };
    Ok(())
}

/// When recent space is empty, destory it.
/// Credit: @PickingUpPieces
async fn destory_recent_space_when_empty(yabai: &Socket, recent_space_id: &u32) -> Result<()> {
    // Allow some time for yabai to process
    sleep(Duration::new(0, 6)).await;

    // Get most recent space object.
    let rspace = yabai
        .spaces("current")
        .await?
        .into_iter()
        .filter(|s| &s.id == recent_space_id)
        .collect::<Vec<Space>>()
        .remove(0);

    // Get Most recent space index as a string
    let rspace_idx = rspace.index.to_string();

    // Get recent space valid windows.
    let rspace_windows = yabai.windows(&rspace_idx).await?;

    // Get minimized window count
    let rspace_hidden_windows_count = rspace_windows
        .iter()
        .filter(|w| w.is_minimized || w.is_hidden)
        .count();

    // Check if space should be keept and not destoryed.
    let should_keep_rspace = rspace.is_visible || rspace.has_focus || rspace.is_native_fullscreen;

    if !should_keep_rspace && rspace_windows.len() == rspace_hidden_windows_count {
        tracing::info!("Destorying space at index {} ..", rspace_idx);
        yabai.execute(&["space", &rspace_idx, "--destroy"]).await?;
    }

    Ok(())
}
