# wordle-cli

built in 100% rust, uses the same answers as the official website

## how to build
it's really not that hard.
`$ cargo build --release`

## todo
- [ ] actually verify answers
  - [ ] hard mode
  - [ ] error check lmao
  - [ ] capitalization issues
- [ ] copy/paste
  - [ ] add emoji
  - [ ] copy directly to clipboard
- [ ] tui implementation
  - [ ] keep input line in one place
  - [ ] mimic actual wordle layout
  - [ ] minor animations
- [ ] scoreboard
  - [ ] local storage, preferably in `~/.local/state` for unix. windows tbd
  - [ ] streaks
  - [ ] maybe sync w/ website?
