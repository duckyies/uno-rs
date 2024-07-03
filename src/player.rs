use std::collections::HashMap;
use crate::card::Card;
use crate::uno_game::UnoGame;

pub struct Player {
    pub(crate) id: i32,
    pub(crate) username: String,
    pub(crate) hand: Vec<Card>,
    pub(crate) called: bool,
    pub(crate) finished: bool,
    pub(crate) cards_played: i32,
    pub(crate) messages : Vec<String>,
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
    pub fn sort_hand(&mut self) {
        self.hand.sort()
    }
    pub fn parse_color(&self, color: String) -> &str {
        match color.as_str() {
            "red" | "r" | "R" => "R",
            "green" | "g" | "G" => "G",
            "blue" | "b" | "B" => "B",
            "yellow" | "y" | "Y" => "Y",
            _ => ""
        }
    }
    fn format_output(&self) -> PlayerOutput {
        PlayerOutput {
            id: self.id,
            cards_played: self.cards_played,
            name: self.username.clone(),
        }
    }
    pub fn get_card(&self, mut words: &Vec<&str>) -> Option<i32> {
        
        let mut color: String = String::new();
        let mut id: String = String::new();
        if words.len() == 1 {
            let str_color: String = words[0].chars().next().unwrap().to_string();
            let parsed = self.parse_color(str_color.clone());
            if  parsed == "" {
                id = words[0].to_string();
            }
            else {
                color = String::from(parsed);
                id = str_color;
            };
        } else {
            color = words[0].parse().unwrap();
            id = words[1].parse().unwrap();
        };
        if id.is_empty() {
            return None
        };
        let wild: [&str; 2] = ["WILD","WILD+4"];
        let aliases = ["W","W+4","REV","R","S","NOU","FUCKU"];
        let wild_aliases = HashMap::from([
            ("W".to_string(), "WILD"),
            ("W+4".to_string(), "WILD+4"),
            ("REV".to_string(), "REVERSE"),
            ("R".to_string(), "REVERSE"),
            ("NOU".to_string(), "REVERSE"),
            ("S".to_string(), "SKIP"),
            ("FUCKU".to_string(), "SKIP"),
        ]);
        let new_color = self.parse_color(color.clone());
        if new_color.is_empty() {
            if color.is_empty() && (wild.contains(&&*id.to_string().to_uppercase().to_string()) || aliases.contains(&&*id.to_uppercase())) {
                return None
            }
            else {
                (id, color) = (color,id);
                if self.parse_color(color.clone()).is_empty() {
                    return None
                };
            }
        }
        if aliases.contains(&&*id.to_uppercase()) {
            id = wild_aliases[&id.to_string().to_uppercase()].to_string()
        };
        return if ["WILD", "WILD+4"].contains(&&*id.to_uppercase().to_string()) {
            let found_card = self.hand.iter().find(|&card: &&Card| card.id.eq_ignore_ascii_case(id.as_str()));
            Some(found_card.unwrap().num)
        } else {
            let found_card = self.hand.iter().find(|&card: &&Card| card.id.eq_ignore_ascii_case(id.as_str()) && card.color.eq_ignore_ascii_case(color.as_str()));
            Some(found_card.unwrap().num)
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
        let hand = self.get_hand();
        self.send_message(hand)
    }
}

impl Clone for Player {
    fn clone(&self) -> Self {
        let player = Player {
            id: self.id,
            username: self.username.clone(),
            hand: Vec::clone(&self.hand),
            called: self.called,
            finished: self.finished,
            cards_played: self.cards_played,
            messages: Vec::clone(&self.messages),
        };
        return player
    }
}