#![allow(dead_code, unused_imports)]

use card::{Card};
use game_types::{GameTypes};
use deck::{Deck};
use player::{Player};

pub struct Table<'a, 'b, 'c : 'b, 'd : 'c> {
    game: GameTypes,
    deck: &'a mut Vec<Card>,
    players:&'b mut [Option<&'c mut Player<'d, 'd>>]
}


impl<'a, 'b, 'c, 'd> Table<'a, 'b, 'c, 'd> {
    pub fn new(
        game: GameTypes, 
        deck: &'a mut Vec<Card>, 
        players: &'b mut[Option<&'c mut Player<'d, 'd>>], 
        _seats: usize) -> Table<'a, 'b, 'c, 'd> {
        Table{game, deck, players}
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn should_be_texas_holdem() {
        let mut deck = Deck::new();
        let player: Option<&mut Player> = None;
        let player_2: Option<&mut Player> = None;
        // TODO - Implement way to Copy (NONE) without deriving Copy
        // prolly with macros
        // this fails:
        // let mut players: [Option<&mut Player>; seats] = [player; seats];
        const SEATS: usize = 2;
        let mut players: [Option<&mut Player>; SEATS] = [player, player_2];
		let table = Table::new(GameTypes::TexasHoldem, &mut deck, &mut players, SEATS);
		assert_eq!(table.game, GameTypes::TexasHoldem);
	}
}
