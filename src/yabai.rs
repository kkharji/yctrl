mod events;
mod models;
mod socket;

use crate::constants::*;
use anyhow::{bail, Error, Result};
pub use events::*;

pub use models::*;
pub use socket::Socket;
use std::fmt;

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

impl TryFrom<Vec<&str>> for Event {
    type Error = Error;
    fn try_from(args: Vec<&str>) -> Result<Self, Error> {
        let val = args.first().unwrap().as_bytes();

        // For some reason match won't work
        let event = if WINDOW_FOCUSED == val {
            Self::Window(WindowEvent::Focused {
                window_id: args.get(1).unwrap().parse::<u32>()?,
            })
        } else if WINDOW_CREATED == val {
            Self::Window(WindowEvent::Created {
                window_id: args.get(1).unwrap().parse::<u32>()?,
            })
        } else if WINDOW_MOVED == val {
            Self::Window(WindowEvent::Moved {
                window_id: args.get(1).unwrap().parse::<u32>()?,
            })
        } else if WINDOW_RESIZED == val {
            Self::Window(WindowEvent::Resized {
                window_id: args.get(1).unwrap().parse::<u32>()?,
            })
        } else if WINDOW_DESTROYED == val {
            Self::Window(WindowEvent::Destroyed {
                window_id: args.get(1).unwrap().parse::<u32>()?,
            })
        } else if WINDOW_MINIMIZED == val {
            Self::Window(WindowEvent::Minimized {
                window_id: args.get(1).unwrap().parse::<u32>()?,
            })
        } else if WINDOW_DEMINIMIZED == val {
            Self::Window(WindowEvent::Deminimized {
                window_id: args.get(1).unwrap().parse::<u32>()?,
            })
        } else if SPACE_CHANGED == val {
            Self::Space(SpaceEvent::Changed {
                space_id: args.get(1).unwrap().parse::<u32>()?,
                recent_space_id: args.get(2).unwrap().parse::<u32>()?,
            })
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
                let event = std::str::from_utf8(&val)?;
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
            match Event::try_from($str) {
                Ok(result) => {
                    if let Event::$type(event) = result {
                        assert!(event.$check_method())
                    }
                }
                Err(e) => panic!("{e}"),
            }
        }};
    }

    should_parse!(vec!["mission_control_exit"], MissionControl, is_exit_event);
    should_parse!(
        vec!["mission_control_enter"],
        MissionControl,
        is_enter_event
    );

    should_parse!(vec!["window_moved", "2"], Window, is_move_event);
    should_parse!(vec!["window_focused", "2"], Window, is_focus_event);
    should_parse!(vec!["window_resized", "2"], Window, is_resize_event);
    should_parse!(vec!["window_created", "3"], Window, is_create_event);
    should_parse!(vec!["window_destroyed", "4"], Window, is_destory_event);
    should_parse!(vec!["window_minimized", "4"], Window, is_minimize_event);
    should_parse!(vec!["window_deminimized", "5"], Window, is_deminimize_event);

    should_parse!(vec!["application_hidden"], Application, is_hidden_event);
    should_parse!(vec!["application_visible"], Application, is_visible_event);

    // should_parse!("window_title_changed", Window, is_title_change_event);
    // should_parse!("application_terminated", Application, is_terminate_event);
    // should_parse!("application_launched", Application, is_launch_event);
    // should_parse!("application_front_switched", Application, is_front_switch_event);
    // should_parse!("application_activated", Application, is_activate_event);
    // should_parse!("application_deactivated", Application, is_deactivate_event);

    should_parse!(vec!["space_changed", "3", "4"], Space, is_change_event);
    should_parse!(vec!["display_changed"], Display, is_change_event);
    should_parse!(vec!["display_added"], Display, is_add_event);
    should_parse!(vec!["display_moved"], Display, is_move_event);
    should_parse!(vec!["display_removed"], Display, is_remove_event);
    should_parse!(vec!["display_resized"], Display, is_resize_event);
}

// TODO: better have this pre event
impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::NotSupported => write!(f, "Not Supported"),
            Event::MissionControl(event) => {
                let phase = if let MissionControlEvent::Enter = event {
                    "Enter"
                } else {
                    "Exit"
                };
                write!(f, "Mission Control {phase}")
            }
            Event::Window(event) => match event {
                WindowEvent::Created { window_id } => write!(f, "Window Created ({window_id})"),
                WindowEvent::Destroyed { window_id } => {
                    write!(f, "Window Destroyed: ({window_id})")
                }
                WindowEvent::Focused { window_id } => write!(f, "Window Focused: ({window_id})"),
                WindowEvent::Moved { window_id } => write!(f, "Window Moved: ({window_id})"),
                WindowEvent::Resized { window_id } => write!(f, "Window Resized: ({window_id})"),
                WindowEvent::Minimized { window_id } => {
                    write!(f, "Window Minimized: ({window_id})")
                }
                WindowEvent::Deminimized { window_id } => {
                    write!(f, "Window Deminimized: ({window_id})")
                }
            },
            Event::Display(event) => match event {
                DisplayEvent::Added => write!(f, "Display Added"),
                DisplayEvent::Removed => write!(f, "Display Removed"),
                DisplayEvent::Moved => write!(f, "Display Moved"),
                DisplayEvent::Resized => write!(f, "Display Resized"),
                DisplayEvent::Changed => write!(f, "Display Changed"),
            },
            Event::Space(event) => match event {
                SpaceEvent::Changed {
                    space_id,
                    recent_space_id,
                } => write!(f, "Space Changed (r:{recent_space_id}, n:{space_id})"),
            },
            Event::Application(event) => match event {
                ApplicationEvent::Deactivated => write!(f, "Application Deactivated"),
                ApplicationEvent::Visible => write!(f, "Application Visible"),
                ApplicationEvent::Hidden => write!(f, "Application Hidden"),
            },
        }
    }
}
