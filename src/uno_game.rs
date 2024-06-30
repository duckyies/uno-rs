use std::collections::HashMap;
use crate::card::Card;
use crate::player::Player;
use crate::rules::Rule;
use rand::thread_rng;
use rand::seq::SliceRandom;

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
    time_started: i64,
    rules: [Rule; 10],
}

impl UnoGame {
    pub fn new() -> UnoGame {
        UnoGame {
            players: HashMap::new(),
            queue: Vec::new(),
            deck: Vec::new(),
            called_out: false,
            finished: Vec::new(),
            discard: Vec::new(),
            dropped: Vec::new(),
            started: false,
            confirm: false,
            time_started: 0,
            rules: UnoGame::generate_rules()
        }
    }
    pub fn start(&mut self) {
        self.generate_deck();
    }
    pub fn generate_deck(&mut self) {
        let decks = self.get_rule("decks");
        if let Some(deck_no) = decks {
            for deck in 0..deck_no.value {
                for color in ["R","G","B","Y"] {
                    for card in 0..10 {
                        self.deck.push(Card::new(card.to_string(), color));
                        self.deck.push(Card::new(card.to_string(), color));
                    }
                    for card in 0..2 {
                        self.deck.push(Card::new("+2".to_string(), color));
                        self.deck.push(Card::new("SKIP".to_string(), color));
                        self.deck.push(Card::new("REVERSE".to_string(), color));
                    }
                }
                for card in 0..4 {
                    self.deck.push(Card::new("WILD".to_string(), ""));
                    self.deck.push(Card::new("WILD+4".to_string(), ""));
                }
            }

        }
        else {
            panic!("Rule 'decks' not found");
        }
        self.shuffle_deck();
    }

    fn shuffle_deck(&mut self) {
        self.deck.shuffle(&mut thread_rng())
    }

    fn get_rule(&mut self , get_rule: &str) -> Option<&Rule>{
        self.rules.iter().find(|rule| rule.name.to_lowercase() == get_rule.to_lowercase())
    }

    fn generate_rules() -> [Rule; 10] {
        [
            Rule{
                desc: "The number of decks to use.".to_string(),
                value: 1,
                name: "Decks".to_string(),
                rtype: "integer".to_string(),
                max: 8,
                min: 1,
            },
            Rule{
                desc: "How many cards to pick up at the beginning.".to_string(),
                value: 7,
                name: "Initial Cards".to_string(),
                rtype: "integer".to_string(),
                min: 1,
                max: 5000,
            },
            Rule {
                desc: "Whether pickup cards (+2, +4) should also skip the next person's turn.".to_string(),
                value: 1,
                name: "Draws Skip".to_string(),
                rtype: "boolean".to_string(),
                min: 0,
                max: 0,
            },
            Rule {
                desc: "Whether reverse cards skip turns when there's only two players left.".to_string(),
                value: 1,
                name: "Reverses Skip".to_string(),
                rtype: "boolean".to_string(),
                min: 0,
                max: 0,
            },
            Rule {
                desc: "Whether someone must play a card if they are able to.".to_string(),
                value: 0,
                name: "Must Play".to_string(),
                rtype: "boolean".to_string(),
                min: 0,
                max: 0,
            },
            Rule {
                desc: "Gives the ability to call someone out for not saying uno!".to_string(),
                value: 1,
                name: "Callouts".to_string(),
                rtype: "boolean".to_string(),
                min: 0,
                max: 0,
            },
            Rule {
                desc: "The number of cards to give someone when called out.".to_string(),
                value: 2,
                name: "Callout Penalty".to_string(),
                rtype: "integer".to_string(),
                max: 1000,
                min: 0,
            },
            Rule {
                desc: "The number of cards to give someone for falsely calling someone out.".to_string(),
                value: 2,
                name: "False Callout Penalty".to_string(),
                rtype: "integer".to_string(),
                max: 1000,
                min: 0,
            },
            Rule {
                desc: "Automatically plays a card after drawing, if possible. If a wild card is drawn, will give a prompt for color.".to_string(),
                value: 0,
                name: "Automatically Play After Draw".to_string(),
                rtype: "boolean".to_string(),
                min: 0,
                max: 0,
            },
            Rule {
                desc: "Automatically proceeds to the next turn after drawing, meaning that you cannot play drawn cards (without DRAW_AUTOPLAY).".to_string(),
                value: 1,
                name: "Automatically Pass Turns (WIP)".to_string(),
                rtype: "boolean".to_string(),
                min: 0,
                max: 0,
            }
        ]
    }

}