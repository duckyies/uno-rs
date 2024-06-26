use std::collections::HashMap;
use crate::card::Card;
use crate::player::Player;

pub struct UnoGame {
    players : HashMap<i32,Player>,
    queue: Vec<Player>,
    deck: Vec<Card>,
    called_out: bool,
    discard: Vec<Card>,
    finished: Vec<Player>,
    dropped: Vec<Player>,
    started: bool,
    confirm: bool,
    timeStarted: i64,
    rules

}