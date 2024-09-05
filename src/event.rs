use std::time::Duration;

use color_eyre::Result;
use crossterm::event::KeyEvent;
use futures::{FutureExt, StreamExt};

#[derive(Clone, Copy, Debug)]
pub enum Event {
  Key(KeyEvent),
  Error,
  Tick,
}

#[derive(Debug)]
pub struct EventHandler {
  _tx: tokio::sync::mpsc::UnboundedSender<Event>,
  rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
  pub fn new() -> Self {
    let tick_rate = Duration::from_millis(250);

    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let _tx = tx.clone();

    let _task = tokio::spawn(async move {
      let mut reader = crossterm::event::EventStream::new();
      let mut interval = tokio::time::interval(tick_rate);
      loop {
        let delay = interval.tick();
        let crossterm_event = reader.next().fuse();
        tokio::select! {
          maybe_event = crossterm_event => {
            match maybe_event {
              Some(Ok(evt)) => {
                match evt {
                  crossterm::event::Event::Key(key) => {
                    if key.kind == crossterm::event::KeyEventKind::Press {
                      tx.send(Event::Key(key)).unwrap();
                    }
                  },
                  _ => {},
                }
              }
              Some(Err(_)) => {
                tx.send(Event::Error).unwrap();
              }
              None => {},
            }
          },
          _ = delay => {
              tx.send(Event::Tick).unwrap();
          },
        }
      }
    });

    Self {
      _tx,
      rx,
      // task: Some(task),
    }
  }

  pub async fn next(&mut self) -> Result<Event> {
    self
      .rx
      .recv()
      .await
      .ok_or(color_eyre::eyre::eyre!("Unable to get event"))
  }
}
