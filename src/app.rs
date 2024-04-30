use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::widgets;

#[derive(Debug, Clone, Copy)]
pub enum Widget {
  Topic,
  GroupId,
}

#[derive(Debug)]
pub enum Mode {
  Normal,
  Editing,
}

#[derive(Debug)]
pub struct App {
  pub widgets: [Widget; 2],
  pub active_widget: Option<Widget>,
  pub mode: Mode,
  pub should_quit: bool,
}

impl App {
  pub fn new() -> Self {
    App {
      widgets: [Widget::Topic, Widget::GroupId],
      active_widget: None,
      mode: Mode::Editing,
      should_quit: false,
    }
  }

  pub fn update(&mut self, key_event: KeyEvent) {
    match key_event.code {
      KeyCode::Esc | KeyCode::Char('q') => self.should_quit = true,
      KeyCode::Char('j') => {
        if key_event.modifiers == KeyModifiers::CONTROL {
          self.active_widget = Some(self.widgets[0])
        }
      }
      KeyCode::Char('k') => {
        if key_event.modifiers == KeyModifiers::CONTROL {
          self.active_widget = Some(self.widgets[1])
        }
      }
      // KeyCode::Right | KeyCode::Char('j') => self.counter += 1,
      // KeyCode::Left | KeyCode::Char('k') => self.counter -= 1,
      _ => {}
    };
  }
}
