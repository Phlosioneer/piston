use std::borrow::ToOwned;
use std::any::Any;

use { GenericEvent, TEXT };

/// An event that gives text from user, such as a character.
pub trait TextEvent: Sized {
    /// Creates a `TextEvent`.
    fn from_text(text: &str, old_event: &Self) -> Option<Self>;
    
    /// Maps a function onto this event, if this is a `TextEvent`.
    ///
    /// Calls closure if the event is a `TextEvent`, and is not None.
    /// The closure will be given the (x, y) coordinates of the mouse.
    /// Returns None if the event is None, or if the event encodes a
    /// different type of event.
    fn text<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(&str) -> U;
    
    /// Returns text string if this is a `TextEvent`.
    ///
    /// #Panics
    ///
    /// Panics if `text` would panic.
    fn text_args(&self) -> Option<String> {
        self.text(|text| text.to_owned())
    }
}

impl<T: GenericEvent> TextEvent for T {
	/// Creates a `TextEvent`.
	///
	/// Never returns None.
    fn from_text(text: &str, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(TEXT, &text.to_owned() as &Any, old_event)
    }
	
	/// Maps a function onto this event, if this is a `TextEvent`.
	/// 
	/// Returns None if and only if this is not a `TextEvent`.
	///
	/// #Panics
	///
	/// Panics if the event doesn't contain a string. This panic is
	/// only possible because the type information for the contained data is
	/// erased via `std::any::Any`.
    fn text<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&str) -> U
    {
        if self.event_id() != TEXT {
            return None;
        }
        self.with_args(|any| {
            if let Some(text) = any.downcast_ref::<String>() {
                Some(f(&text))
            } else {
                panic!("Expected &str")
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_text() {
        use super::super::Input;

        let e = Input::Text("".to_string());
        let x: Option<Input> = TextEvent::from_text("hello", &e);
        let y: Option<Input> = x.clone().unwrap().text(|text|
            TextEvent::from_text(text, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }


    #[test]
    fn test_event_text() {
        use Event;
        use super::super::Input;

        let e = Event::Input(Input::Text("".to_string()));
        let x: Option<Event> = TextEvent::from_text("hello", &e);
        let y: Option<Event> = x.clone().unwrap().text(|text|
            TextEvent::from_text(text, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }
}
