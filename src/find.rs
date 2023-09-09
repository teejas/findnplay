use std::{
  error::Error,
  fs,
  path::PathBuf
};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
  /// The path to the directory to search for audio files in.
  #[arg(short, long)]
  dir: PathBuf,
}

pub fn parse_args(args: Cli) -> PathBuf {
  args.dir
}

// searches directory and returns a vector of audio pathbufs
pub fn search_directory(dir: PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> { 
  let mut ret: Vec<PathBuf> = Vec::new();
  for entry in fs::read_dir(dir)? {
    let entry = entry?;
    let path = entry.path();
    let metadata = fs::metadata(&path)?;
    if metadata.is_file() {
      if let Some(ext) = path.extension() {
        if ext == "wav" || ext == "mp3" {
          ret.push(path);
        }
      }
  }
  }
  Ok(ret)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn search_dir_with_no_audio_files() {
    let ret = search_directory(PathBuf::from(".")).unwrap();
    let check: Vec<PathBuf> = Vec::new();
    assert_eq!(ret, check);
  }
  #[test]
  fn search_dir_with_one_audio_file() {
    let mut ret = search_directory(PathBuf::from("examples/")).unwrap();
    ret.sort();
    assert_eq!(ret, 
      vec![
        PathBuf::from("examples/mypianothingilike.mp3"), 
        PathBuf::from("examples/shorter_cc_hornlinez.wav")
      ]
    );
  }
}