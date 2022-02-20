#![allow(dead_code)]
use bytes::Bytes;

use anyhow::{bail, Error, Result};
use std::fmt::Debug;

#[derive(Debug)]
pub enum Event {
    UnSupported,
    MissionControl(MissionControlEvent),
    Window(WindowEvent),
    Display(DisplayEvent),
    Space(SpaceEvent),
    Application(ApplicationEvent),
}

#[derive(Debug, PartialEq)]
pub enum MissionControlEvent {
    Enter,
    Exit,
}

impl MissionControlEvent {
    /// Returns `true` if the mission control event is [`Enter`].
    ///
    /// [`Enter`]: MissionControlEvent::Enter
    pub fn is_enter(&self) -> bool {
        matches!(self, Self::Enter)
    }

    /// Returns `true` if the mission control event is [`Exit`].
    ///
    /// [`Exit`]: MissionControlEvent::Exit
    pub fn is_exit(&self) -> bool {
        matches!(self, Self::Exit)
    }
}

#[derive(Debug)]
pub enum SpaceEvent {
    Changed,
}

impl SpaceEvent {
    /// Returns `true` if the space event is [`Changed`].
    ///
    /// [`Changed`]: SpaceEvent::Changed
    pub fn is_changed(&self) -> bool {
        matches!(self, Self::Changed)
    }
}

#[derive(Debug)]
pub enum DisplayEvent {
    Added,
    Removed,
    Moved,
    Resized,
    Changed,
}

impl DisplayEvent {
    /// Returns `true` if the display event is [`Added`].
    ///
    /// [`Added`]: DisplayEvent::Added
    pub fn is_added(&self) -> bool {
        matches!(self, Self::Added)
    }

    /// Returns `true` if the display event is [`Removed`].
    ///
    /// [`Removed`]: DisplayEvent::Removed
    pub fn is_removed(&self) -> bool {
        matches!(self, Self::Removed)
    }

    /// Returns `true` if the display event is [`Moved`].
    ///
    /// [`Moved`]: DisplayEvent::Moved
    pub fn is_moved(&self) -> bool {
        matches!(self, Self::Moved)
    }

    /// Returns `true` if the display event is [`Resized`].
    ///
    /// [`Resized`]: DisplayEvent::Resized
    pub fn is_resized(&self) -> bool {
        matches!(self, Self::Resized)
    }

    /// Returns `true` if the display event is [`Changed`].
    ///
    /// [`Changed`]: DisplayEvent::Changed
    pub fn is_changed(&self) -> bool {
        matches!(self, Self::Changed)
    }
}

#[derive(Debug)]
pub enum WindowEvent {
    Created,
    Destroyed,
    Focused,
    Moved,
    Resized,
    Minimized,
    Deminimized,
    TitleChanged,
}

impl WindowEvent {
    /// Returns `true` if the window event is [`Created`].
    ///
    /// [`Created`]: WindowEvent::Created
    pub fn is_created(&self) -> bool {
        matches!(self, Self::Created)
    }

    /// Returns `true` if the window event is [`Destroyed`].
    ///
    /// [`Destroyed`]: WindowEvent::Destroyed
    pub fn is_destroyed(&self) -> bool {
        matches!(self, Self::Destroyed)
    }

    /// Returns `true` if the window event is [`Focused`].
    ///
    /// [`Focused`]: WindowEvent::Focused
    pub fn is_focused(&self) -> bool {
        matches!(self, Self::Focused)
    }

    /// Returns `true` if the window event is [`Moved`].
    ///
    /// [`Moved`]: WindowEvent::Moved
    pub fn is_moved(&self) -> bool {
        matches!(self, Self::Moved)
    }

    /// Returns `true` if the window event is [`Resized`].
    ///
    /// [`Resized`]: WindowEvent::Resized
    pub fn is_resized(&self) -> bool {
        matches!(self, Self::Resized)
    }

    /// Returns `true` if the window event is [`Minimized`].
    ///
    /// [`Minimized`]: WindowEvent::Minimized
    pub fn is_minimized(&self) -> bool {
        matches!(self, Self::Minimized)
    }

    /// Returns `true` if the window event is [`Deminimized`].
    ///
    /// [`Deminimized`]: WindowEvent::Deminimized
    pub fn is_deminimized(&self) -> bool {
        matches!(self, Self::Deminimized)
    }

    /// Returns `true` if the window event is [`TitleChanged`].
    ///
    /// [`TitleChanged`]: WindowEvent::TitleChanged
    pub fn is_title_changed(&self) -> bool {
        matches!(self, Self::TitleChanged)
    }
}

#[derive(Debug)]
pub enum ApplicationEvent {
    Launched,
    Terminated,
    FrontSwitched,
    Activated,
    Deactivated,
    Visible,
    Hidden,
}

impl ApplicationEvent {
    /// Returns `true` if the application event is [`Launched`].
    ///
    /// [`Launched`]: ApplicationEvent::Launched
    pub fn is_launched(&self) -> bool {
        matches!(self, Self::Launched)
    }

    /// Returns `true` if the application event is [`Terminated`].
    ///
    /// [`Terminated`]: ApplicationEvent::Terminated
    pub fn is_terminated(&self) -> bool {
        matches!(self, Self::Terminated)
    }

    /// Returns `true` if the application event is [`FrontSwitched`].
    ///
    /// [`FrontSwitched`]: ApplicationEvent::FrontSwitched
    pub fn is_front_switched(&self) -> bool {
        matches!(self, Self::FrontSwitched)
    }

    /// Returns `true` if the application event is [`Activated`].
    ///
    /// [`Activated`]: ApplicationEvent::Activated
    pub fn is_activated(&self) -> bool {
        matches!(self, Self::Activated)
    }

    /// Returns `true` if the application event is [`Deactivated`].
    ///
    /// [`Deactivated`]: ApplicationEvent::Deactivated
    pub fn is_deactivated(&self) -> bool {
        matches!(self, Self::Deactivated)
    }

    /// Returns `true` if the application event is [`Visible`].
    ///
    /// [`Visible`]: ApplicationEvent::Visible
    pub fn is_visible(&self) -> bool {
        matches!(self, Self::Visible)
    }

    /// Returns `true` if the application event is [`Hidden`].
    ///
    /// [`Hidden`]: ApplicationEvent::Hidden
    pub fn is_hidden(&self) -> bool {
        matches!(self, Self::Hidden)
    }
}

/// Events
const WINDOW_CREATED: &[u8; 14] = b"window_created";
const WINDOW_DESTROYED: &[u8; 16] = b"window_destroyed";
const WINDOW_FOCUSED: &[u8; 14] = b"window_focused";
const WINDOW_MOVED: &[u8; 12] = b"window_moved";
const WINDOW_RESIZED: &[u8; 14] = b"window_resized";
const WINDOW_MINIMIZED: &[u8; 16] = b"window_minimized";
const WINDOW_DEMINIMIZED: &[u8; 18] = b"window_deminimized";
const WINDOW_TITLE_CHANGED: &[u8; 20] = b"window_title_changed";

const SPACE_CHANGED: &[u8; 13] = b"space_changed";

const DISPLAY_ADDED: &[u8; 13] = b"display_added";
const DISPLAY_REMOVED: &[u8; 15] = b"display_removed";
const DISPLAY_MOVED: &[u8; 13] = b"display_moved";
const DISPLAY_RESIZED: &[u8; 15] = b"display_resized";
const DISPLAY_CHANGED: &[u8; 15] = b"display_changed";

const MISSON_CONTROL_ENTER: &[u8; 21] = b"mission_control_enter";
const MISSON_CONTROL_EXIT: &[u8; 20] = b"mission_control_exit";

const APPLICATION_LAUNCHED: &[u8; 20] = b"application_launched";
const APPLICATION_TERMINATED: &[u8; 22] = b"application_terminated";
const APPLICATION_FRONT_SWITCHED: &[u8; 26] = b"application_front_switched";
const APPLICATION_ACTIVATED: &[u8; 21] = b"application_activated";
const APPLICATION_DEACTIVATED: &[u8; 23] = b"application_deactivated";
const APPLICATION_VISIBLE: &[u8; 19] = b"application_visible";
const APPLICATION_HIDDEN: &[u8; 18] = b"application_hidden";

impl TryFrom<&Bytes> for Event {
    type Error = Error;
    fn try_from(bytes: &Bytes) -> Result<Self, Error> {
        let val = &*bytes.as_ref();
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
            Self::Window(WindowEvent::Created)
        } else if SPACE_CHANGED == val {
            Self::Space(SpaceEvent::Changed)
        } else if APPLICATION_VISIBLE == val {
            Self::Application(ApplicationEvent::Visible)
        } else if APPLICATION_HIDDEN == val {
            Self::Application(ApplicationEvent::Hidden)
        } else if APPLICATION_ACTIVATED == val {
            Self::Application(ApplicationEvent::Activated)
        } else if APPLICATION_DEACTIVATED == val {
            Self::Application(ApplicationEvent::Deactivated)
        } else if APPLICATION_LAUNCHED == val {
            Self::Application(ApplicationEvent::Launched)
        } else if APPLICATION_TERMINATED == val {
            Self::Application(ApplicationEvent::Terminated)
        } else if APPLICATION_FRONT_SWITCHED == val {
            Self::Application(ApplicationEvent::FrontSwitched)
        } else if WINDOW_TITLE_CHANGED == val {
            Self::Window(WindowEvent::TitleChanged)
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
            Self::UnSupported
        };

        match event {
            Self::UnSupported => {
                let event = std::str::from_utf8(bytes)?;
                bail!("Event {event} is unsupported.")
            }
            _ => Ok(event),
        }
    }
}

#[test]
fn parse_string_to_event() {
    macro_rules! should_parse {
        ($str: expr, $type: ident, $check_method: ident) => {{
            let event_category = Bytes::from($str).split_off(6);
            match Event::try_from(&event_category) {
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

    should_parse!("event mission_control_exit", MissionControl, is_exit);
    should_parse!("event mission_control_enter", MissionControl, is_enter);
    should_parse!("event window_moved", Window, is_moved);
    should_parse!("event window_focused", Window, is_focused);
    should_parse!("event window_resized", Window, is_resized);
    should_parse!("event window_created", Window, is_created);
    should_parse!("event window_destroyed", Window, is_destroyed);
    should_parse!("event window_minimized", Window, is_minimized);
    should_parse!("event window_title_changed", Window, is_title_changed);
    should_parse!("event application_terminated", Application, is_terminated);
    should_parse!("event application_hidden", Application, is_hidden);
    should_parse!("event application_visible", Application, is_visible);
    should_parse!("event application_launched", Application, is_launched);
    should_parse!(
        "event application_front_switched",
        Application,
        is_front_switched
    );
    should_parse!("event application_activated", Application, is_activated);
    should_parse!("event application_deactivated", Application, is_deactivated);
    should_parse!("event space_changed", Space, is_changed);
    should_parse!("event display_changed", Display, is_changed);
    should_parse!("event display_added", Display, is_added);
    should_parse!("event display_moved", Display, is_moved);
    should_parse!("event display_removed", Display, is_removed);
    should_parse!("event display_resized", Display, is_resized);
}
