use crate::scratchpad::Scratchpad;
use anyhow::Result;

#[derive(Debug)]
pub struct Config {
    auto_close_empty_spaces: bool,
    scratchpad_grid: String,
    scratchpads: Vec<Scratchpad>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            auto_close_empty_spaces: true,
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
