use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, PartialEq)]
pub enum Widget {
  Topic,
  GroupId,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Consumer {
  pub topic: String,
  pub group_id: String,
}

#[derive(Debug)]
pub struct App {
  pub active_widget: Option<Widget>,
  pub consumer: Consumer,
  pub should_quit: bool,
}

impl App {
  pub fn new() -> Self {
    App {
      consumer: Consumer {
        topic: String::from(""),
        group_id: String::from(""),
      },
      active_widget: None,
      should_quit: false,
    }
  }

  pub fn update(&mut self, key_event: KeyEvent) {
    match key_event.code {
      KeyCode::Esc => self.should_quit = true,
      KeyCode::Char('j') if key_event.modifiers == KeyModifiers::CONTROL => {
        self.active_widget = Some(Widget::GroupId)
      }
      KeyCode::Char('k') if key_event.modifiers == KeyModifiers::CONTROL => {
        self.active_widget = Some(Widget::Topic)
      }
      KeyCode::Backspace if self.active_widget != None => {
        self.get_active_buffer().pop();
      }
      KeyCode::Char(value) if self.active_widget != None => self.get_active_buffer().push(value),
      _ => {}
    };
  }

  pub fn get_active_buffer(&mut self) -> &mut String {
    if let Some(widget) = &self.active_widget {
      match widget {
        Widget::Topic => &mut self.consumer.topic,
        Widget::GroupId => &mut self.consumer.group_id,
      }
    } else {
      &mut self.consumer.topic
    }
  }
}
