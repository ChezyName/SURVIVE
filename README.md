<h1 align="center"> SURVIVE </h1>

A simple top-down 2D vampire-survivor-like Created in Rust with a simple challange of what if every single line of code, costs exactly one dollar? Well, this project costs around $2,654 - which by many comparisons is not a lot as some repos can hit a few million -- like the Linux Kernel which uses *31,505,798* Lines.

## Getting Started
Make sure you have Rust and Cargo, to start the project in the root dir just run `cargo run` or `cargo build` for  the build. Standard Rust and Cargo commands.

* `cargo run` to run the project
* `cargo build` to build the dev version
* `cargo build --release` for the final build

## Creating Custom Cards
This game like many vampire-survivior likes use power-ups/items to upgrade the player, the same holds true for this game. You can add a new card by using the following example.

1. Create a new file inside `src/items`, Example: a new card such as `Master.rs`
2. Then copy the basic card function which looks like this

Basic Card - No Functionality
``` rust
use super::{Item, ItemFactory};
use crate::player::Player;
use crate::config::format;

#[derive(Clone, Default)]
pub struct ITEM_NAME;

impl Item for ITEM_NAME {
    //returns the name of the card
    fn name(&self) -> String { "ITEM_NAME".to_string() }

    //returns the cost of the item
    fn cost(&self) -> usize { 999 }

    //returns the description or what this card does
    fn description(&self, _player: &mut Player) -> String {
        format!("ITEM_NAME Does ITEM_THING")
    }

    //when this card is bought, modify the player
    fn apply(&self, player: &mut Player) { }

    //if this is a one-time purchase item
    fn is_unique(&self) -> bool { false }
    
    //if this item can be bought currently
    //example: return player.crit_percent == 100.0
    fn can_buy(&self, _player: &mut Player) -> bool { return true; }

    fn clone_box(&self) -> Box<dyn Item> { Box::new(self.clone()) }
}

//Add the item to 
inventory::submit!(ItemFactory(|| Box::new(ITEM_NAME::default())));
```

3. **IMPORTANT:** Make sure to update the `mod.rs` in `src/items/mod.rs` to include your new file, or it will not be added to the build.

``` rust
//Items
// ........ ^ items above this line
pub mod wave_switch;

pub mod ITEM_NAME; //<- add your file here at the end of mod.rs
```

----
<p align="center">
© 2026 ChezyName
| <a href="https://youtube.com/@chezyname/">YouTube</a>
| <a href="https://twitter.com/@chezyname/">Twitter</a>
</p>
