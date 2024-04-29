use std::io;

use color_eyre::Result;
use crossterm::{
  event::{DisableMouseCapture, EnableMouseCapture},
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

use crate::{app::App, event::EventHandler, ui};

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
pub struct Tui {
  /// Interface to the Terminal.
  terminal: CrosstermTerminal,
  /// Terminal event handler.
  pub events: EventHandler,
}

impl Tui {
  pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self {
    Self { terminal, events }
  }

  pub fn enter(&mut self) -> Result<()> {
    terminal::enable_raw_mode()?;
    crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;
    self.terminal.hide_cursor()?;
    self.terminal.clear()?;
    Ok(())
  }

  pub fn draw(&mut self, app: &mut App) -> Result<()> {
    self.terminal.draw(|frame| ui::render(app, frame))?;
    Ok(())
  }

  fn reset() -> Result<()> {
    terminal::disable_raw_mode()?;
    crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
  }

  pub fn exit(&mut self) -> Result<()> {
    Self::reset()?;
    self.terminal.show_cursor()?;
    Ok(())
  }
}
