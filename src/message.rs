use crossterm::event::{KeyEvent, MouseEvent};
use serde::{Deserialize, Serialize};

/// Represents various messages that can be sent to an asynchronous channel
/// for processing within the application. Each message corresponds to a specific action
/// or event that the application needs to handle.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Message {
  /// Indicates that the application has started and is ready for initialization tasks.
  Init,

  /// Signals that the application should terminate gracefully.
  Quit,

  /// Represents an error that has occurred within the application.
  Error,

  /// Indicates that the current view or context should be closed.
  Closed,

  /// Triggers a refresh of the application state or view.
  Tick,

  /// Requests a re-render of the application's user interface.
  Render,

  /// Indicates that the application has gained focus and should resume its activities.
  FocusGained,

  /// Signals that the application has lost focus and should pause or suspend its activities.
  FocusLost,

  /// Contains a string of text that has been pasted into the application.
  Paste(String),

  /// Represents a keyboard event, used to handle keyboard input.
  Key(KeyEvent),

  /// Represents a mouse event, used to handle mouse input.
  Mouse(MouseEvent),

  /// Indicates that the application window should resize to the specified dimensions.
  Resize(u16, u16),
}

