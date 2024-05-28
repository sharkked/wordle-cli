# wordle-cli

built in 100% rust, uses the same answers as the official website

## how to build
it's really not that hard. u will need rust installed. if you don't, [it's really easy.](https://www.rust-lang.org/tools/install)
```sh
$ cargo build --release
```
### what i like to do after that
i have a folder at ~/.local/bin added to my computer's $PATH, and move my executables there. for  development, i've set up a symlink to this so i can just type `wordle` from anywhere in my shell n run my latest test build. 
```sh
$ ln -s /path/to/project/target/release/wordle ~/.local/bin/wordle
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
