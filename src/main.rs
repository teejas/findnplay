use std::{
    error::Error,
    path::PathBuf
};
use clap::Parser;
use crossterm::{terminal};

mod find;
mod input;
mod play;
use crate::play::play_audio;
use crate::input::{capture_user_input, CleanUp};
use crate::find::{Cli, parse_args, search_directory};

fn main() -> Result<(), Box<dyn Error>> {
  let _clean_up = CleanUp;

  let dir = parse_args(Cli::parse());
  let audiofiles: Vec<PathBuf> = search_directory(dir)?;
  println!("{:?}", audiofiles);

  let mut af_iter = audiofiles.iter();
  terminal::enable_raw_mode()?;
  let _ = capture_user_input(af_iter);

  Ok(())
}