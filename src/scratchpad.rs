use crate::config::Config;
use crate::yabai::Socket;
use crate::{state::SharedState, yabai};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TargetKind {
    Title,
    App,
}

impl TargetKind {
    pub fn is_app(&self) -> bool {
        matches!(self, Self::App)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scratchpad {
    pub tag: String,
    pub kind: TargetKind,
    pub target: String,
    pub command: Vec<String>,
    pub timeout: Option<u8>,
    pub space: Option<u8>,
}

impl Scratchpad {
    pub fn configure_args(&self, config: &Config) -> Vec<String> {
        let target = if self.kind.is_app() {
            format!("app=^{}$", self.target)
        } else {
            format!("title=^{}$", self.target)
        };
        vec![
            "rule".into(),
            "--add".into(),
            target,
            format!("grid={}", config.scratchpad_grid()),
            "manage=off".into(),
        ]
    }
}

pub struct ScratchpadEvent;

impl ScratchpadEvent {
    pub async fn toggle(state: SharedState, tag: &str) -> Result<()> {
        let yabai = Socket::new()?;
        let config = &state.lock().await.config;
        let sp = if let Some(sp) = config.scratchpad_by_tag(tag) {
            sp
        } else {
            bail!("No Scratchpad with given tag: {tag}");
        };
        tracing::debug!("{sp:#?}");

        let window = yabai.focused_window().await?;
        tracing::debug!("{window:#?}");

        let target = if sp.kind.is_app() {
            window.app
        } else {
            window.title
        };

        if target == sp.target {
            // minimize it
            yabai
                .execute(&[
                    "window".to_string(),
                    format!("{}", window.id),
                    "--minimize".into(),
                ])
                .await?;
        } else {
            let mut args = sp.command.clone();
            let cmd = args.remove(0);
            tracing::info!("running: {:?} with {:?}", cmd, args);
            Command::new(cmd).args(args).spawn()?;
        };

        Ok(())
    }
}
