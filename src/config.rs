use crate::scratchpad::Scratchpad;
use anyhow::Result;

#[derive(Debug)]
pub struct Config {
    /// Whether to auto close empty spaces.
    /// default: true
    auto_close_empty_spaces: bool,
    scratchpad_launch_timeout: u8,
    scratchpad_space: u8,
    scratchpad_grid: String,
    scratchpads: Vec<Scratchpad>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            auto_close_empty_spaces: true,
            scratchpad_launch_timeout: 10,
            scratchpad_space: 8,
            scratchpad_grid: "6:4:1:1:2:4".into(),
            scratchpads: vec![],
        }
    }
}

impl Config {
    pub fn set_auto_close_empty_spaces_with_str(&mut self, value: &str) -> Result<()> {
        self.auto_close_empty_spaces = value.parse()?;
        Ok(())
    }

    pub fn set_scratchpad_launch_timeout_with_str(&mut self, value: &str) -> Result<()> {
        self.scratchpad_launch_timeout = value.parse()?;
        Ok(())
    }

    pub fn set_scratchpad_space_with_str(&mut self, value: &str) -> Result<()> {
        self.scratchpad_space = value.parse()?;
        Ok(())
    }

    pub fn set_scratchpad_grid_with_str(&mut self, value: &str) -> Result<()> {
        self.scratchpad_grid = value.parse()?;
        Ok(())
    }

    pub fn set_scratchpads_with_str(&mut self, scratchpads: &str) -> Result<()> {
        self.scratchpads = json5::from_str(scratchpads)?;
        Ok(())
    }

    pub fn auto_close_empty_spaces(&self) -> &bool {
        &self.auto_close_empty_spaces
    }

    pub fn scratchpad_launch_timeout(&self) -> &u8 {
        &self.scratchpad_launch_timeout
    }

    pub fn scratchpad_space(&self) -> &u8 {
        &self.scratchpad_space
    }

    pub fn scratchpad_grid(&self) -> &String {
        &self.scratchpad_grid
    }

    pub fn scratchpads(&self) -> &Vec<Scratchpad> {
        &self.scratchpads
    }

    pub fn scratchpad_by_tag(&self, tag: &str) -> Option<&Scratchpad> {
        self.scratchpads.iter().find(|sp| sp.tag == tag)
    }
}
