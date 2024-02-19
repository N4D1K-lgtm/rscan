use std::{
  ops::{Deref, DerefMut},
  time::Duration,
};

use color_eyre::eyre::Result;
use crossterm::{
  cursor,
  event::{DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture, Event},
  terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{FutureExt, StreamExt};
use ratatui::backend::CrosstermBackend as Backend;
use tokio::{
  sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
  task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

use crate::message::Message;

/// Configuration options for the TUI.
#[derive(Debug, Clone)]
pub struct TuiOptions {
  /// The rate at which the TUI should update, in ticks per second.
  pub tick_rate: f64,
  /// The frame rate of the TUI, in frames per second.
  pub frame_rate: f64,
  /// Whether mouse support is enabled.
  pub mouse: bool,
  /// Whether paste support is enabled.
  pub paste: bool,
}

impl Default for TuiOptions {
  fn default() -> Self {
    Self { tick_rate: 4.0, frame_rate: 60.0, mouse: true, paste: true }
  }
}

/// The main struct for the TUI, handling event loop and rendering.
pub struct Tui {
  /// The underlying terminal instance.
  pub terminal: ratatui::Terminal<Backend<std::io::Stderr>>,
  /// The handle for the async task running the event loop.
  pub task: JoinHandle<()>,
  /// A token used to signal cancellation of the event loop.
  pub cancellation_token: CancellationToken,
  /// The receiver for messages from the event loop.
  pub event_rx: UnboundedReceiver<Message>,
  /// The sender for messages to the event loop.
  pub event_tx: UnboundedSender<Message>,
  /// Configuration options for the TUI.
  pub options: TuiOptions,
}
impl Tui {
  /// Creates a new Tui instance with default options.
  pub fn new() -> Result<Self> {
    let terminal = ratatui::Terminal::new(Backend::new(std::io::stderr()))?;

    let (event_tx, event_rx) = mpsc::unbounded_channel();
    let cancellation_token = CancellationToken::new();

    let task = tokio::spawn(async {});

    let options = TuiOptions::default();

    Ok(Self { terminal, task, cancellation_token, event_rx, event_tx, options })
  }

  /// Sets the Tui options.
  pub fn options(mut self, options: TuiOptions) -> Self {
    self.options = options;
    self
  }

  /// Sets the tick rate for the Tui.
  pub fn tick_rate(mut self, tick_rate: f64) -> Self {
    self.options.tick_rate = tick_rate;
    self
  }

  /// Sets the frame rate for the Tui.
  pub fn frame_rate(mut self, frame_rate: f64) -> Self {
    self.options.frame_rate = frame_rate;
    self
  }

  /// Enables or disables mouse support.
  pub fn mouse(mut self, mouse: bool) -> Self {
    self.options.mouse = mouse;
    self
  }

  /// Enables or disables paste support.
  pub fn paste(mut self, paste: bool) -> Self {
    self.options.paste = paste;
    self
  }

  /// Starts the event loop for the TUI.
  ///
  /// This method initiates the event loop that handles rendering and user input. It sets up intervals
  /// for ticking and rendering based on the configured rates, and listens for events from crossterm.
  /// The loop continues until the cancellation token is triggered. Events are processed and sent to
  /// the event channel for handling by the application.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use rscantui::tui::Tui;
  ///
  /// let mut tui = Tui::new().unwrap();
  /// tui.start();
  /// ```
  ///
  /// # Panics
  ///
  /// This method panics if it fails to send a message to the event channel.
  pub fn start(&mut self) {
    // Calculate the delay intervals for ticking and rendering based on the configured rates.
    let tick_delay = std::time::Duration::from_secs_f64(1.0 / self.options.tick_rate);
    let render_delay = std::time::Duration::from_secs_f64(1.0 / self.options.frame_rate);

    // Cancel any previous event loop and create a new cancellation token.
    self.cancel();
    self.cancellation_token = CancellationToken::new();

    // Clone the cancellation token and event sender for use in the async task.
    let _cancellation_token = self.cancellation_token.clone();
    let _event_tx = self.event_tx.clone();

    // Spawn an asynchronous task to run the event loop.
    self.task = tokio::spawn(async move {
      // Create a new event stream for reading crossterm events.
      let mut reader = crossterm::event::EventStream::new();

      // Set up intervals for ticking and rendering.
      let mut tick_interval = tokio::time::interval(tick_delay);
      let mut render_interval = tokio::time::interval(render_delay);

      // Send an initialization message to the event channel.
      _event_tx.send(Message::Init).unwrap();

      // Event loop: continues until the cancellation token is triggered.
      loop {
        // Wait for the next tick, render interval, or crossterm event.
        let tick_delay = tick_interval.tick();
        let render_delay = render_interval.tick();
        let crossterm_event = reader.next().fuse();

        // Use `tokio::select` to handle multiple asynchronous events.
        tokio::select! {
            // If the cancellation token is triggered, break out of the loop.
            _ = _cancellation_token.cancelled() => {
                break;
            }
            // Handle crossterm events.
            maybe_event = crossterm_event => {
                match maybe_event {
                    Some(Ok(evt)) => {
                        let message = Message::from(evt);
                        _event_tx.send(message).unwrap();
                    }
                    // Send an error message if there's an error reading an event.
                    Some(Err(_)) => {
                        _event_tx.send(Message::Error).unwrap();
                    }
                    // Do nothing if there are no events.
                    None => {},
                }
            },
            // Send a tick message at each tick interval.
            _ = tick_delay => {
                _event_tx.send(Message::Tick).unwrap();
            },
            // Send a render message at each render interval.
            _ = render_delay => {
                _event_tx.send(Message::Render).unwrap();
            },
        }
      }
    });
  }

  /// Stops the event loop and cleans up resources.
  pub fn stop(&self) -> Result<()> {
    self.cancel();
    let mut counter = 0;

    while !self.task.is_finished() {
      std::thread::sleep(Duration::from_millis(1));
      counter += 1;
      if counter > 50 {
        self.task.abort();
      }
      if counter > 100 {
        log::error!("Failed to abort task in 100 milliseconds for unknown reason");
        break;
      }
    }
    Ok(())
  }

  /// Prepares the terminal for TUI rendering.
  ///
  /// This method enables raw mode, enters the alternate screen, and hides the cursor. It also
  /// enables mouse and paste support if configured. After setting up the terminal, it starts the
  /// event loop for handling user input and rendering.
  ///
  /// # Errors
  ///
  /// Returns an error if any issues occur while setting up the terminal or starting the event loop.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use rscantui::tui::Tui;
  ///
  /// let mut tui = Tui::new().unwrap();
  /// tui.enter().unwrap();
  /// // The terminal is now ready for rendering and input handling.
  /// ```
  pub fn enter(&mut self) -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), EnterAlternateScreen, cursor::Hide)?;
    if self.options.mouse {
      crossterm::execute!(std::io::stderr(), EnableMouseCapture)?;
    }
    if self.options.paste {
      crossterm::execute!(std::io::stderr(), EnableBracketedPaste)?;
    }
    self.start();
    Ok(())
  }

  /// Restores the terminal to its original state.
  ///
  /// This method stops the event loop, restores the cursor, exits the alternate screen, and disables
  /// raw mode. It also disables mouse and paste support if they were enabled. This method is
  /// typically called when exiting the TUI to ensure the terminal is left in a clean state.
  ///
  /// # Errors
  ///
  /// Returns an error if any issues occur while restoring the terminal.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use rscantui::tui::Tui;
  ///
  /// let mut tui = Tui::new().unwrap();
  /// tui.enter().unwrap();
  /// // Perform some operations...
  /// tui.exit().unwrap();
  /// // The terminal is now restored to its original state.
  /// ```
  pub fn exit(&mut self) -> Result<()> {
    self.stop()?;

    if crossterm::terminal::is_raw_mode_enabled()? {
      self.flush()?;
      if self.options.paste {
        crossterm::execute!(std::io::stderr(), DisableBracketedPaste)?;
      }
      if self.options.mouse {
        crossterm::execute!(std::io::stderr(), DisableMouseCapture)?;
      }

      crossterm::execute!(std::io::stderr(), LeaveAlternateScreen, cursor::Show)?;

      crossterm::terminal::disable_raw_mode()?;
    }
    Ok(())
  }

  /// Cancels the event loop.
  ///
  /// This method triggers the cancellation token, which signals the event loop to stop. It is
  /// should be called internally by other methods like [`stop`](Tui::stop) or [`exit`](Tui::exit)
  /// when the TUI needs to be shut down.
  pub fn cancel(&self) {
    self.cancellation_token.cancel();
  }

  /// Suspends the TUI and restores the terminal.
  ///
  /// This method is intended to be used when the TUI needs to be temporarily suspended, for example,
  /// when handling a SIGTSTP signal for terminal suspension. It exits the TUI, restoring the terminal
  /// to its original state, and then sends a SIGTSTP signal to suspend the process.
  ///
  /// # Errors
  ///
  /// Returns an error if any issues occur while suspending the TUI or restoring the terminal.
  ///
  /// # Examples
  ///
  /// ```rust no_run
  /// use rscantui::tui::Tui;
  ///
  /// let mut tui = Tui::new().unwrap();
  /// tui.enter().unwrap();
  /// // Perform some operations...
  /// tui.suspend().unwrap();
  /// // The TUI is now suspended, and the terminal is restored.
  /// ```
  pub fn suspend(&mut self) -> Result<()> {
    self.exit()?;
    #[cfg(not(windows))]
    signal_hook::low_level::raise(signal_hook::consts::signal::SIGTSTP)?;
    Ok(())
  }

  /// Resumes the TUI after suspension.
  ///
  /// This method is intended to be used when the TUI needs to be resumed after being suspended. It
  /// re-enters the TUI, preparing the terminal for rendering and input handling again.
  ///
  /// # Errors
  ///
  /// Returns an error if any issues occur while resuming the TUI.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use rscantui::tui::Tui;
  ///
  /// let mut tui = Tui::new().unwrap();
  /// tui.enter().unwrap();
  /// // Perform some operations...
  /// tui.suspend().unwrap();
  /// // The TUI is now suspended.
  /// tui.resume().unwrap();
  /// // The TUI is now resumed and ready for rendering and input handling.
  /// ```
  pub fn resume(&mut self) -> Result<()> {
    self.enter()?;
    Ok(())
  }

  /// Waits for the next message from the event loop.
  ///
  /// This asynchronous method waits for the next message from the event loop and returns it. It is
  /// typically used in the main loop of the application to
  /// process events and update the UI accordingly.
  ///
  /// # Returns
  ///
  /// Returns Some(Message) if a message is received, or None if the channel is closed.
  ///
  /// # Examples
  ///
  /// ```rust no_run
  /// use rscantui::tui::Tui;
  ///
  /// #[tokio::main]
  /// async fn main() {
  ///   let mut tui = Tui::new().unwrap();
  ///   tui.enter().unwrap();
  ///   while let Some(message) = tui.next().await {
  ///       // Handle the message...
  ///   }
  /// }
  /// ```
  pub async fn next(&mut self) -> Option<Message> {
    self.event_rx.recv().await
  }
}

/// [`Deref`] trait impl for [`Tui`]. This allows immutable access to the underlying terminal. For
/// example in a closure like so:
///
/// ```rust
/// use ratatui::widgets::*;
/// use rscantui::tui::Tui;
///
/// let tui = Tui::new().unwrap().enter().unwrap();
///
/// let widget = Block::default();
///
/// // instead of using:
///
/// // tui.terminal.draw(|f| {
/// //    render_widget(f, widget);
/// //})
///
/// // we can use:
///
/// tui.draw(|f| {
///   render_widget(f, widget);
/// });
/// ```
impl Deref for Tui {
  type Target = ratatui::Terminal<Backend<std::io::Stderr>>;

  fn deref(&self) -> &Self::Target {
    &self.terminal
  }
}

/// [`DerefMut`] trait impl for [`Tui`]. This allows mutable access to the underlying terminal. For
/// example in a closure like so:
/// ```rust
/// use ratatui::widgets::*;
/// use rscantui::tui::Tui;
///
/// let mut tui = Tui::new().unwrap();
/// tui.enter().unwrap();
///
/// let widget = Block::default();
///
/// // instead of using:
///
/// // tui.terminal.draw(|f| {
/// //    render_widget(f, widget);
/// //})
///
/// // we can use:
///
/// tui.draw(|f| {
///   render_widget(f, widget);
/// });
/// ```
impl DerefMut for Tui {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.terminal
  }
}

/// [`Drop`] trait impl for [`Tui`]. This will restore the terminal to its original state when
/// [`Tui`] goes out of scope.
impl Drop for Tui {
  fn drop(&mut self) {
    self.exit().unwrap();
  }
}

/// [`From`] trait impl for [`Event`] to [`Message`]. Allows using
/// [`Event::into`] and [`Message::from`].
///
/// ```rust
/// use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
/// use rscantui::message::Message;
///
/// let key_event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty());
/// let event = Event::Key(KeyEvent);
///
/// let message_into: Message = event.into();
/// let message_from = Message::from(event);
///
/// assert_eq!(message_into, Message::Key(key_event));
/// assert_eq!(message_from, Message::Key(key_event));
/// assert_eq!(message_into, message_from);
/// ```
impl From<Event> for Message {
  fn from(event: Event) -> Self {
    match event {
      Event::Key(key) => Message::Key(key),
      Event::Mouse(mouse) => Message::Mouse(mouse),
      Event::Resize(x, y) => Message::Resize(x, y),
      Event::FocusLost => Message::FocusLost,
      Event::FocusGained => Message::FocusGained,
      Event::Paste(s) => Message::Paste(s),
    }
  }
}
