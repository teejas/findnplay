use rodio::{Decoder, OutputStream};
use rodio::source::{Source};
use std::{
  error::Error,
  fs,
  io::{BufReader},
  path::PathBuf,
  sync::mpsc::{TryRecvError, Receiver},
  thread,
  time::Duration
};

pub fn play_audio<T>(af: &PathBuf, rx: Receiver<T>) -> Result<(), Box<dyn Error>> {
  // Get a output stream handle to the default physical sound device
  let (_stream, stream_handle) = OutputStream::try_default().unwrap();
  // let sink = Sink::try_new(&stream_handle).unwrap();
  let file = BufReader::new(fs::File::open(af)?);
  // Decode that sound file into a source
  let source = Decoder::new(file)?;
  let _ = stream_handle.play_raw(source.convert_samples());

  // The sound plays in a separate audio thread,
  // so we need to keep the main thread alive while it's playing.
  loop {
    thread::sleep(Duration::from_millis(250));
    match rx.try_recv() {
      Ok(_) | Err(TryRecvError::Disconnected) => {
        println!("Terminating.");
        break;
      }
      Err(TryRecvError::Empty) => {}
    }
  }
  Ok(())
}