use crate::{config::Config, yabai};
use anyhow::{bail, Result};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct State {
    pub config: Config,
    pub scratchpad_open: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            config: Config::default(),
            scratchpad_open: false,
        }
    }
}

pub type SharedState = Arc<Mutex<State>>;

impl State {
    pub async fn handle(&mut self, mut args: Vec<&str>) -> Result<()> {
        let key = args.remove(0);
        let value = args.get(1).unwrap();
        let yabai = yabai::Socket::new()?;

        match key {
            "yctrl_auto_close_empty_spaces" => {
                self.config.set_auto_close_empty_spaces_with_str(value)?;
            }
            "yctrl_scratchpad_launch_timeout" => {
                self.config.set_scratchpad_launch_timeout_with_str(value)?;
            }
            "yctrl_scratchpad_space" => {
                self.config.set_scratchpad_space_with_str(value)?;
            }
            "yctrl_scratchpad_grid" => {
                self.config.set_scratchpad_grid_with_str(value)?;
            }
            "yctrl_scratchpads" => {
                self.config.set_scratchpads_with_str(&args.join(" "))?;
                for configure_args in self
                    .config
                    .scratchpads()
                    .iter()
                    .map(|sp| sp.configure_args(&self.config))
                {
                    yabai.request(&configure_args).await?;
                }
            }

            _ => bail!("Unknown config key {key}"),
        }

        Ok(())
    }
}
