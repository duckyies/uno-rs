use std::collections::HashMap;
use std::ops::Index;
use crate::card::Card;
use crate::uno_game::UnoGame;

pub struct Player {
    username: String,
    uno_game: UnoGame,
    hand: Vec<Card>,
    called: bool,
    finished: bool,
    cards_played: i32,
    parse_color: ()
}

struct PlayerOutput {
    cards_played: i32,
    name: String,
}

impl Player {
    fn cards_changed(&mut self) {
        self.sort_hand();
    }
    fn sort_hand(&mut self) {
        self.hand.sort()
    }
    pub fn parse_color<'a>(&self, color: String) -> &'a str {
        match color.as_str() {
            "red" | "r" | "R" => "R",
            "green" | "g" | "G" => "G",
            "blue" | "b" | "B" => "B",
            "yellow" | "y" | "Y" => "Y",
            _ => color.as_str()
        }
    }
    fn format_output(&self) -> PlayerOutput {
        PlayerOutput {
            cards_played: self.cards_played,
            name: self.username.clone(),
        }
    }
    fn get_card(&self, mut words: &Vec<String>) -> Option<Card> {
        let mut color: String = String::new();
        let mut id: String = String::new();
        if words.len() == 1 {
            let str_color: String = words[0].clone().chars()[0];
            let parsed = self.parse_color(str_color.clone());
            if  parsed == "" {
                id = words[0].clone();
            }
            else {
                color = String::from(parsed);
                id = str_color;
            }
        } else {
            color = words[0].clone();
            id = words[1].clone();
        }
        if id.as_str() == "" {
            None
        }
        let wild: [&str; 2] = ["WILD","WILD+4"];
        let wild_aliases = HashMap::from([
            ("W", "WILD"),
            ("W+4", "WILD+4"),
            ("REV", "REVERSE"),
            ("R", "REVERSE"),
            ("NOU", "REVERSE"),
            ("S", "SKIP"),
            ("FUCKU", "SKIP"),
            ("WSH", "WILDSWAPHANDS"),
            ("WSWAP", "WILDSWAPHANDS"),
            ("WSWAPHAND", "WILDSWAPHANDS"),
            ("LOL", "WILDSWAPHANDS"),
        ]);
        let new_color = self.parse_color(color);
        if new_color {
            if color == "" && (wild.contains(&&*id.to_uppercase()) || wild_aliases.contains_key(&id.to_uppercase())) {
                None
            }

            else {
                color = self.parse_color(id).parse().unwrap();
                if color == ""
            }
        }
    }
}