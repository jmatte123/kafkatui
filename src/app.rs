use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use kafka::client::{FetchPartition, KafkaClient};

#[derive(Debug, Clone, PartialEq)]
pub enum Widget {
  Topic,
  Broker,
  GroupId,
  MessageBox,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Consumer {
  pub topic: String,
  pub broker: String,
  pub group_id: String,
  pub message_box: String,
}

#[derive(Debug)]
pub struct App {
  pub widgets: Vec<Widget>,
  pub active_widget_index: usize,
  pub active_widget: Option<Widget>,
  pub consumer: Consumer,
  pub should_quit: bool,
}

impl App {
  pub fn new() -> Self {
    App {
      consumer: Consumer {
        topic: String::from(""),
        broker: String::from(""),
        group_id: String::from(""),
        message_box: String::from(""),
      },
      widgets: vec![Widget::Topic, Widget::Broker, Widget::GroupId],
      active_widget_index: 0,
      active_widget: None,
      should_quit: false,
    }
  }

  pub fn update(&mut self, key_event: KeyEvent) {
    match key_event.code {
      KeyCode::Esc => self.should_quit = true,
      KeyCode::Enter => {
        self.active_widget = Some(Widget::MessageBox);
        self.consume()
      }
      KeyCode::Char('j') if key_event.modifiers == KeyModifiers::CONTROL => {
        if self.active_widget_index < self.widgets.len() - 1 {
          self.active_widget_index += 1;
          self.active_widget = self.widgets.get(self.active_widget_index).cloned()
        }
      }
      KeyCode::Char('k') if key_event.modifiers == KeyModifiers::CONTROL => {
        if self.active_widget_index > 0 {
          self.active_widget_index -= 1;
          self.active_widget = self.widgets.get(self.active_widget_index).cloned()
        }
      }
      KeyCode::Backspace if self.active_widget != None => {
        self.get_active_buffer().pop();
      }
      KeyCode::Char(value)
        if self.active_widget != None && self.active_widget != Some(Widget::MessageBox) =>
      {
        self.get_active_buffer().push(value)
      }
      _ => {}
    };
  }

  pub fn get_active_buffer(&mut self) -> &mut String {
    if let Some(widget) = &self.active_widget {
      match widget {
        Widget::Topic => &mut self.consumer.topic,
        Widget::Broker => &mut self.consumer.broker,
        Widget::GroupId => &mut self.consumer.group_id,
        Widget::MessageBox => &mut self.consumer.message_box,
      }
    } else {
      &mut self.consumer.topic
    }
  }

  pub fn consume(&mut self) {
    self.consumer.message_box.push_str(&format!(
      "Starting consumer on Broker {} and Topic {}\n",
      self.consumer.broker, self.consumer.topic
    ));
    let mut client = kafka::client::KafkaClient::new(vec![self.consumer.broker.to_owned()]);
    client.load_metadata_all().unwrap();
    let reqs = &[FetchPartition::new(&self.consumer.topic, 0, 0)];
    let resps = client.fetch_messages(reqs).unwrap();
    for resp in resps {
      for t in resp.topics() {
        for p in t.partitions() {
          match p.data() {
            Err(ref e) => self.consumer.message_box.push_str(&format!(
              "partition error: {}:{}: {}\n",
              t.topic(),
              p.partition(),
              e
            )),
            Ok(ref data) => {
              self.consumer.message_box.push_str(&format!(
                "topic: {} / partition: {} / latest available message offset: {}\n",
                t.topic(),
                p.partition(),
                data.highwatermark_offset()
              ));
              for msg in data.messages() {
                self.consumer.message_box.push_str(&format!(
                  "topic: {} / partition: {} / message.offset: {} / message: {}\n",
                  t.topic(),
                  p.partition(),
                  msg.offset,
                  String::from_utf8(msg.value.to_vec()).unwrap()
                ));
              }
            }
          }
        }
      }
    }
    self.consumer.message_box.push_str("Stopped consuming\n")
  }
}
