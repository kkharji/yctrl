use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Space {
    id: u32,
    uuid: String,
    pub index: u32,
    pub label: String,
    r#type: String,
    display: u32,
    pub windows: Vec<u32>,
    #[serde(rename(deserialize = "first-window"))]
    pub first_window: u32,
    #[serde(rename(deserialize = "last-window"))]
    pub last_window: u32,
    #[serde(rename(deserialize = "has-focus"))]
    pub has_focus: bool,
    #[serde(rename(deserialize = "is-visible"))]
    pub is_visible: bool,
    #[serde(rename(deserialize = "is-native-fullscreen"))]
    is_native_fullscreen: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Display {
    id: u32,
    uuid: String,
    index: u32,
    frame: Frame,
    spaces: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Frame {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Window {
    pub id: u32,
    pub pid: u32,
    pub app: String,
    pub title: String,
    pub frame: Frame,
    pub role: String,
    pub subrole: String,
    pub display: u32,
    pub space: u32,
    pub level: u32,
    pub opacity: f32,
    #[serde(rename(deserialize = "split-type"))]
    pub split_type: String,
    #[serde(rename(deserialize = "stack-index"))]
    pub stack_index: u32,
    #[serde(rename(deserialize = "can-move"))]
    pub is_moveable: bool,
    #[serde(rename(deserialize = "can-resize"))]
    pub is_resizeable: bool,
    #[serde(rename(deserialize = "has-focus"))]
    pub has_focus: bool,
    #[serde(rename(deserialize = "has-shadow"))]
    pub has_shadow: bool,
    #[serde(rename(deserialize = "has-border"))]
    pub has_border: bool,
    #[serde(rename(deserialize = "has-parent-zoom"))]
    pub has_parent_zoom: bool,
    #[serde(rename(deserialize = "has-fullscreen-zoom"))]
    pub has_fullscreen_zoom: bool,
    #[serde(rename(deserialize = "is-native-fullscreen"))]
    pub is_native_fullscreen: bool,
    #[serde(rename(deserialize = "is-visible"))]
    pub is_visible: bool,
    #[serde(rename(deserialize = "is-minimized"))]
    pub is_minimized: bool,
    #[serde(rename(deserialize = "is-hidden"))]
    pub is_hidden: bool,
    #[serde(rename(deserialize = "is-floating"))]
    pub is_floating: bool,
    #[serde(rename(deserialize = "is-sticky"))]
    pub is_sticky: bool,
    #[serde(rename(deserialize = "is-topmost"))]
    pub is_topmost: bool,
    #[serde(rename(deserialize = "is-grabbed"))]
    pub is_grabbed: bool,
}
