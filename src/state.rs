use anyhow::{bail, Result};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Config {
    /// Whether to auto close empty spaces.
    /// default: true
    pub auto_close_empty_spaces: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            auto_close_empty_spaces: true,
        }
    }
}

pub struct State {
    pub config: Config,
}

impl Default for State {
    fn default() -> Self {
        Self {
            config: Config::default(),
        }
    }
}

pub type SharedState = Arc<Mutex<State>>;

impl State {
    pub fn handle(&mut self, arguemnts: Vec<&str>) -> Result<()> {
        let key = arguemnts.get(0).unwrap();
        let value = arguemnts.get(1).unwrap();

        if key == &"yctrl_auto_close_empty_spaces" {
            self.config.auto_close_empty_spaces = value.parse::<bool>()?;
        } else {
            bail!("Unknown config key {key}")
        }

        Ok(())
    }
}
