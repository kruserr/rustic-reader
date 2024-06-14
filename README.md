<p align="center">
  <a href="https://github.com/kruserr/rustic-reader" target="_blank">
    <img width="300" src="https://raw.githubusercontent.com/kruserr/rustic-reader/main/assets/logo/logo.svg">
  </a>
</p>

# RusticReader
A minimalistic ebook reader

## Features
- CLI client
- Each component in the CLI client is exposed as a unix style utility
- Converts PDF to plain text
- Justifies the plain text to specified column width
- Centers the text based on the width of the terminal
- Minimalistic less like interactive reader with vim like bindings
- Written in pure rust
- Statically linked single binary executable
- Cross platform

## Getting Started
### Cargo
Install and run with cargo
```sh
cargo install --locked rustic-reader
rustic-reader document.pdf
```

### Git and cargo
Clone the repo, build from source and run
```sh
git clone https://github.com/kruserr/rustic-reader.git
cd rustic-reader/rustic-reader
cargo install --locked --path .
rustic-reader document.pdf
```

## Roadmap
- [ ] Integrated command line
- [ ] Auto saving progress
- [ ] Text highlighting
- [ ] EPUB format support
- [ ] Offline PWA web client
- [ ] Server to sync books, progress and highlights
- [ ] Support more ebook and document formats
- [ ] CLI client image to ascii art converter
- [ ] Natural sounding ai voice model for text to speech narration
