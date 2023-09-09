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

pub fn play_audio(af: &PathBuf) -> Result<(), Box<dyn Error>> {
  // Get a output stream handle to the default physical sound device
  let (_stream, stream_handle) = OutputStream::try_default()?;
  // Load a sound from a file, using a path relative to Cargo.toml
  let file = BufReader::new(fs::File::open(af)?);
  // Decode that sound file into a source
  let source = Decoder::new(file)?;
  // Play the sound directly on the device
  stream_handle.play_raw(source.convert_samples());
  Ok(())
}