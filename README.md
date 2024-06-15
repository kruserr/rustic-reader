<p align="center">
  <a href="https://github.com/kruserr/rustic-reader" target="_blank">
    <img width="300" src="https://raw.githubusercontent.com/kruserr/rustic-reader/main/assets/logo/logo.svg">
  </a>
</p>

# RusticReader
A minimalistic ebook reader

## Features
- CLI client
- Each component in the CLI client is exposed as a UNIX style utility
- Converts PDF to plain text
- Justifies the plain text to specified column width
- Centers the text based on the width of the terminal
- Minimalistic less like interactive reader with vim like bindings
- Written in pure Rust
- Cross platform
- Statically linked single binary executable

## Getting Started
### Install the Rust toolchain
For UNIX type operating systems run the following command:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If you are running some other type of operating system, checkout the Rust docs for install instructions: https://www.rust-lang.org/learn/get-started

### Install and run with cargo
```sh
cargo install --locked rustic-reader
rustic-reader document.pdf
```

If the `rustic-reader` binary executable is not found, checkout how to add `~/.cargo/bin` to your path.

e.g. for the fish shell you add the following to your config:

~/.config/fish/config.fish
```fish
fish_add_path ~/.cargo/bin
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

## Reference
The pdf to text converter should produce similar results as lesspipe with pdftotext into less.
```sh
lesspipe document.pdf | less
```

We are not looking to replace these tools, but rather build a cross platform standalone statically linked single binary executable, that has a minimal set of features, that make ebook reading enjoyable on a desktop computer.
Furthermore we are building a seamless experience of reading ebooks, both on a desktop computer and a tablet or ereader with a browser.
