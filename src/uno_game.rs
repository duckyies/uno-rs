use std::collections::HashMap;
use crate::card::Card;
use crate::player::Player;
use crate::rules::Rule;

pub struct UnoGame<'rule> {
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
    rules: [Rule<'rule>; 10],
}

impl UnoGame {
    fn new() -> UnoGame {
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
    fn generate_deck() {

    }
    fn generate_rules() -> [Rule; 10] {
        [
            Rule{
                desc: "The number of decks to use.",
                value: 1,
                name: "Decks",
                rtype: "integer",
                max: 8,
                min: 1,
            },
            Rule{
                desc: "How many cards to pick up at the beginning.",
                value: 7,
                name: "Initial Cards",
                rtype: "integer",
                min: 1,
                max: 5000,
            },
            Rule {
                desc: "Whether pickup cards (+2, +4) should also skip the next person's turn.",
                value: 1,
                name: "Draws Skip",
                rtype:"boolean",
                min: 0,
                max: 0,
            },
            Rule {
                desc: "Whether reverse cards skip turns when there's only two players left.",
                value: 1,
                name: "Reverses Skip",
                rtype: "boolean",
                min: 0,
                max: 0,
            },
            Rule {
                desc: "Whether someone must play a card if they are able to.",
                value: 0,
                name: "Must Play",
                rtype: "boolean",
                min: 0,
                max: 0,
            },
            Rule {
                desc: "Gives the ability to call someone out for not saying uno!",
                value: 1,
                name: "Callouts",
                rtype: "boolean",
                min: 0,
                max: 0,
            },
            Rule {
                desc: "The number of cards to give someone when called out.",
                value: 2,
                name: "Callout Penalty",
                rtype: "integer",
                max: 1000,
                min: 0,
            },
            Rule {
                desc: "The number of cards to give someone for falsely calling someone out.",
                value: 2,
                name: "False Callout Penalty",
                rtype: "integer",
                max: 1000,
                min: 0,
            },
            Rule {
                desc: "Automatically plays a card after drawing, if possible. If a wild card is drawn, will give a prompt for color.",
                value: 0,
                name: "Automatically Play After Draw",
                rtype: "boolean",
                min: 0,
                max: 0,
            },
            Rule {
                desc: "Automatically proceeds to the next turn after drawing, meaning that you cannot play drawn cards (without DRAW_AUTOPLAY).",
                value: 1,
                name: "Automatically Pass Turns (WIP)",
                rtype: "boolean",
                min: 0,
                max: 0,
            }
        ]
    }

}