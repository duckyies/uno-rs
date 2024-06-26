use std::collections::HashMap;
use std::ffi::c_void;
use std::ops::Index;
use crate::card::Card;
use crate::uno_game::UnoGame;

pub struct Player {
    id: i32,
    username: String,
    uno_game: UnoGame,
    hand: Vec<Card>,
    called: bool,
    finished: bool,
    cards_played: i32,
    messages : Vec<String>,
}

struct PlayerOutput {
    id: i32,
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
            id: self.id,
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
            };
        } else {
            color = words[0].clone();
            id = words[1].clone();
        };
        if id.is_empty() {
            return None
        };
        let wild: [&str; 2] = ["WILD","WILD+4"];
        let wild_aliases = HashMap::from([
            ("W", "WILD"),
            ("W+4", "WILD+4"),
            ("REV", "REVERSE"),
            ("R", "REVERSE"),
            ("NOU", "REVERSE"),
            ("S", "SKIP"),
            ("FUCKU", "SKIP"),
        ]);
        let new_color = self.parse_color(color);
        if new_color {
            if color.is_empty() && (wild.contains(&&*id.to_uppercase()) || wild_aliases.contains_key(&id.to_uppercase())) {
                return None
            }
            else {
                (id, color) = (color,id);
                if self.parse_color(color).is_empty() {
                    return None
                };
            }
        }
        if wild_aliases.contains_key(&id.to_uppercase()) {
            id = wild_aliases[id.to_uppercase()]
        };
        if ["WILD","WILD+4"].contains(&&*id.to_uppercase()) {
            let found_card = self.hand.iter().find(|&card: &Card| card.id.eq_ignore_ascii_case(id.as_str()));
            return *found_card
        }
        else {
            let found_card = self.hand.iter().find(|&card: &Card| card.id.eq_ignore_ascii_case(id.as_str()) && card.color.eq_ignore_ascii_case(color.as_str()));
            return *found_card
        }
    }

    pub fn send_message(&mut self, message: String) {
        self.messages.push(message);
    }

    pub fn get_hand(&mut self) -> String {
        self.sort_hand();
        format!(
            "Here is your hand:\n\n{}\n\nYou currently have {} card(s).",
            self.hand
                .iter()
                .map(|card| format!("**{}**", card))
                .collect::<Vec<String>>()
                .join(" | "),
            self.hand.len()
        )
    }

    pub fn hand(&mut self) -> &Vec<Card> {
        self.sort_hand();
        &self.hand
    }

    pub fn send_hand(&mut self) {
        self.send_message(self.get_hand())
    }

}