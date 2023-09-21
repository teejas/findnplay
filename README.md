# Introduction

A simple CLI tool, which when given a directory will search it for audio files and play them back. This repo contains two implementations of it, just the CLI in Rust, and both the CLI and a web server in Go.

I'm thinking of re-implementing the [`betterjenkins`](https://www.github.com/teejas/betterjenkins) controller in Go, reasons being I want faster build times for the controller and it just makes more sense to build a Kubernetes-based project in Go rather than Rust.

As a way to familiarize myself with Go again, I'm going to build a web extension of the [`findnplay`](https://www.github.com/teejas/findnplay) CLI tool I previously built.

# Steps
## Go
- [ ] Re-implement `findnplay` CLI tool in Go
- [ ] Create a web server that runs in its own thread (or Go routine) and serves up audio files from the specified directory
- [ ] 