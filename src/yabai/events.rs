use crate::constants::*;
use anyhow::{bail, Error, Result};
use std::fmt;
use std::fmt::Debug;

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

#[derive(Debug, PartialEq)]
pub enum MissionControlEvent {
    /// Triggered when mission-control activates.
    Enter,
    /// Triggered when mission-control deactivates.
    Exit,
}

#[derive(Debug)]
pub enum SpaceEvent {
    /// Triggered when the active space has changed.
    ///
    /// Passes two arguments: $YABAI_SPACE_ID, $YABAI_RECENT_SPACE_ID
    Changed { space_id: u32, recent_space_id: u32 },
}

#[derive(Debug)]
pub enum DisplayEvent {
    /// Triggered when a new display has been added.
    ///
    /// Passes one argument: $YABAI_DISPLAY_ID
    Added,
    /// Triggered when a display has been removed.
    ///
    /// Passes one argument: $YABAI_DISPLAY_ID
    Removed,
    /// Triggered when a change has been made to display arrangement.
    ///
    /// Passes one argument: $YABAI_DISPLAY_ID
    Moved,
    /// Triggered when a display has changed resolution.
    ///
    /// Passes one argument: $YABAI_DISPLAY_ID
    Resized,
    /// Triggered when the active display has changed.
    ///
    /// Passes two arguments: $YABAI_DISPLAY_ID, $YABAI_RECENT_DISPLAY_ID
    Changed,
}

#[derive(Debug)]
pub enum WindowEvent {
    /// Triggered when a window is created + implicitly created at application launch.
    ///
    /// Passes one argument: $YABAI_WINDOW_ID
    Created { window_id: u32 },
    /// Triggered when a window is destroyed + implicitly destroyed at application exit.
    ///
    /// Passes one argument: $YABAI_WINDOW_ID
    Destroyed { window_id: u32 },
    /// Triggered when a window becomes the key-window. +
    ///
    /// Passes one argument: $YABAI_WINDOW_ID
    Focused { window_id: u32 },
    /// Triggered when a window changes position. +
    ///
    /// Passes one argument: $YABAI_WINDOW_ID
    Moved { window_id: u32 },
    /// Triggered when a window changes dimensions.
    ///
    /// Passes one argument: $YABAI_WINDOW_ID
    Resized { window_id: u32 },
    /// Triggered when a window has been minimized. +
    ///
    /// Passes one argument: $YABAI_WINDOW_ID
    Minimized { window_id: u32 },
    /// Triggered when a window has been deminimized. +
    ///
    /// Passes one argument: $YABAI_WINDOW_ID
    Deminimized { window_id: u32 },
    // Triggered when a window changes its title. +
    //
    // Passes one argument: $YABAI_WINDOW_ID
    // TitleChanged,
}

#[derive(Debug)]
pub enum ApplicationEvent {
    /// Triggered when a new application is launched.
    ///
    /// Passes one argument: $YABAI_PROCESS_ID
    // Launched,
    /// Triggered when an application is terminated. +
    ///
    /// Passes one argument: $YABAI_PROCESS_ID
    // Terminated,
    /// Triggered when the front-most application changes
    ///
    /// Passes two arguments: $YABAI_PROCESS_ID, $YABAI_RECENT_PROCESS_ID
    // FrontSwitched,
    /// Triggered when an application is activated.
    ///
    /// Passes one argument: $YABAI_PROCESS_ID
    // Activated,
    /// Triggered when an application is deactivated.
    ///
    /// Passes one argument: $YABAI_PROCESS_ID
    // Deactivated,
    /// Triggered when an application is unhidden.
    ///
    /// Passes one argument: $YABAI_PROCESS_ID
    Visible,
    /// Triggered when an application is hidden.
    ///
    /// Passes one argument: $YABAI_PROCESS_ID
    Hidden,
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
                // ApplicationEvent::Deactivated => write!(f, "Application Deactivated"),
                ApplicationEvent::Visible => write!(f, "Application Visible"),
                ApplicationEvent::Hidden => write!(f, "Application Hidden"),
            },
        }
    }
}
