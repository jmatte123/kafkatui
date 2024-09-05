use ratatui::{
  layout::{Constraint, Rect, Size},
  prelude::{Alignment, Layout},
  style::{Color, Style},
  widgets::{Block, Borders, Paragraph},
  Frame,
};
use tui_scrollview::ScrollView;

use crate::app::{App, Widget};

pub fn render(app: &mut App, f: &mut Frame) {
  let sub_layout = Layout::default()
    .margin(1)
    .constraints(vec![
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Length(3),
      Constraint::Fill(100),
    ])
    .split(Rect::new(0, 0, f.size().width, f.size().height));

  let help = Paragraph::new(String::from("Press ^j and ^k to move up and down"))
    .block(Block::default().borders(Borders::NONE));

  let topic = Paragraph::new(String::from(&app.consumer.topic)).block(
    Block::default()
      .title("topic")
      .title_alignment(Alignment::Left)
      .borders(Borders::ALL)
      .style(app.get_style(&Widget::Topic)),
  );

  let broker = Paragraph::new(String::from(&app.consumer.broker)).block(
    Block::default()
      .title("broker")
      .title_alignment(Alignment::Left)
      .borders(Borders::ALL)
      .style(app.get_style(&Widget::Broker)),
  );

  let group_id = Paragraph::new(String::from(&app.consumer.group_id)).block(
    Block::default()
      .title("group id")
      .title_alignment(Alignment::Left)
      .borders(Borders::ALL)
      .style(app.get_style(&Widget::GroupId)),
  );

  let message_box = Paragraph::new(String::from(&app.consumer.message_box)).block(
    Block::default()
      .title("messages")
      .title_alignment(Alignment::Left)
      .borders(Borders::ALL)
      .style(app.get_style(&Widget::MessageBox)),
  );
  let mut scroll_view = ScrollView::new(Size::new(
    sub_layout[4].width - 2,
    app.scroll_view_height.try_into().unwrap(),
  ));

  scroll_view.render_widget(
    message_box,
    Rect::new(
      0,
      0,
      sub_layout[4].width,
      app.scroll_view_height.try_into().unwrap(),
    ),
  );
  f.render_widget(help, sub_layout[0]);
  f.render_widget(topic, sub_layout[1]);
  f.render_widget(broker, sub_layout[2]);
  f.render_widget(group_id, sub_layout[3]);
  f.render_stateful_widget(scroll_view, sub_layout[4], &mut app.scroll_view_state);
}

impl App {
  fn get_style(&self, current_widget: &Widget) -> Style {
    if let Some(widget) = &self.active_widget {
      if widget == current_widget {
        Style::default().fg(Color::Cyan)
      } else {
        Style::default().fg(Color::Gray)
      }
    } else {
      Style::default().fg(Color::Gray)
    }
  }
}
