# Retris
This is a simple implementation of the classic Tetris game in Rust using [Cursive](https://github.com/gyscos/cursive) library.

# Features
* Play Tetris using the keyboard controls
* Leftmost and rightmost movement using the `a` and `d` keys respectively
* Counter-clockwise rotation using the `w` key
* Clockwise rotation using the `↑` or `e` keys
* Speed up the block using the `↓` key
* Hard drop the block using the ` ` key
* Stop and resume the game using the `m` key
* Start a new game using the `n` key

# Installation
To install and run the game, you'll need to have Rust and Cargo installed on your system. Once you have Rust and Cargo installed, you can clone the repository and run the game using the following commands:

```
git clone https://github.com/your-username/tetris.git
cd tetris
cargo run
```
Or you can download the crate from crates.io with following command:
```
cargo install retris
```

# How to Play
The goal of the game is to clear as many lines as possible by fitting the falling blocks together. Use the keyboard controls to move and rotate the blocks as they fall. The game ends when the blocks reach the top of the screen.

# Controls
* a: Move the block to the leftmost position
* d: Move the block to the rightmost position
* w: Rotate the block counter-clockwise
* ↑ or e: Rotate the block clockwise
* ↓: Speed up the block
* space: Hard drop the block
* m: Stop and resume the game
* n: Start a new game

# Acknowledgements
This project was inspired by the classic Tetris game and Cursive library for Rust.

# License
This project is licensed under the MIT License. See the LICENSE file for details.
