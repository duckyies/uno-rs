use crate::card::Card;
use crate::uno_game::UnoGame;

pub struct Player {
    username: String,
    uno_game: UnoGame,
    hand: Vec<Card>,
    called: bool,
    finished: bool,
    cards_played: i32,
}