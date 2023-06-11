# Glasswing (WIP)

Glasswing is a fast and flexible framework for building AI agents to play zero-sum games. It leverages Rust's advanced features to achieve high speed and offers extensive customization options. 

Simply configure your game, implement the necessary traits and get started. Glasswing takes care of the rest, or only part of it. That's up to you.

So far, Glasswing provides implementations for common agents such as *Minimax*, but you can define your own agents and abstraction layers as needed.

## Future goals 🎯

The dream is to be able to simply implement the necessary traits and have Glasswing's default agents maximise the outcome. The plan is to automatically support NNUE, provide an implementation of AlphaZero and automatically generate Endgame tables.

With the help of `tournament-rs` (WIP), swiss system tournaments as well as other tournament styles will be supported to help select the best agent. This will enable automatic testing.

# Progress

| Total      | ![](https://progress-bar.dev/30/) |
| ---------- | --------------------------------- |
| Core       | ![](https://progress-bar.dev/95/) |
| Structure  | ![](https://progress-bar.dev/65/) |
| Agent eval | ![](https://progress-bar.dev/20/) |
| Logging    | ![](https://progress-bar.dev/0/)  |
| Examples   | ![](https://progress-bar.dev/5/)  |



#### Checklist

- [x] Base traits
- [x] Contests
- [x] Game histories
- [x] Agents
- [ ] Errors, Results
  - [ ] Add str to errors
  - [x] Timeouts
  - [x] Errors in contest
  - [ ] Custom error support
  - [ ] Write errors to history
- [x] Timed Contests
- [x] Move Legality checks
- [ ] Documentation
  - [x] traits.rs
  - [ ] others
- [ ] Arenas and Tournaments
  - [ ] Agent evaluation
- [ ] Support for playing lines
- [ ] logging
- [ ] Split up code and set up workspace
- [ ] Neural Networks for evaluation
- [ ] Pondering
- [ ] Endgame tables
  - [ ] Generation
  - [ ] IO, read/write access