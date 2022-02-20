#![allow(dead_code)]

pub enum Event {
    MissionControl(MissionControlEvent),
    Window(WindowEvent),
    Display(DisplayEvent),
    Space(SpaceEvent),
    Application(ApplicationEvent),
}

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

pub enum WindowEvent {
    Created(u32),
    Destroyed(u32),
    Focused(u32),
    Moved(u32),
    Resized(u32),
    Minimized(u32),
    Deminimized,
    TitleChanged,
}

impl WindowEvent {
    /// Returns `true` if the window event is [`Created`].
    ///
    /// [`Created`]: WindowEvent::Created
    pub fn is_created(&self) -> bool {
        matches!(self, Self::Created(..))
    }

    /// Returns `true` if the window event is [`Destroyed`].
    ///
    /// [`Destroyed`]: WindowEvent::Destroyed
    pub fn is_destroyed(&self) -> bool {
        matches!(self, Self::Destroyed(..))
    }

    /// Returns `true` if the window event is [`Focused`].
    ///
    /// [`Focused`]: WindowEvent::Focused
    pub fn is_focused(&self) -> bool {
        matches!(self, Self::Focused(..))
    }

    /// Returns `true` if the window event is [`Moved`].
    ///
    /// [`Moved`]: WindowEvent::Moved
    pub fn is_moved(&self) -> bool {
        matches!(self, Self::Moved(..))
    }

    /// Returns `true` if the window event is [`Resized`].
    ///
    /// [`Resized`]: WindowEvent::Resized
    pub fn is_resized(&self) -> bool {
        matches!(self, Self::Resized(..))
    }

    /// Returns `true` if the window event is [`Minimized`].
    ///
    /// [`Minimized`]: WindowEvent::Minimized
    pub fn is_minimized(&self) -> bool {
        matches!(self, Self::Minimized(..))
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
