use std::any::Any;

use { GenericEvent, RESIZE };

/// An event that is triggered when the window is resized.
pub trait ResizeEvent: Sized {
    /// Creates a `ResizeEvent`.
    fn from_width_height(w: u32, h: u32, old_event: &Self) -> Option<Self>;
    
    /// Maps a function onto this event, if this is a mouse event.
    ///
    /// Calls closure if the event is a mouse event, and is not None.
    /// The closure will be given the (x, y) coordinates of the mouse.
    /// Returns None if the event is None, or if the event encodes a
    /// different type of event.
    fn resize<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(u32, u32) -> U;
    
    /// If this is a `ResizeEvent`, returns the new (width, height).
    ///
    /// Otherwise, returns None.
    ///
    /// #Errors
    ///
    /// Panics if `resize` panics.
    fn resize_args(&self) -> Option<[u32; 2]> {
        self.resize(|x, y| [x, y])
    }
}

impl<T: GenericEvent> ResizeEvent for T {
	/// Creates a `ResizeEvent`.
    fn from_width_height(w: u32, h: u32, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(RESIZE, &(w, h) as &Any, old_event)
    }
	
	/// Maps a function onto this event, if this is a mouse event.
    ///
    /// Calls closure if the event is a mouse event, and is not None.
    /// The closure will be given the (x, y) coordinates of the mouse.
    /// Returns None if the event is None, or if the event encodes a
    /// different type of event.
    ///
    /// #Errors
	///
	/// Panics if the event doesn't contain an (x,y) pair. This panic is
	/// only possible because the type information for the contained data is
	/// erased via `std::any::Any`.
    fn resize<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(u32, u32) -> U
    {
        if self.event_id() != RESIZE {
            return None;
        }
        self.with_args(|any| {
            if let Some(&(w, h)) = any.downcast_ref::<(u32, u32)>() {
                Some(f(w, h))
            } else {
                panic!("Expected (u32, u32)")
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_resize() {
        use super::super::Input;

        let e = Input::Resize(0, 0);
        let x: Option<Input> = ResizeEvent::from_width_height(100, 100, &e);
        let y: Option<Input> = x.clone().unwrap().resize(|w, h|
            ResizeEvent::from_width_height(w, h, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }

    #[test]
    fn test_event_resize() {
        use Event;
        use super::super::Input;

        let e = Event::Input(Input::Resize(0, 0));
        let x: Option<Event> = ResizeEvent::from_width_height(100, 100, &e);
        let y: Option<Event> = x.clone().unwrap().resize(|w, h|
            ResizeEvent::from_width_height(w, h, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }
}
