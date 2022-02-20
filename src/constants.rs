pub const QUERY_SPACE_WINDOWS: &[&str; 3] = &["query", "--windows", "--space"];
pub const QUERY_CURRENT_SPACE: &[&str; 3] = &["query", "--spaces", "--space"];
pub const WINDOW_CREATED: &[u8; 14] = b"window_created";
pub const WINDOW_DESTROYED: &[u8; 16] = b"window_destroyed";
pub const WINDOW_FOCUSED: &[u8; 14] = b"window_focused";
pub const WINDOW_MOVED: &[u8; 12] = b"window_moved";
pub const WINDOW_RESIZED: &[u8; 14] = b"window_resized";
pub const WINDOW_MINIMIZED: &[u8; 16] = b"window_minimized";
pub const WINDOW_DEMINIMIZED: &[u8; 18] = b"window_deminimized";
pub const WINDOW_TITLE_CHANGED: &[u8; 20] = b"window_title_changed";

pub const SPACE_CHANGED: &[u8; 13] = b"space_changed";

pub const DISPLAY_ADDED: &[u8; 13] = b"display_added";
pub const DISPLAY_REMOVED: &[u8; 15] = b"display_removed";
pub const DISPLAY_MOVED: &[u8; 13] = b"display_moved";
pub const DISPLAY_RESIZED: &[u8; 15] = b"display_resized";
pub const DISPLAY_CHANGED: &[u8; 15] = b"display_changed";

pub const MISSON_CONTROL_ENTER: &[u8; 21] = b"mission_control_enter";
pub const MISSON_CONTROL_EXIT: &[u8; 20] = b"mission_control_exit";

pub const APPLICATION_LAUNCHED: &[u8; 20] = b"application_launched";
pub const APPLICATION_TERMINATED: &[u8; 22] = b"application_terminated";
pub const APPLICATION_FRONT_SWITCHED: &[u8; 26] = b"application_front_switched";
pub const APPLICATION_ACTIVATED: &[u8; 21] = b"application_activated";
pub const APPLICATION_DEACTIVATED: &[u8; 23] = b"application_deactivated";
pub const APPLICATION_VISIBLE: &[u8; 19] = b"application_visible";
pub const APPLICATION_HIDDEN: &[u8; 18] = b"application_hidden";
