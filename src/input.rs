use std::{
  error::Error,
  io::BufReader,
  fs,
  path::PathBuf,
  slice::Iter
};
use clap::Parser;
use crossterm::{cursor, event, execute, terminal};
use rodio::{Decoder, OutputStream, source::Source};

use crate::play::play_audio;

pub struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Failed to disable raw-mode for terminal");
        execute!(
            std::io::stdout(), 
            terminal::Clear(terminal::ClearType::All)
        ).expect("Error clearing the screen on exit");
        execute!(
            std::io::stdout(), 
            cursor::MoveTo(0,0),
        ).expect("Error resetting the cursor to (0,0) on exit");
    }
}

fn read_key() -> Result<event::KeyEvent, Box<dyn Error>> {
  loop {
      if event::poll(std::time::Duration::from_millis(5000))? {
          if let event::Event::Key(event) = event::read()? {
              return Ok(event);
          }
      }
  }
}

pub fn capture_user_input(mut iter: Iter<PathBuf>) -> Result<(), Box<dyn Error>> {
  match read_key()? {
      event::KeyEvent {
          code: event::KeyCode::Char('n'),
          ..
      } => {
          let _ = play_audio(iter.next().unwrap());
          return capture_user_input(iter);
      },
      _ => {
          return Ok(());
      }
  }
  Ok(())
}