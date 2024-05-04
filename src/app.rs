use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, PartialEq)]
pub enum Widget {
  Topic {
    value: Option<String>,
    is_active: bool,
  },
  GroupId {
    value: Option<String>,
    is_active: bool,
  },
}

#[derive(Debug)]
pub struct App {
  pub widgets: [Widget; 2],
  pub active_widget: Option<Widget>,
  pub buffer: String,
  pub should_quit: bool,
}

impl App {
  pub fn new() -> Self {
    App {
      widgets: [
        Widget::Topic {
          value: None,
          is_active: false,
        },
        Widget::GroupId {
          value: None,
          is_active: false,
        },
      ],
      active_widget: None,
      buffer: String::from(""),
      should_quit: false,
    }
  }

  pub fn update(&mut self, key_event: KeyEvent) {
    match key_event.code {
      KeyCode::Esc => self.should_quit = true,
      KeyCode::Char('j') if key_event.modifiers == KeyModifiers::CONTROL => {
        if let Widget::Topic {
          ref mut is_active, ..
        } = self.widgets[0]
        {
          *is_active = false
        }

        if let Widget::GroupId {
          ref mut is_active, ..
        } = self.widgets[1]
        {
          *is_active = true;
          self.active_widget = Some(self.widgets[1].clone())
        }
      }
      KeyCode::Char('k') if key_event.modifiers == KeyModifiers::CONTROL => {
        if let Widget::Topic {
          ref mut is_active, ..
        } = self.widgets[0]
        {
          *is_active = true;
          self.active_widget = Some(self.widgets[1].clone())
        }

        if let Widget::GroupId {
          ref mut is_active, ..
        } = self.widgets[1]
        {
          *is_active = false
        }
      }

      // KeyCode::Right | KeyCode::Char('j') => self.counter += 1,
      // KeyCode::Left | KeyCode::Char('k') => self.counter -= 1,
      KeyCode::Backspace if self.active_widget != None => {
        self.buffer.pop();
      }
      KeyCode::Char(value) if self.active_widget != None => self.buffer.push(value),

      _ => {}
    };
  }
}
