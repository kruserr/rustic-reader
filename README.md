<p align="center">
  <a href="https://github.com/kruserr/rustic-reader" target="_blank">
    <img width="300" src="https://raw.githubusercontent.com/kruserr/rustic-reader/main/assets/logo/logo.svg">
  </a>
</p>

# RusticReader
A minimalistic ebook reader

## Overview
The goal of this project is to build an ebook reader that has a minimal set of features, that make ebook reading enjoyable on a desktop computer.
Furthermore we are building a seamless experience for reading ebooks, both on a desktop computer and a tablet or ereader with a browser.

## Features
- CLI client
  - Converts PDF to plain text
  - Justifies the plain text to specified column width
  - Horizontally centers the text
  - Minimalistic less like interactive reader with vim like bindings
  - Written in pure Rust
  - Cross platform
  - Statically linked single binary executable
  - Each component in the CLI client is exposed as a UNIX style utility

## Quick start guide
### Install the CLI client
```sh
cargo install --locked rustic-reader
rustic-reader document.pdf
```

For further install instructions read the [Getting started page](docs/pages/getting-started.md)

## Documentation
Visit the [Documentation](docs/README.md)

## Roadmap
- [ ] EPUB format support
- [ ] Auto saving progress
- [ ] Offline PWA web client
- [ ] Server to sync progress
- [ ] Integrated command line
- [ ] Text highlighting
- [ ] Extend server to sync books and highlights
- [ ] Support more ebook and document formats
- [ ] CLI client image to ascii art converter
- [ ] Natural sounding ai voice model for text to speech narration
