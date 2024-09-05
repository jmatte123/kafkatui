/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// logging
pub mod logging;

use crate::logging::initialize_logging;
use app::App;
use color_eyre::Result;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;

#[tokio::main]
async fn main() -> Result<()> {
  initialize_logging()?;
  tracing::event!(tracing::Level::INFO, value = "value", test = 123);
 
  // Create an application.
  let mut app = App::new();

  // Initialize the terminal user interface.
  let backend = CrosstermBackend::new(std::io::stderr());
  let terminal = Terminal::new(backend)?;
  let events = EventHandler::new();
  let mut tui = Tui::new(terminal, events);
  tui.enter()?;

  // Start the main loop.
  while !app.should_quit {
    // Render the user interface.
    tui.draw(&mut app)?;
    // Handle events.
    match tui.events.next().await? {
      Event::Key(key_event) => app.update(key_event),
      Event::Error => {}
      Event::Tick => {}
    };
  }

  // Exit the user interface.
  tui.exit()?;
  Ok(())
}
