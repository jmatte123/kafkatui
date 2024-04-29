use color_eyre::owo_colors::{style, OwoColorize};
use ratatui::{
  layout::{Constraint, Direction},
  prelude::{Alignment, Frame, Layout},
  style::{Color, Style},
  widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
  let parent_layout = Layout::default()
    .constraints(vec![Constraint::Percentage(100)])
    .split(f.size());

  let block = Block::default()
    .title("Consumers")
    .title_alignment(Alignment::Center)
    .borders(Borders::ALL)
    .style(Style::default().fg(Color::Cyan));

  f.render_widget(block, parent_layout[0]);

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
