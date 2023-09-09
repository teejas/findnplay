use std::{
  error::Error,
  path::PathBuf,
  slice::Iter,
  sync::mpsc::{self},
  thread
};
use crossterm::{cursor, event, execute, terminal};

use crate::play::play_audio;

pub struct UserControl<'a> {
  iter: Iter<'a, PathBuf>
}

impl Drop for UserControl<'_> {
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

impl UserControl<'_> {
  pub fn new(iter: Iter<PathBuf>) -> UserControl {
    terminal::enable_raw_mode();
    UserControl {
      iter
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

  pub fn capture_user_input(&mut self) -> Result<(), Box<dyn Error>> {
    let first_af = self.iter.next().unwrap().clone();
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
      println!("Working...");
      let _ = play_audio(&first_af, rx);
    });
    match UserControl::read_key()? {
      event::KeyEvent {
        code: event::KeyCode::Char('n'),
        ..
      } => {
        let _ = tx.send(());
        return self.capture_user_input();
      },
      _ => {
        return Ok(());
      }
    }
    Ok(())
  }
}