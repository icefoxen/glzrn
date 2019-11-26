//! New api fiddling goes here
use std::path::PathBuf;

use crate::dpi::{LogicalPosition, LogicalSize};
use crate::event;
use crate::event_loop;
use crate::window::WindowId;

/// Describes a generic event.
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    /// Emitted when the OS sends an event to a winit window.
    WindowEvent {
        window_id: WindowId,
        event: WindowEvent,
    },
    /// Emitted when the OS sends an event to a device.
    DeviceEvent {
        device_id: event::DeviceId,
        event: event::DeviceEvent,
    },
    /// Emitted when all of the event loop's events have been processed and control flow is about
    /// to be taken away from the program.
    EventsCleared,

    /// Emitted when the event loop is being shut down. This is irreversable - if this event is
    /// emitted, it is guaranteed to be the last event emitted.
    LoopDestroyed,

    /// Emitted when the application has been suspended.
    Suspended,

    /// Emitted when the application has been resumed.
    Resumed,
}

/// Describes an event from a `Window`.
#[derive(Clone, Debug, PartialEq)]
pub enum WindowEvent {
    /// The size of the window has changed. Contains the client area's new dimensions.
    Resized(LogicalSize),

    /// The position of the window has changed. Contains the window's new position.
    Moved(LogicalPosition),

    /// The window has been requested to close.
    CloseRequested,

    /// The window has been destroyed.
    Destroyed,

    /// A file has been dropped into the window.
    ///
    /// When the user drops multiple files at once, this event will be emitted for each file
    /// separately.
    DroppedFile(PathBuf),

    /// A file is being hovered over the window.
    ///
    /// When the user hovers multiple files at once, this event will be emitted for each file
    /// separately.
    HoveredFile(PathBuf),

    /// A file was hovered, but has exited the window.
    ///
    /// There will be a single `HoveredFileCancelled` event triggered even if multiple files were
    /// hovered.
    HoveredFileCancelled,

    /// The window received a unicode character.
    ReceivedCharacter(char),

    /// The window gained or lost focus.
    ///
    /// The parameter is true if the window has gained focus, and false if it has lost focus.
    Focused(bool),

    /// An event from the keyboard has been received.
    KeyboardInput {
        device_id: event::DeviceId,
        input: event::KeyboardInput,
    },

    /// The cursor has moved on the window.
    MouseMoved {
        device_id: event::DeviceId,

        /// (x,y) coords in pixels relative to the top-left corner of the window. Because the range of this data is
        /// limited by the display area and it may have been transformed by the OS to implement effects such as cursor
        /// acceleration, it should not be used to implement non-cursor-like interactions such as 3D camera control.
        position: LogicalPosition,
        modifiers: event::ModifiersState,
    },

    /// The cursor has entered the window.
    MouseEntered { device_id: event::DeviceId },

    /// The cursor has left the window.
    MouseLeft { device_id: event::DeviceId },

    /// A mouse wheel movement or touchpad scroll occurred.
    MouseWheel {
        device_id: event::DeviceId,
        delta: event::MouseScrollDelta,
        phase: event::TouchPhase,
        modifiers: event::ModifiersState,
    },

    /// An mouse button press has been received.
    MouseInput {
        device_id: event::DeviceId,
        state: event::ElementState,
        button: event::MouseButton,
        modifiers: event::ModifiersState,
        position: LogicalPosition,
    },

    /// Touchpad pressure event.
    ///
    /// At the moment, only supported on Apple forcetouch-capable macbooks.
    /// The parameters are: pressure level (value between 0 and 1 representing how hard the touchpad
    /// is being pressed) and stage (integer representing the click level).
    TouchpadPressure {
        device_id: event::DeviceId,
        pressure: f32,
        stage: i64,
    },

    /// Motion on some analog axis. May report data redundant to other, more specific events.
    AxisMotion {
        device_id: event::DeviceId,
        axis: event::AxisId,
        value: f64,
    },

    /// The OS or application has requested that the window be redrawn.
    RedrawRequested,

    /// Touch event has been received
    Touch(event::Touch),

    /// The DPI factor of the window has changed.
    ///
    /// The following user actions can cause DPI changes:
    ///
    /// * Changing the display's resolution.
    /// * Changing the display's DPI factor (e.g. in Control Panel on Windows).
    /// * Moving the window to a display with a different DPI factor.
    ///
    /// For more information about DPI in general, see the [`dpi`](crate::dpi) module.
    HiDpiFactorChanged(f64),
}

/*
/// Represents raw hardware events that are not associated with any particular window.
///
/// Useful for interactions that diverge significantly from a conventional 2D GUI, such as 3D camera or first-person
/// game controls. Many physical actions, such as mouse movement, can produce both device and window events. Because
/// window events typically arise from virtual devices (corresponding to GUI cursors and keyboard focus) the device IDs
/// may not match.
///
/// Note that these events are delivered regardless of input focus.
#[derive(Clone, Debug, PartialEq)]
pub enum DeviceEvent {
    Added,
    Removed,

    /// Change in physical position of a pointing device.
    ///
    /// This represents raw, unfiltered physical motion. Not to be confused with `WindowEvent::CursorMoved`.
    MouseMotion {
        /// (x, y) change in position in unspecified units.
        ///
        /// Different devices may use different units.
        delta: (f64, f64),
    },

    /// Physical scroll event
    MouseWheel {
        delta: MouseScrollDelta,
    },

    /// Motion on some analog axis.  This event will be reported for all arbitrary input devices
    /// that winit supports on this platform, including mouse devices.  If the device is a mouse
    /// device then this will be reported alongside the MouseMotion event.
    Motion {
        axis: AxisId,
        value: f64,
    },

    Button {
        button: ButtonId,
        state: ElementState,
    },

    Key(KeyboardInput),

    /// Keyboard modifiers have changed
    #[doc(hidden)]
    ModifiersChanged {
        modifiers: ModifiersState,
    },

    Text {
        codepoint: char,
    },
}
*/

pub trait EventHandler {
    fn handle(&mut self, _event: Event);
}

pub struct Heck {
    el: event_loop::EventLoop<()>,
}

impl Heck {
    pub fn poll_events(&mut self) -> impl Iterator<Item = Event> {
        None.into_iter()
    }

    pub fn run(self, handler: impl EventHandler + 'static) {
        // we own handler we can do what ever we want to it
        let mut handler = handler;
        self.el.run(move |event, _, controlflow| {
            *controlflow = event_loop::ControlFlow::Poll;
            match event {
                event::Event::WindowEvent { window_id, event } => {
                    //handler.handle(Event::WindowEvent { window_id, event })
                }
                event::Event::DeviceEvent { device_id, event } => {
                    handler.handle(Event::DeviceEvent { device_id, event })
                }
                event::Event::EventsCleared => handler.handle(Event::EventsCleared),
                event::Event::LoopDestroyed => handler.handle(Event::LoopDestroyed),

                event::Event::Suspended => handler.handle(Event::Suspended),

                event::Event::Resumed => handler.handle(Event::Resumed),
                _ => (),
            }
        });
    }
}
