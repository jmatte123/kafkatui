use color_eyre::owo_colors::{style, OwoColorize};
use ratatui::{
  layout::{Constraint, Direction, Rect},
  prelude::{Alignment, Frame, Layout},
  style::{Color, Style},
  widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::{App, Widget};

pub fn render(app: &mut App, f: &mut Frame) {
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

  let topic = Block::default()
    .title("topic")
    .title_alignment(Alignment::Left)
    .borders(Borders::ALL)
    .style(app.widgets[0].get_style());

  let group_id = Block::default()
    .title("group id")
    .title_alignment(Alignment::Left)
    .borders(Borders::ALL)
    .style(app.widgets[1].get_style());

  f.render_widget(help, sub_layout[0]);
  f.render_widget(topic, sub_layout[1]);
  f.render_widget(group_id, sub_layout[2]);

  // f.render_widget(
  //   Paragraph::new(format!(
  //     "
  //       Press `j` and `k` to increment and decrement the counter respectively.\n\
  //     ",
  //   ))
  //   .block(
  //     Block::default()
  //       .title("Counter App")
  //       .title_alignment(Alignment::Center)
  //       .borders(Borders::ALL)
  //       .border_type(BorderType::Rounded),
  //   )
  //   .style(Style::default().fg(Color::Yellow))
  //   .alignment(Alignment::Center),
  //   parent_layout[1],
  // )
}

impl Widget {
  fn get_style(&self) -> Style {
    match self {
       Widget::Topic { value, is_active } => check_is_active(*is_active),
       Widget::GroupId { value, is_active } => check_is_active(*is_active)
    }
  }
}

  fn check_is_active(is_active: bool) -> Style {
    if is_active {
      Style::default().fg(Color::Cyan)
    } else {
      Style::default().fg(Color::Gray)
    }
  }
