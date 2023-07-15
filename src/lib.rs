//! # Purpose
//!
//! Provides custom serialization/deserialization which is more readable than
//! the existing serialization/deserialization of the KeyEvent from crossterm
//!
//! In my view this is better suited for configuration file where the user tweaks the value to change
//! shortcuts.
//!
//! # Example
//!```
//! use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
//! use crossterm_serde::SerDeConfigKeyEvent;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
//! pub struct KeyBoard {
//!     #[serde(with = "SerDeConfigKeyEvent")]
//!     move_up: KeyEvent,
//!     #[serde(with = "SerDeConfigKeyEvent")]
//!     move_down: KeyEvent,
//! }
//! fn main() {
//!     let key_board = KeyBoard {
//!         move_up: KeyEvent::new(
//!             KeyCode::Char('a'),
//!             KeyModifiers::NONE | KeyModifiers::ALT | KeyModifiers::CONTROL,
//!         ),
//!         move_down: KeyEvent::new(KeyCode::Up, KeyModifiers::ALT),
//!     };
//!
//!     let string = serde_json::to_string_pretty(&key_board).unwrap();
//!     assert_eq!(
//!         r#"{
//!  "move_up": {
//!    "code": "a",
//!    "modifiers": "ALT+CONTROL"
//!  },
//!  "move_down": {
//!    "code": "Up",
//!    "modifiers": "ALT"
//!  }
//!}"#,
//!         &string
//!     );
//!
//!     let back_from_str: KeyBoard =
//!         serde_json::from_str(&string).expect("Should be converted back from the text");
//!     assert_eq!(key_board, back_from_str);
//! }
//!```

mod key_event_serde;
pub use key_event_serde::SerDeConfigKeyEvent;
