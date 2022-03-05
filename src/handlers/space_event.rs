use crate::runtime::EventHandler;
use crate::yabai::{self, Socket, Space, SpaceEvent, Window};
use anyhow::Result;
use async_trait::async_trait;
use std::time::Duration;
use tokio::time::sleep;

#[async_trait]
impl EventHandler for SpaceEvent {
    async fn handle(&self) -> Result<()> {
        let yabai = yabai::Socket::new()?;

        tracing::info!("Handling {:?} Event", self);

        match self {
            SpaceEvent::Changed {
                space_id: _,
                recent_space_id,
            } => {
                // TODO: Make it configurable
                destory_recent_space_when_empty(&yabai, recent_space_id).await
            }
        }
    }
}

/// When recent space is empty, destory it.
/// Credit: @PickingUpPieces
async fn destory_recent_space_when_empty(yabai: &Socket, recent_space_id: &u32) -> Result<()> {
    // Allow some time for yabai to process
    sleep(Duration::new(0, 6)).await;

    // Get most recent space object.
    let rspace = yabai
        .query::<Vec<Space>, _>(&["query", "--spaces"])
        .await?
        .into_iter()
        .filter(|s| &s.id == recent_space_id)
        .collect::<Vec<Space>>()
        .remove(0);

    // Get Most recent space index as a string
    let rspace_idx = rspace.index.to_string();

    // Get recent space valid windows.
    let rspace_windows = yabai
        .query::<Vec<Window>, _>(&["query", "--windows", "--space", &rspace_idx])
        .await?
        .into_iter()
        .filter(|w| w.subrole != "AXUnknown.Hammerspoon")
        .collect::<Vec<Window>>();

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
