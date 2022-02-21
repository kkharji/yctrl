mod events;
mod models;
mod socket;

use crate::constants::*;
use anyhow::{bail, Error, Result};
pub use events::*;

pub use models::*;
pub use socket::Socket;

#[derive(Debug)]
pub enum Event {
    NotSupported,
    /// Mission Control specfic events
    MissionControl(MissionControlEvent),
    /// Window specfic event
    Window(WindowEvent),
    /// Display specfic events
    Display(DisplayEvent),
    /// Space specfic events
    Space(SpaceEvent),
    /// Application specfic events
    Application(ApplicationEvent),
}

impl TryFrom<&mut Vec<u8>> for Event {
    type Error = Error;
    fn try_from(bytes: &mut Vec<u8>) -> Result<Self, Error> {
        let val = &**bytes;
        // For some reason match won't work
        let event = if WINDOW_FOCUSED == val {
            Self::Window(WindowEvent::Focused)
        } else if WINDOW_CREATED == val {
            Self::Window(WindowEvent::Created)
        } else if WINDOW_MOVED == val {
            Self::Window(WindowEvent::Moved)
        } else if WINDOW_RESIZED == val {
            Self::Window(WindowEvent::Resized)
        } else if WINDOW_DESTROYED == val {
            Self::Window(WindowEvent::Destroyed)
        } else if WINDOW_MINIMIZED == val {
            Self::Window(WindowEvent::Minimized)
        } else if WINDOW_DEMINIMIZED == val {
            Self::Window(WindowEvent::Deminimized)
        } else if SPACE_CHANGED == val {
            Self::Space(SpaceEvent::Changed)
        } else if APPLICATION_VISIBLE == val {
            Self::Application(ApplicationEvent::Visible)
        } else if APPLICATION_HIDDEN == val {
            Self::Application(ApplicationEvent::Hidden)
        } else if MISSON_CONTROL_ENTER == val {
            Self::MissionControl(MissionControlEvent::Enter)
        } else if MISSON_CONTROL_EXIT == val {
            Self::MissionControl(MissionControlEvent::Exit)
        } else if DISPLAY_ADDED == val {
            Self::Display(DisplayEvent::Added)
        } else if DISPLAY_REMOVED == val {
            Self::Display(DisplayEvent::Removed)
        } else if DISPLAY_MOVED == val {
            Self::Display(DisplayEvent::Moved)
        } else if DISPLAY_RESIZED == val {
            Self::Display(DisplayEvent::Resized)
        } else if DISPLAY_CHANGED == val {
            Self::Display(DisplayEvent::Changed)
        } else {
            Self::NotSupported
        };

        match event {
            Self::NotSupported => {
                let event = std::str::from_utf8(&bytes)?;
                bail!("Event {event} is not supported.")
            }
            _ => Ok(event),
        }
    }
}

#[test]
fn parse_string_to_event() {
    macro_rules! should_parse {
        ($str: expr, $type: ident, $check_method: ident) => {{
            let mut event_category = $str.as_bytes().to_vec();
            match Event::try_from(&mut event_category) {
                Ok(result) => {
                    if let Event::$type(event) = result {
                        assert!(event.$check_method())
                    } else {
                        panic!("enable to parse {}", $str)
                    }
                }
                Err(e) => panic!("{e}"),
            }
        }};
    }

    should_parse!("mission_control_exit", MissionControl, is_exit_event);
    should_parse!("mission_control_enter", MissionControl, is_enter_event);

    should_parse!("window_moved", Window, is_move_event);
    should_parse!("window_focused", Window, is_focus_event);
    should_parse!("window_resized", Window, is_resize_event);
    should_parse!("window_created", Window, is_create_event);
    should_parse!("window_destroyed", Window, is_destory_event);
    should_parse!("window_minimized", Window, is_minimize_event);
    should_parse!("window_deminimized", Window, is_deminimize_event);

    should_parse!("application_hidden", Application, is_hidden_event);
    should_parse!("application_visible", Application, is_visible_event);

    // should_parse!("window_title_changed", Window, is_title_change_event);
    // should_parse!("application_terminated", Application, is_terminate_event);
    // should_parse!("application_launched", Application, is_launch_event);
    // should_parse!("application_front_switched", Application, is_front_switch_event);
    // should_parse!("application_activated", Application, is_activate_event);
    // should_parse!("application_deactivated", Application, is_deactivate_event);

    should_parse!("space_changed", Space, is_change_event);
    should_parse!("display_changed", Display, is_change_event);
    should_parse!("display_added", Display, is_add_event);
    should_parse!("display_moved", Display, is_move_event);
    should_parse!("display_removed", Display, is_remove_event);
    should_parse!("display_resized", Display, is_resize_event);
}
