# lib-battleship
[![Build Status](https://travis-ci.org/Leopard2A5/lib-battleship.svg?branch=master)](https://travis-ci.org/Leopard2A5/lib-battleship)
[![codecov](https://codecov.io/gh/Leopard2A5/lib-battleship/branch/master/graph/badge.svg)](https://codecov.io/gh/Leopard2A5/lib-battleship)

A Rust library for battleship implementations. Find the documentation [here](https://docs.rs/lib_battleship/).

## How to use
You can set up the game by creating and configuring a `PreGame` object like so:
```rust
extern crate lib_battleship;

use lib_battleship::PreGame;
use lib_battleship::common::Player::*;
use lib_battleship::common::Orientation::*;
use lib_battleship::results::ShootOk;

// when hard-coding the game's dimensions, use `unwrap()`.
// PreGame's constructor makes sure that the battlefield
// is no smaller than 2x2.
let mut pregame = PreGame::new(10, 10).unwrap();

// add ship types
// `PreGame` validates that a ship is no shorter
// than 1 in length, thus the call to `unwrap()`.
let sub = pregame.add_ship_type("Submarine", 1).unwrap();
let corvette = pregame.add_ship_type("Corvette", 2).unwrap();
```

Then each player has to place all their ships on the battlefield. Each player has one ship of every ship type.

```rust
// pregame also validates the placement of each ship.
pregame.place_ship(P1, &corvette, 0, 0, Horizontal).unwrap();
pregame.place_ship(P1, &sub, 9, 9, Horizontal).unwrap();

pregame.place_ship(P2, &corvette, 5, 5, Vertical).unwrap();
pregame.place_ship(P2, &sub, 3, 7, Horizontal).unwrap();
```

Display a player's battlefield by iterating over its cells like so:

```rust
for y in 0..pregame.height() {
    for x in 0..pregame.width() {
        // PreGame::get_cell only returns Empty or Ship.
        let char = match pregame.get_cell(P1, x, y) {
            CellStatus::Empty => " ",
            CellStatus::Ship => "X",
            _ => unreachable!()
        };
        print!("{}", char);
    }
    println!("");
}
```

When all ships have been placed, start the game like so:

```rust
// pregame::start() checks that all ships have been
// placed and will complain if that's not the case.
let mut game = pregame.start().unwrap();
```

Use Game::get_cell and Game::get_opponent_cell to display the respective
battlefields, similar to PreGame::get_cell above. Use Game::get_cell to display
a battlefield to its owner and Game::get_opponent_cell to display their adversary's
battlefield with unhit cells appearing empty.

From now on, players can take turns shooting at each other's ships. A player can keep shooting as long as they score hits.

```rust
/// game validates that it's the shooting player's
/// turn and that they don't shoot out of bounds.
match game.shoot(P2, 0, 0).unwrap() {
  ShootOk::Hit => println!("hit!"),
  ShootOk::Miss => println!("miss!"),
  ShootOk::Destroyed => println!("ship destroyed!"),
  ShootOk::WinningShot => println!("you won!")
}
```
