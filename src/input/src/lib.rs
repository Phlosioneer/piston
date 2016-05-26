#![crate_name = "input"]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! A flexible structure for user interactions
//! to be used in window frameworks and widgets libraries.

#[macro_use]
extern crate bitflags;
extern crate rustc_serialize;
extern crate viewport;

pub use controller::{ ControllerAxisArgs, ControllerButton };
pub use keyboard::Key;
pub use mouse::MouseButton;

pub mod controller;
pub mod keyboard;
pub mod mouse;

pub use after_render::{ AfterRenderArgs, AfterRenderEvent };
pub use controller::{ ControllerAxisEvent };
pub use cursor::CursorEvent;
pub use event::Event;
pub use focus::FocusEvent;
pub use generic_event::GenericEvent;
pub use idle::{ IdleArgs, IdleEvent };
pub use mouse::{ MouseCursorEvent, MouseRelativeEvent, MouseScrollEvent };
pub use press::PressEvent;
pub use release::ReleaseEvent;
pub use resize::ResizeEvent;
pub use render::{ RenderArgs, RenderEvent };
pub use text::TextEvent;
pub use touch::{ Touch, TouchArgs, TouchEvent };
pub use update::{ UpdateArgs, UpdateEvent };

pub mod generic_event;

mod after_render;
mod cursor;
mod event;
mod focus;
mod idle;
mod press;
mod release;
mod render;
mod resize;
mod text;
mod touch;
mod update;

/// Used to identify events arguments provided by traits.
// TODO: What...?
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct EventId(pub &'static str);

const AFTER_RENDER: EventId = EventId("piston/after_render");
const CONTROLLER_AXIS: EventId = EventId("piston/controller_axis");
const CURSOR: EventId = EventId("piston/cursor");
const FOCUS: EventId = EventId("piston/focus");
const IDLE: EventId = EventId("piston/idle");
const MOUSE_SCROLL: EventId = EventId("piston/mouse_scroll");
const MOUSE_RELATIVE: EventId = EventId("piston/mouse_relative");
const MOUSE_CURSOR: EventId = EventId("piston/mouse_cursor");
const PRESS: EventId = EventId("piston/press");
const RELEASE: EventId = EventId("piston/release");
const RENDER: EventId = EventId("piston/render");
const RESIZE: EventId = EventId("piston/resize");
const TEXT: EventId = EventId("piston/text");
const TOUCH: EventId = EventId("piston/touch");
const UPDATE: EventId = EventId("piston/update");

/// Models each kind of button that might be used by a backend.
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq, Eq, Hash, Debug)]
pub enum Button {
    /// A keyboard button.
    Keyboard(Key),
    /// A mouse button.
    Mouse(MouseButton),
    /// A controller button.
    Controller(ControllerButton),
}

/// Models each kind of input motion, from mouse pointers to joysticks.
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq, Debug)]
pub enum Motion {
    /// Gives the mouse position (x, y) in window coordinates.
    MouseCursor(f64, f64),
    
    /// Gives the mouse position (x, y) in relative coordinates.
    // TODO: Relative to what?
    MouseRelative(f64, f64),
    
    /// Gives the scroll bar position for x and y directions
    /// in scroll ticks.
    // TODO: What controlls tick size?
    MouseScroll(f64, f64),
    
    /// Used when the axis of a joystick or a controller's analog stick moves.
    ControllerAxis(ControllerAxisArgs),
    
    /// Used for touch events.
    Touch(TouchArgs),
}

/// Models piston's default input events.
///
/// Most backends will use this enum instead of providing their own input-handling
/// enum. It's used during the Event Loop.
// TODO: How to link to the event loop? ../event_loop/index.html is broken if
//       documentation is only built for the input module.
#[derive(Clone, RustcDecodable, RustcEncodable, PartialEq, Debug)]
pub enum Input {
    /// The user pressed a button.
    Press(Button),
    /// The user released a button.
    Release(Button),
    /// The user moved the mouse cursor, a joystick, or there was a touch event.
    Move(Motion),
    /// Text. This will usually be full unicode or characters, as opposed to single
    /// keypresses. May also be used by backends that don't support individual
    /// key presses.
    Text(String),
    /// The window was resized. Gives the new (height, width) in pixels.
    Resize(u32, u32),
    /// If true, the window gained focus.
    Focus(bool),
    /// If true, the cursor just entered the window area. Otherwise, the cursor just
    /// left the window area.
    Cursor(bool),
}

/// Convenience method for making a Button wrapper around a key input.
impl From<Key> for Button {
    fn from(key: Key) -> Self {
        Button::Keyboard(key)
    }
}

/// Convinience method for making a Button wrapper around a mouse input.
impl From<MouseButton> for Button {
    fn from(btn: MouseButton) -> Self {
        Button::Mouse(btn)
    }
}

/// Convinience method for making a Button wrapper around a controller input.
impl From<ControllerButton> for Button {
    fn from(btn: ControllerButton) -> Self {
        Button::Controller(btn)
    }
}

/// Convinience method for making a Motion wrapper around a controller axis input.
impl From<ControllerAxisArgs> for Motion {
    fn from(args: ControllerAxisArgs) -> Self {
        Motion::ControllerAxis(args)
    }
}

/// Convinience method for making an Input event around a Motion event.
impl From<Motion> for Input {
    fn from(motion: Motion) -> Self {
        Input::Move(motion)
    }
}

// TOTO: It seems like the conversions are arbitrary... it should probably be all
//       the conversions the user could want, or no conversions.


