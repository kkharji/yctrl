#![allow(dead_code)]

use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum MissionControlEvent {
    /// Triggered when mission-control activates.
    Enter,
    /// Triggered when mission-control deactivates.
    Exit,
}

impl MissionControlEvent {
    /// Returns `true` if the mission control event is [`Enter`].
    ///
    /// i.e. did mission control activate?
    ///
    /// [`Enter`]: MissionControlEvent::Enter
    pub fn is_enter_event(&self) -> bool {
        matches!(self, Self::Enter)
    }

    /// Returns `true` if the mission control event is [`Exit`].
    ///
    /// i.e. did mission control deactivates?
    ///
    /// [`Exit`]: MissionControlEvent::Exit
    pub fn is_exit_event(&self) -> bool {
        matches!(self, Self::Exit)
    }
}

#[derive(Debug)]
pub enum SpaceEvent {
    /// Triggered when the active space has changed.
    ///
    /// Passes two arguments: $YABAI_SPACE_ID, $YABAI_RECENT_SPACE_ID
    Changed { space_id: u32, recent_space_id: u32 },
}

impl SpaceEvent {
    /// Returns `true` if the space event is [`Changed`].
    ///
    /// i.e. did active space change?
    ///
    /// [`Changed`]: SpaceEvent::Changed
    pub fn is_change_event(&self) -> bool {
        matches!(
            self,
            Self::Changed {
                space_id: _,
                recent_space_id: _
            }
        )
    }
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

impl DisplayEvent {
    /// Returns `true` if the display event is [`Added`].
    ///
    /// i.e. did a new display got attached?
    ///
    /// [`Added`]: DisplayEvent::Added
    pub fn is_add_event(&self) -> bool {
        matches!(self, Self::Added)
    }

    /// Returns `true` if the display event is [`Removed`].
    ///
    /// i.e. did a display got removed?
    ///
    /// [`Removed`]: DisplayEvent::Removed
    pub fn is_remove_event(&self) -> bool {
        matches!(self, Self::Removed)
    }

    /// Returns `true` if the display event is [`Moved`].
    ///
    /// i.e. did a display got moved?
    ///
    /// [`Moved`]: DisplayEvent::Moved
    pub fn is_move_event(&self) -> bool {
        matches!(self, Self::Moved)
    }

    /// Returns `true` if the display event is [`Resized`].
    ///
    /// i.e. did a display got resized?
    ///
    /// [`Resized`]: DisplayEvent::Resized
    pub fn is_resize_event(&self) -> bool {
        matches!(self, Self::Resized)
    }

    /// Returns `true` if the display event is [`Changed`].
    ///
    /// i.e. did focus switch to different display
    ///
    /// [`Changed`]: DisplayEvent::Changed
    pub fn is_change_event(&self) -> bool {
        matches!(self, Self::Changed)
    }
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

impl WindowEvent {
    /// Returns `true` if the window event is [`Created`].
    ///
    /// i.e. a new window created in a current space.
    ///
    /// [`Created`]: WindowEvent::Created
    pub fn is_create_event(&self) -> bool {
        matches!(self, Self::Created { window_id: _ })
    }

    /// Returns `true` if the window event is [`Destroyed`].
    ///
    /// i.e. a window destroyed in a current space.
    ///
    /// [`Destroyed`]: WindowEvent::Destroyed
    pub fn is_destory_event(&self) -> bool {
        matches!(self, Self::Destroyed { window_id: _ })
    }

    /// Returns `true` if the window event is [`Focused`].
    ///
    /// i.e. a new window gained focus
    ///
    /// [`Focused`]: WindowEvent::Focused
    pub fn is_focus_event(&self) -> bool {
        matches!(self, Self::Focused { window_id: _ })
    }

    /// Returns `true` if the window event is [`Moved`].
    ///
    /// i.e. a window position has changed
    ///
    /// [`Moved`]: WindowEvent::Moved
    pub fn is_move_event(&self) -> bool {
        matches!(self, Self::Moved { window_id: _ })
    }

    /// Returns `true` if the window event is [`Resized`].
    ///
    /// i.e. a window has been resized
    ///
    /// [`Resized`]: WindowEvent::Resized
    pub fn is_resize_event(&self) -> bool {
        matches!(self, Self::Resized { window_id: _ })
    }

    /// Returns `true` if the window event is [`Minimized`].
    ///
    /// i.e. a window got minimized
    ///
    /// [`Minimized`]: WindowEvent::Minimized
    pub fn is_minimize_event(&self) -> bool {
        matches!(self, Self::Minimized { window_id: _ })
    }

    /// Returns `true` if the window event is [`Deminimized`].
    ///
    /// i.e. a window got deminimized
    ///
    /// [`Deminimized`]: WindowEvent::Deminimized
    pub fn is_deminimize_event(&self) -> bool {
        matches!(self, Self::Deminimized { window_id: _ })
    }

    // Returns `true` if the window event is [`TitleChanged`].
    //
    // i.e. a window got title got updated.
    //
    // [`TitleChanged`]: WindowEvent::TitleChanged
    // pub fn is_title_change_event(&self) -> bool {
    //     matches!(self, Self::TitleChanged)
    // }
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
    Deactivated,
    /// Triggered when an application is unhidden.
    ///
    /// Passes one argument: $YABAI_PROCESS_ID
    Visible,
    /// Triggered when an application is hidden.
    ///
    /// Passes one argument: $YABAI_PROCESS_ID
    Hidden,
}

impl ApplicationEvent {
    /// Returns `true` if the application event is [`Launched`].
    ///
    /// i.e. a new application launched
    ///
    /// [`Launched`]: ApplicationEvent::Launched
    // pub fn is_launch_event(&self) -> bool {
    //     matches!(self, Self::Launched)
    // }

    /// Returns `true` if the application event is [`Terminated`].
    ///
    /// i.e. an application got terminated.
    ///
    /// [`Terminated`]: ApplicationEvent::Terminated
    // pub fn is_terminate_event(&self) -> bool {
    //     matches!(self, Self::Terminated)
    // }

    /// Returns `true` if the application event is [`FrontSwitched`].
    ///
    /// i.e. an application became ontop?
    ///
    /// [`FrontSwitched`]: ApplicationEvent::FrontSwitched
    // pub fn is_front_switch_event(&self) -> bool {
    //     matches!(self, Self::FrontSwitched)
    // }

    /// Returns `true` if the application event is [`Activated`].
    ///
    /// i.e. an application got activated
    ///
    /// [`Activated`]: ApplicationEvent::Activated
    // pub fn is_activate_event(&self) -> bool {
    //     matches!(self, Self::Activated)
    // }

    /// Returns `true` if the application event is [`Deactivated`].
    ///
    /// i.e. an application got deactivated
    ///
    /// [`Deactivated`]: ApplicationEvent::Deactivated
    pub fn is_deactivate_event(&self) -> bool {
        matches!(self, Self::Deactivated)
    }

    /// Returns `true` if the application event is [`Visible`].
    ///
    /// i.e. an application got unhidden or made visible.
    ///
    /// [`Visible`]: ApplicationEvent::Visible
    pub fn is_visible_event(&self) -> bool {
        matches!(self, Self::Visible)
    }

    /// Returns `true` if the application event is [`Hidden`].
    ///
    /// i.e. an application got hidden.
    ///
    /// [`Hidden`]: ApplicationEvent::Hidden
    pub fn is_hidden_event(&self) -> bool {
        matches!(self, Self::Hidden)
    }
}
