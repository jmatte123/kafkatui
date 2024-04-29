use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug)]
pub enum ActiveWidget { Topic, GroupId}

#[derive(Debug, Default)]
pub struct App {
  pub active_widget: Option<ActiveWidget>,
  pub should_quit: bool,
}

impl App {
  pub fn new() -> Self {
    App {
      active_widget: None,
      should_quit: false,
    }
  }

  pub fn tick(&self) {}

  pub fn update(&mut self, key_event: KeyEvent) {
    match key_event.code {
      KeyCode::Esc | KeyCode::Char('q') => self.should_quit = true,
      KeyCode::Char('h') => {
        if key_event.modifiers == KeyModifiers::CONTROL {
          self.should_quit = true
        }
      }
      KeyCode::Char('l') => {
        if key_event.modifiers == KeyModifiers::CONTROL {
          self.should_quit = true
        }
      }
      // KeyCode::Right | KeyCode::Char('j') => self.counter += 1,
      // KeyCode::Left | KeyCode::Char('k') => self.counter -= 1,
      _ => {}
    };
  }
}
