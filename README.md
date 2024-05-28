# wordle-cli

built in 100% rust, uses the same answers as the official website

## how to install
it's really not that hard. u will need rust installed. if you don't, [it's really easy.](https://www.rust-lang.org/tools/install)
```sh
$ git clone https://github.com/sharkked/wordle-cli.git
$ cd wordle-cli
$ cargo install --path .
$ wordle
```

## todo
- [x] actually verify answers
  - [x] error check lmao
  - [x] capitalization issues
  - [x] valid words only
- [ ] hard mode
- [ ] copy/paste
  - [x] add emoji
  - [ ] copy directly to clipboard
- [ ] tui implementation
  - [ ] keep input line in one place
  - [ ] mimic actual wordle layout
  - [ ] minor animations
- [ ] scoreboard
  - [ ] local storage, preferably in `~/.local/state` for unix. windows tbd
  - [ ] streaks
  - [ ] maybe sync w/ website?
