use ratatui::{
  layout::{Constraint, Rect},
  prelude::{Alignment, Layout},
  style::{Color, Style},
  widgets::{Block, Borders, Paragraph},
  Frame,
};

use crate::app::{App, Widget};

pub fn render(app: &App, f: &mut Frame) {
  let sub_layout = Layout::default()
    .margin(1)
    .constraints(vec![
      Constraint::Percentage(20),
      Constraint::Percentage(40),
      Constraint::Percentage(40),
    ])
    .split(Rect::new(0, 0, f.size().width, 10));

  let help = Paragraph::new(String::from("Press ^j and ^k to move up and down"))
    .block(Block::default().borders(Borders::NONE));

  let topic = Paragraph::new(String::from(&app.consumer.topic)).block(
    Block::default()
      .title("topic")
      .title_alignment(Alignment::Left)
      .borders(Borders::ALL)
      .style(app.get_style(&Widget::Topic)),
  );

  let group_id = Paragraph::new(String::from(&app.consumer.group_id)).block(
    Block::default()
      .title("group id")
      .title_alignment(Alignment::Left)
      .borders(Borders::ALL)
      .style(app.get_style(&Widget::GroupId)),
  );

  f.render_widget(help, sub_layout[0]);
  f.render_widget(topic, sub_layout[1]);
  f.render_widget(group_id, sub_layout[2]);
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
