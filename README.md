<p align="center">
  <a href="https://github.com/kruserr/rustic-reader" target="_blank">
    <img width="300" src="https://raw.githubusercontent.com/kruserr/rustic-reader/main/assets/logo/logo.svg">
  </a>
</p>

# cli-pdf-reader
A minimalistic CLI PDF reader

## Features
- Converts PDF to plain text
- Justifies the plain text to specified column width
- Centers the text based on the width of the terminal
- Uses a minimalistic less like interactive reader
- Written in pure rust
- Statically linked single binary executable
- Cross platform

## Getting Started
### Cargo
Install and run with cargo
```sh
cargo install --locked cli-pdf-reader
cli-pdf-reader document.pdf
```

### Git and cargo
Clone the repo, build from source and run
```sh
git clone https://github.com/kruserr/rustic-reader.git
cd cli-pdf-reader
cargo install --locked --path .
cli-pdf-reader document.pdf
```
