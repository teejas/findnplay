use std::{
    error::Error,
    path::PathBuf
};
use clap::Parser;


mod find;
mod input;
mod play;

use crate::input::UserControl;
use crate::find::{Cli, parse_args, search_directory};

fn main() -> Result<(), Box<dyn Error>> {

  let dir = parse_args(Cli::parse());
  let audiofiles: Vec<PathBuf> = search_directory(dir)?;
  println!("{:?}", audiofiles);

  if !audiofiles.is_empty() {
    let af_iter = audiofiles.iter();
    let mut ctrlr = UserControl::new(af_iter);
    let _ = ctrlr.capture_user_input();
  }

  Ok(())
}