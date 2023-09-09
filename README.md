# Introduction

A simple CLI tool, which when given a directory will search it for audio files and play them back.

## Steps
1. Build simple command line arg parser, should just take one flag (a directory)
2. Search directory for audio files (start with hardcoding search for .mp3 and .wav files)
3. Playback using `hound` or smn
4. Allow user to control playback, pause and skip through files in the directory
5. Build some kind of GUI/TUI to show user what song is playing and progress
6. Give user the option to recursively search through directories, with a -r flag prob