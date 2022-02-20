#![allow(dead_code)]

use crate::constants::*;
use anyhow::{bail, Error, Result};
use std::fmt::Debug;

#[derive(Debug)]
pub enum Event {
    NotSupported,
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
    pub fn is_enter_event(&self) -> bool {
        matches!(self, Self::Enter)
    }

    /// Returns `true` if the mission control event is [`Exit`].
    ///
    /// [`Exit`]: MissionControlEvent::Exit
    pub fn is_exit_event(&self) -> bool {
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
    pub fn is_change_event(&self) -> bool {
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
    pub fn is_add_event(&self) -> bool {
        matches!(self, Self::Added)
    }

    /// Returns `true` if the display event is [`Removed`].
    ///
    /// [`Removed`]: DisplayEvent::Removed
    pub fn is_remove_event(&self) -> bool {
        matches!(self, Self::Removed)
    }

    /// Returns `true` if the display event is [`Moved`].
    ///
    /// [`Moved`]: DisplayEvent::Moved
    pub fn is_move_event(&self) -> bool {
        matches!(self, Self::Moved)
    }

    /// Returns `true` if the display event is [`Resized`].
    ///
    /// [`Resized`]: DisplayEvent::Resized
    pub fn is_resize_event(&self) -> bool {
        matches!(self, Self::Resized)
    }

    /// Returns `true` if the display event is [`Changed`].
    ///
    /// [`Changed`]: DisplayEvent::Changed
    pub fn is_change_event(&self) -> bool {
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
    pub fn is_create_event(&self) -> bool {
        matches!(self, Self::Created)
    }

    /// Returns `true` if the window event is [`Destroyed`].
    ///
    /// [`Destroyed`]: WindowEvent::Destroyed
    pub fn is_destory_event(&self) -> bool {
        matches!(self, Self::Destroyed)
    }

    /// Returns `true` if the window event is [`Focused`].
    ///
    /// [`Focused`]: WindowEvent::Focused
    pub fn is_focus_event(&self) -> bool {
        matches!(self, Self::Focused)
    }

    /// Returns `true` if the window event is [`Moved`].
    ///
    /// [`Moved`]: WindowEvent::Moved
    pub fn is_move_event(&self) -> bool {
        matches!(self, Self::Moved)
    }

    /// Returns `true` if the window event is [`Resized`].
    ///
    /// [`Resized`]: WindowEvent::Resized
    pub fn is_resize_event(&self) -> bool {
        matches!(self, Self::Resized)
    }

    /// Returns `true` if the window event is [`Minimized`].
    ///
    /// [`Minimized`]: WindowEvent::Minimized
    pub fn is_minimize_event(&self) -> bool {
        matches!(self, Self::Minimized)
    }

    /// Returns `true` if the window event is [`Deminimized`].
    ///
    /// [`Deminimized`]: WindowEvent::Deminimized
    pub fn is_deminimize_event(&self) -> bool {
        matches!(self, Self::Deminimized)
    }

    /// Returns `true` if the window event is [`TitleChanged`].
    ///
    /// [`TitleChanged`]: WindowEvent::TitleChanged
    pub fn is_title_change_event(&self) -> bool {
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
    pub fn is_launch_event(&self) -> bool {
        matches!(self, Self::Launched)
    }

    /// Returns `true` if the application event is [`Terminated`].
    ///
    /// [`Terminated`]: ApplicationEvent::Terminated
    pub fn is_terminate_event(&self) -> bool {
        matches!(self, Self::Terminated)
    }

    /// Returns `true` if the application event is [`FrontSwitched`].
    ///
    /// [`FrontSwitched`]: ApplicationEvent::FrontSwitched
    pub fn is_front_switch_event(&self) -> bool {
        matches!(self, Self::FrontSwitched)
    }

    /// Returns `true` if the application event is [`Activated`].
    ///
    /// [`Activated`]: ApplicationEvent::Activated
    pub fn is_activate_event(&self) -> bool {
        matches!(self, Self::Activated)
    }

    /// Returns `true` if the application event is [`Deactivated`].
    ///
    /// [`Deactivated`]: ApplicationEvent::Deactivated
    pub fn is_deactivate_event(&self) -> bool {
        matches!(self, Self::Deactivated)
    }

    /// Returns `true` if the application event is [`Visible`].
    ///
    /// [`Visible`]: ApplicationEvent::Visible
    pub fn is_visible_event(&self) -> bool {
        matches!(self, Self::Visible)
    }

    /// Returns `true` if the application event is [`Hidden`].
    ///
    /// [`Hidden`]: ApplicationEvent::Hidden
    pub fn is_hidden_event(&self) -> bool {
        matches!(self, Self::Hidden)
    }
}

impl TryFrom<&Vec<u8>> for Event {
    type Error = Error;
    fn try_from(bytes: &Vec<u8>) -> Result<Self, Error> {
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
            let event_category = $str.as_bytes().to_vec();
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

    should_parse!("mission_control_exit", MissionControl, is_exit_event);
    should_parse!("mission_control_enter", MissionControl, is_enter_event);
    should_parse!("window_moved", Window, is_move_event);
    should_parse!("window_focused", Window, is_focus_event);
    should_parse!("window_resized", Window, is_resize_event);
    should_parse!("window_created", Window, is_create_event);
    should_parse!("window_destroyed", Window, is_destory_event);
    should_parse!("window_minimized", Window, is_minimize_event);
    should_parse!("window_deminimized", Window, is_deminimize_event);
    should_parse!("window_title_changed", Window, is_title_change_event);
    should_parse!("application_terminated", Application, is_terminate_event);
    should_parse!("application_hidden", Application, is_hidden_event);
    should_parse!("application_visible", Application, is_visible_event);
    should_parse!("application_launched", Application, is_launch_event);
    should_parse!(
        "application_front_switched",
        Application,
        is_front_switch_event
    );
    should_parse!("application_activated", Application, is_activate_event);
    should_parse!("application_deactivated", Application, is_deactivate_event);
    should_parse!("space_changed", Space, is_change_event);
    should_parse!("display_changed", Display, is_change_event);
    should_parse!("display_added", Display, is_add_event);
    should_parse!("display_moved", Display, is_move_event);
    should_parse!("display_removed", Display, is_remove_event);
    should_parse!("display_resized", Display, is_resize_event);
}
