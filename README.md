# Introduction

A simple CLI tool, which when given a directory will search it for audio files and play them back.

This repo previously contained an implementation of the CLI tool in Rust but now contains the CLI and a web server in Go.
Search through the repo history to see the Rust implementation.

I'm thinking of re-implementing the [`betterjenkins`](https://www.github.com/teejas/betterjenkins) controller in Go, reasons being I want faster build times for the controller and it just makes more sense to build a Kubernetes-based project in Go rather than Rust.

As a way to familiarize myself with Go again, I'm going to build a web extension of the [`findnplay`](https://www.github.com/teejas/findnplay) CLI tool I previously built.

# Steps
## Go
- [x] Re-implement `findnplay` CLI tool in Go
- [x] Create a web server that ~~runs in its own thread (or Go routine) and~~ serves up audio files from the specified directory