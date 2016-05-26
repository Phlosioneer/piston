
//! Back-end agnostic mouse events.

use std::any::Any;

use { GenericEvent, MOUSE_SCROLL, MOUSE_RELATIVE, MOUSE_CURSOR };

/// Represent a mouse button press.
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq,
    Eq, Ord, PartialOrd, Hash, Debug)]
pub enum MouseButton {
    /// Unknown mouse button.
    ///
    /// See your specific backend's documentation for situations where this
    /// might be returned.
    Unknown,
    /// Left mouse button.
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button.
    Middle,
    /// Extra mouse button number 1.
    X1,
    /// Extra mouse button number 2.
    X2,
    /// Mouse button number 6.
    Button6,
    /// Mouse button number 7.
    Button7,
    /// Mouse button number 8.
    Button8,
}

/// Convenience function for converting from a number into an instance of the
/// Mouse Button enum.
// TODO: Why exactly is this a u32...?
impl From<u32> for MouseButton {
    fn from(n: u32) -> MouseButton {
        match n {
            0 => MouseButton::Unknown,
            1 => MouseButton::Left,
            2 => MouseButton::Right,
            3 => MouseButton::Middle,
            4 => MouseButton::X1,
            5 => MouseButton::X2,
            6 => MouseButton::Button6,
            7 => MouseButton::Button7,
            8 => MouseButton::Button8,
            _ => MouseButton::Unknown,
        }
    }
}

/// Convenience function for converting from an instance of the
/// Mouse Button enum into a number.
// TODO: Why exactly is this a u32...?
impl From<MouseButton> for u32 {
    fn from(button: MouseButton) -> u32 {
        match button {
            MouseButton::Unknown => 0,
            MouseButton::Left => 1,
            MouseButton::Right => 2,
            MouseButton::Middle => 3,
            MouseButton::X1 => 4,
            MouseButton::X2 => 5,
            MouseButton::Button6 => 6,
            MouseButton::Button7 => 7,
            MouseButton::Button8 => 8,
        }
    }
}

#[cfg(test)]
mod mouse_button_tests {
    use super::*;
	
	// Test that the encoding and decoding of Mouse Button <---> u32 works.
    #[test]
    fn test_mouse_button_primitives() {
        for i in 0u32..9 {
            let button: MouseButton = i.into();
            let j: u32 = button.into();
            assert_eq!(i, j);
        }
    }
    
    // TODO: There is room for improvement here, to increase test coverage.
}

/// An event that gives the position of the mouse cursor relative to the 
/// window origin.
pub trait MouseCursorEvent: Sized {
    /// Creates a mouse cursor event.
    fn from_xy(x: f64, y: f64, old_event: &Self) -> Option<Self>;
    
    /// Maps a function onto this event, if this is a `MouseEvent`.
    ///
    /// Calls closure if the event is a mouse event, and is not None.
    /// The closure will be given the (x, y) coordinates of the mouse.
    /// Returns None if the event is None, or if the event encodes a
    /// different type of event.
    fn mouse_cursor<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U;
        
    /// Returns the mouse (x,y) coordinates.
    ///
    /// If this event isn't a `MouseCursorEvent`, returns None.
    ///
    /// #Errors
	///
	/// Panics if `mouse_cursor` would panic.
    fn mouse_cursor_args(&self) -> Option<[f64; 2]> {
        self.mouse_cursor(|x, y| [x, y])
    }
}

impl<T: GenericEvent> MouseCursorEvent for T {
	/// Creates a `MouseCursorEvent`.
	///
	/// Never returns None.
    fn from_xy(x: f64, y: f64, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(MOUSE_CURSOR, &(x, y) as &Any, old_event)
    }
	
	/// Maps a function onto this event, if this is a `MouseCursorEvent`.
	/// 
	/// Returns None if and only if this is not a `MouseCursorEvent`.
	///
	/// #Errors
	///
	/// Panics if the event doesn't contain an (x,y) pair. This panic is
	/// only possible because the type information for the contained data is
	/// erased via `std::any::Any`.
    fn mouse_cursor<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U
    {
        if self.event_id() != MOUSE_CURSOR {
            return None;
        }
        self.with_args(|any| {
            if let Some(&(x, y)) = any.downcast_ref::<(f64, f64)>() {
                Some(f(x, y))
            } else {
                panic!("Expected (f64, f64)")
            }
        })
    }
}

/// An event that gives the movement of mouse cursor, relative to the last
/// position of the cursor.
pub trait MouseRelativeEvent: Sized {
    /// Creates a `MouseRelativeEvent`.
    fn from_xy(x: f64, y: f64, old_event: &Self) -> Option<Self>;
    
    /// Maps a function onto this event, `MouseRelativeEvent`, otherwise
    /// returns None.
    fn mouse_relative<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U;
        
    /// Returns the mouse motion, relative to its previous location.
    /// 
    /// If this event isn't a `MouseRelativeEvent`, returns None.
    ///
    /// #Errors
	///
	/// Panics if `mouse_relative` would panic.
    fn mouse_relative_args(&self) -> Option<[f64; 2]> {
        self.mouse_relative(|x, y| [x, y])
    }
}

impl<T: GenericEvent> MouseRelativeEvent for T {
	/// Creates a `MouseRelativeEvent`.
	///
	/// Never returns None.
	// TODO: If this never returns none, why does it return an optional?
    fn from_xy(x: f64, y: f64, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(MOUSE_RELATIVE, &(x, y) as &Any, old_event)
    }
	
	/// Maps a function onto this event, if this is a `MouseRelativeEvent	`.
	/// 
	/// Returns None if and only if this is not a `MouseRelativeEvent	`.
	///
	/// #Errors
	///
	/// Panics if the event doesn't contain an (x,y) pair. This panic is
	/// only possible because the type information for the contained data is
	/// erased via `std::any::Any`.
    fn mouse_relative<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U
    {
        if self.event_id() != MOUSE_RELATIVE {
            return None;
        }
        self.with_args(|any| {
            if let Some(&(x, y)) = any.downcast_ref::<(f64, f64)>() {
                Some(f(x, y))
            } else {
                panic!("Expected (f64, f64)")
            }
        })
    }
}

/// An event that gives the x-scrolling and y-scrolling of the window.
///
/// Normally, mice only have y scrolling. However, touch pads and ball-mice
/// can generate x-direction scrolling events, so both coordinates are included.
///
/// The units of the values in this event are defined per-backend, though they
/// are probably in pixels. See your specific backend's documentation for more
/// information.
pub trait MouseScrollEvent: Sized {
    /// Creates a `MouseScrollEvent`.
    fn from_xy(x: f64, y: f64, old_event: &Self) -> Option<Self>;
    
    /// Maps a function onto this event, if this is a `MouseScrollEvent`.
    /// Otherwise, returns None.
    fn mouse_scroll<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U;
    
    /// Returns mouse scroll arguments, if this is a `MouseScrollEvent`.
    /// 
    /// If this is not a `MouseScrollEvent`, returns None.
    ///
    /// #Errors
    ///
    /// Panics if mouse_scroll would panic.
    fn mouse_scroll_args(&self) -> Option<[f64; 2]> {
        self.mouse_scroll(|x, y| [x, y])
    }
}

impl<T: GenericEvent> MouseScrollEvent for T {
	/// Creates a `MouseScrollEvent`.
	///
	/// Never returns None.
	// TODO: If this never returns none, why does it return an optional?
    fn from_xy(x: f64, y: f64, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(MOUSE_SCROLL, &(x, y) as &Any, old_event)
    }
	
	/// Maps a function onto this event, if this is a `MouseScrollEvent`.
    /// Otherwise, returns None.
    ///
    /// #Errors
    ///
    /// Panics if the event doesn't contain an (x,y) pair. This panic is
	/// only possible because the type information for the contained data is
	/// erased via `std::any::Any`.
    fn mouse_scroll<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U
    {
        if self.event_id() != MOUSE_SCROLL {
            return None;
        }
        self.with_args(|any| {
            if let Some(&(x, y)) = any.downcast_ref::<(f64, f64)>() {
                Some(f(x, y))
            } else {
                panic!("Expected (f64, f64)")
            }
        })
    }
}

#[cfg(test)]
mod mouse_event_tests {
    use super::*;
	
	// Various tests to ensure that the two methods of constructing these events
	// produce the same results.
    #[test]
    fn test_input_mouse_cursor() {
        use super::super::{ Input, Motion };

        let e = Input::Move(Motion::MouseCursor(0.0, 0.0));
        let a: Option<Input> = MouseCursorEvent::from_xy(1.0, 0.0, &e);
        let b: Option<Input> = a.clone().unwrap().mouse_cursor(|x, y|
            MouseCursorEvent::from_xy(x, y, a.as_ref().unwrap())).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_event_mouse_cursor() {
        use Event;
        use super::super::{ Input, Motion };

        let e = Event::Input(Input::Move(Motion::MouseCursor(0.0, 0.0)));
        let a: Option<Event> = MouseCursorEvent::from_xy(1.0, 0.0, &e);
        let b: Option<Event> = a.clone().unwrap().mouse_cursor(|x, y|
            MouseCursorEvent::from_xy(x, y, a.as_ref().unwrap())).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_input_mouse_relative() {
        use super::super::{ Input, Motion };

        let e = Input::Move(Motion::MouseRelative(0.0, 0.0));
        let a: Option<Input> = MouseRelativeEvent::from_xy(1.0, 0.0, &e);
        let b: Option<Input> = a.clone().unwrap().mouse_relative(|x, y|
            MouseRelativeEvent::from_xy(x, y, a.as_ref().unwrap())).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_event_mouse_relative() {
        use Event;
        use super::super::{ Input, Motion };

        let e = Event::Input(Input::Move(Motion::MouseRelative(0.0, 0.0)));
        let a: Option<Event> = MouseRelativeEvent::from_xy(1.0, 0.0, &e);
        let b: Option<Event> = a.clone().unwrap().mouse_relative(|x, y|
            MouseRelativeEvent::from_xy(x, y, a.as_ref().unwrap())).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_input_mouse_scroll() {
        use super::super::{ Input, Motion };

        let e = Input::Move(Motion::MouseScroll(0.0, 0.0));
        let a: Option<Input> = MouseScrollEvent::from_xy(1.0, 0.0, &e);
        let b: Option<Input> = a.clone().unwrap().mouse_scroll(|x, y|
            MouseScrollEvent::from_xy(x, y, a.as_ref().unwrap())).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_event_mouse_scroll() {
        use Event;
        use super::super::{ Input, Motion };

        let e = Event::Input(Input::Move(Motion::MouseScroll(0.0, 0.0)));
        let a: Option<Event> = MouseScrollEvent::from_xy(1.0, 0.0, &e);
        let b: Option<Event> = a.clone().unwrap().mouse_scroll(|x, y|
            MouseScrollEvent::from_xy(x, y, a.as_ref().unwrap())).unwrap();
        assert_eq!(a, b);
    }
}
