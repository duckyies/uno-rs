use std::cmp::min;
use std::collections::HashMap;
use std::fmt::format;
use std::time::SystemTime;
use crate::card::Card;
use crate::player::Player;
use crate::rules::Rule;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::time::{UNIX_EPOCH};

pub struct UnoGame {
    players : HashMap<i32,Player>,
    queue: Vec<Player>,
    deck: Vec<Card>,
    called_out: bool,
    discard: Vec<Card>,
    finished: Vec<Player>,
    dropped: Vec<Player>,
    started: bool,
    drawn: i32,
    confirm: bool,
    card_num: i32,
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
            drawn: 0,
            card_num: 1,
            started: false,
            confirm: false,
            time_started: 0,
            rules: UnoGame::generate_rules()
        }
    }
    pub fn start(&mut self) {
        if self.players.len() < 2 {
            panic!("Need atleast two players to start!")
        }
        self.generate_deck();
        for (_, player) in &self.players {
            self.queue.push(player.clone())
        }
        self.time_started = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()/60) as i64;
        self.discard.push(self.deck.pop().unwrap());
        self.started = true;
        let start_card_no = self.get_rule("Initial Cards").unwrap().value;
        if start_card_no * self.players.len() as i32 > self.deck.len() as i32 {
            panic!("Did not find enough cards to start playing")
        }
        for id in 0..self.players.len() as i32{
            self.deal(id, start_card_no);
        }
    }

    fn deal(&mut self, player_id: i32, number: i32) -> i32 {
        if self.deck.len() < number as usize {
            if self.discard.len() == 0 {
                panic!("Not enough cards found to play");
            }
            self.generate_deck();
            self.discard = Vec::from([self.discard[0].clone()]);
        }
        if let Some(player) = self.players.get_mut(&player_id) {
            let card = self.deck[0].num;
            for _ in 0..number {
                player.hand.push(self.deck[0].clone());
                self.deck.remove(0);
                self.drawn += 1;
            }
            player.sort_hand();
            player.called = false;
            return card
        }
        else {
            panic!("Player with id {} not found", player_id)
        }
    }

    pub fn scoreboard(&self) -> String {
        let mut out = String::new();
        let mut rank = 1;
        for person in &self.finished {
            out.push_str(format!("{}. *{}*", rank, person.username).as_str());
            rank+=1;
        }
        let mins = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()/60) as i64 - self.time_started;
        out.push_str(format!("\nThis game lasted {} minutes and {} cards were drawn",mins,self.drawn).as_str());
        out
    }

    fn generate_deck(&mut self) {
        let decks = self.get_rule("decks");
        if let Some(deck_no) = decks {
            for _ in 0..deck_no.value {
                for color in ["R","G","B","Y"] {
                    for card in 0..10 {
                        self.deck.push(Card::new(card.to_string(), color, self.card_num));
                        self.card_num+=1;
                        self.deck.push(Card::new(card.to_string(), color, self.card_num));
                        self.card_num+=1;
                    }
                    for _ in 0..2 {
                        self.deck.push(Card::new("+2".to_string(), color, self.card_num));
                        self.card_num+=1;
                        self.deck.push(Card::new("SKIP".to_string(), color, self.card_num));
                        self.card_num+=1;
                        self.deck.push(Card::new("REVERSE".to_string(), color, self.card_num));
                        self.card_num+=1;
                    }
                }
                for _ in 0..4 {
                    self.deck.push(Card::new("WILD".to_string(), "", self.card_num));
                    self.card_num+=1;
                    self.deck.push(Card::new("WILD+4".to_string(), "", self.card_num));
                    self.card_num+=1;
                }
            }

        }
        else {
            panic!("Rule 'decks' not found");
        }
        self.shuffle_deck();
    }

    pub fn add_player(&mut self, name: &str) -> &Player {
        let player = Player {
            id: self.players.len() as i32,
            username: name.to_string(),
            hand: vec![],
            called: false,
            finished: false,
            cards_played: 0,
            messages: vec![],
        };
        let id = player.id;
        self.players.insert(player.id, player);
        return &self.players[&id]
    }
    fn shuffle_deck(&mut self) {
        self.deck.shuffle(&mut thread_rng())
    }

    pub fn remove_player(&mut self, player_id: i32) {
        let player = self.players.get_mut(&player_id);
        if let Some(player) = player {
            self.dropped.push(player.clone());
            if self.queue[0].id == player_id {
                self.next();
            };
            self.players.retain(|f_player, _| *f_player != player_id);
            self.queue.retain(|f_player| f_player.id != player_id);
        }
        else {
            panic!("Player with id {} not found",player_id)
        }
    }
    
    pub fn set_rule(&mut self, rule: &str, value: i32) -> String {
        let found_rule = self.get_rule(rule);
        if let Some(rule) = found_rule {
            if rule.rtype == "integer" {
                if value > rule.max || value < rule.min {
                    return format!("Value {} is out of bounds for this rule", value)
                }
                self.rules[rule.idx as usize].value = value;
            }
            else if rule.rtype == "boolean" {
                if value != 1 && value != 0 {
                    return format!("Value for rule {} must be 0 or 1", rule.name)
                }
                self.rules[rule.idx as usize].value = value;
            }
            return "Rule changed".to_string()
        }
        else {
            panic!("Rule {} not found", rule)
        }
    }

    pub fn show_rule(&mut self, rule: &str) -> String {
        let found_rule = self.get_rule(rule);
        if let Some(rule) = found_rule {
            format!("*{}*\nType: {}\nValue: {}\n\n{}", rule.name,rule.rtype,rule.value,rule.desc)
        }
        else {
            return String::from("")
        }
    }
    
    pub fn show_all_rules(&mut self) -> String {
        let mut rules = String::new();
        for rule in UnoGame::generate_rules() {
            rules.push_str(&format!("*{}*\nType: {}\nValue: {}\n{}\n\n", rule.name,rule.rtype,rule.value,rule.desc))
        }
        rules
    }

    pub fn get_rule(&mut self , get_rule: &str) -> Option<&Rule>{
        self.rules.iter().find(|rule| rule.name.to_lowercase() == get_rule.to_lowercase())
    }

    pub fn get_curr_player(&mut self) -> &Player {
        &self.queue[0]
    }

    pub fn get_curr_card(&mut self) -> &Card {
        &self.discard.last().unwrap()
    }

    fn next(&mut self) -> &Player {
        if self.queue.is_empty() {
            panic!("Game has ended!")   
        }
        let player = self.queue[0].clone();
        self.queue.remove(0usize);
        self.queue.push(player);
        self.queue.retain(|player| !player.finished);
        &self.queue[0]
    }

    pub fn notify_player(&mut self, id: i32, msg: &str) -> String{
        let fplayer = self.players.get_mut(&id);
        if let Some(player) = fplayer {
            player.messages.push(msg.to_string());
            msg.to_string()
        }
        else {
            panic!("Player {} not found",id)
        }
    }



    fn generate_rules() -> [Rule; 10] {
        [
            Rule{
                idx: 0,
                desc: "The number of decks to use.".to_string(),
                value: 1,
                name: "Decks".to_string(),
                rtype: "integer".to_string(),
                max: 8,
                min: 1,
            },
            Rule{
                idx: 1,
                desc: "How many cards to pick up at the beginning.".to_string(),
                value: 7,
                name: "Initial Cards".to_string(),
                rtype: "integer".to_string(),
                min: 1,
                max: 5000,
            },
            Rule {
                idx: 2,
                desc: "Whether pickup cards (+2, +4) should also skip the next person's turn.".to_string(),
                value: 1,
                name: "Draws Skip".to_string(),
                rtype: "boolean".to_string(),
                min: 0,
                max: 0,
            },
            Rule {
                idx: 3,
                desc: "Whether reverse cards skip turns when there's only two players left.".to_string(),
                value: 1,
                name: "Reverses Skip".to_string(),
                rtype: "boolean".to_string(),
                min: 0,
                max: 0,
            },
            Rule {
                idx: 4,
                desc: "Whether someone must play a card if they are able to.".to_string(),
                value: 0,
                name: "Must Play".to_string(),
                rtype: "boolean".to_string(),
                min: 0,
                max: 0,
            },
            Rule {
                idx: 5,
                desc: "Gives the ability to call someone out for not saying uno!".to_string(),
                value: 1,
                name: "Callouts".to_string(),
                rtype: "boolean".to_string(),
                min: 0,
                max: 0,
            },
            Rule {
                idx: 6,
                desc: "The number of cards to give someone when called out.".to_string(),
                value: 2,
                name: "Callout Penalty".to_string(),
                rtype: "integer".to_string(),
                max: 1000,
                min: 0,
            },
            Rule {
                idx: 7,
                desc: "The number of cards to give someone for falsely calling someone out.".to_string(),
                value: 2,
                name: "False Callout Penalty".to_string(),
                rtype: "integer".to_string(),
                max: 1000,
                min: 0,
            },
            Rule {
                idx: 8,
                desc: "Automatically plays a card after drawing, if possible. If a wild card is drawn, will give a prompt for color.".to_string(),
                value: 0,
                name: "Automatically Play After Draw".to_string(),
                rtype: "boolean".to_string(),
                min: 0,
                max: 0,
            },
            Rule {
                idx: 9,
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

//Commands
impl UnoGame {
    pub fn play(&mut self, card: String) -> Result<String,String> {
        if self.queue.is_empty() {
            Err("Game has ended!".to_string())
        }
        else {
            let rev_skip = self.get_rule("Reverses Skip").unwrap().value;
            let draw_skip = self.get_rule("Draws Skip").unwrap().value;
            let player = &mut self.queue[0];

            let found_card = player.get_card(&(card.split_whitespace().collect()));

            if let Some(found_card) = found_card {

                let card = player.hand.iter().find(|cards| cards.num == found_card).unwrap().clone();
                let curr_card = self.discard.last().unwrap();

                if curr_card.wild || curr_card.color.is_empty() || curr_card.id == card.id || curr_card.color == card.color {
                    self.called_out = false;
                    self.discard.push(card.clone());
                    player.hand.retain(|c_num| c_num.num != found_card);
                    player.sort_hand();

                    let mut prefix = String::new();
                    let mut extra = String::new();

                    if player.hand.len() == 0 {
                        player.finished = true;
                        self.finished.push(player.clone());
                        prefix.push_str(format!("{} has no more cards. They finished in rank *{}*!\n\n", player.username, self.finished.len()).as_str());
                        
                        if self.queue.len() == 2 {
                            prefix.push_str(self.scoreboard().as_str());
                            self.finished.push(self.queue[1].clone());
                            self.queue = Vec::new();
                            return Ok(prefix)
                        }
                    }
                    
                    match card.id.as_str() {
                        "REVERSE" => {
                            if self.queue.len() > 2 {
                                self.queue.reverse();
                                let ins = self.queue.pop().unwrap();
                                self.queue.insert(0, ins);
                                extra.push_str("Turns are now in reverse order!");
                            }
                            else if rev_skip == 1 {
                                self.queue.reverse();
                                extra.push_str(format!("{}, skip a turn!", self.queue[0].username.clone()).as_str());
                            };
                        }
                        "SKIP" => {
                            let ins = self.queue.remove(0);
                            self.queue.push(ins);
                            extra.push_str(format!("{}, skip a turn!", self.queue[0].username.clone()).as_str());
                        }
                        "+2" => {
                            let mut amount = 0;
                            for i in (self.discard.len() - 1) ..=0 {
                                if self.discard[i].id == "+2" {
                                    amount += 2;
                                }
                                else {
                                    break;
                                }
                            }
                            self.deal(self.queue[1].id, amount);
                            extra.push_str(format!("{} picks up {}!",self.queue[1].username.clone(), amount ).as_str());
                            if draw_skip == 1 {
                                extra.push_str("Also, skip a turn!");
                                let ins = self.queue.remove(0);
                                self.queue.push(ins);
                            }
                        }
                        "WILD" => {
                            extra.push_str(format!("The color is now {}", card.color).as_str());
                        }
                        "WILD+4" => {
                            self.deal(self.queue[1].id, 4);
                            extra.push_str(format!("{} picks up! The current color is now {}", self.queue[1].username.clone(), card.color).as_str());
                            if draw_skip == 1 {
                                extra.push_str("Also, skip a turn!");
                                let ins = self.queue.remove(0);
                                self.queue.push(ins);
                            }
                        }
                        _ => { 
                            
                        }
                    };
                    self.next();
                    Ok(prefix)
                }
                else {
                    Err(format!("You cannot play this card here. Last played card was {} {}",curr_card.id, curr_card.color))
                }
            }
            else {
                Err(format!("Card {} not found in hand, its currently {}'s turn", card, player.username))
            }
        }
    }
    
    pub fn draw(&mut self) -> Result<String,String> {
        let must_play = self.get_rule("Must Play").unwrap().value;
        let draw_autoplay = self.get_rule("Automatically Play After Draw").unwrap().value;

        let player = &mut self.queue[0];
        if must_play == 1 {
            for card in &player.hand {
                let curr_card = self.discard.last().unwrap();
                if curr_card.wild || curr_card.color.is_empty() || curr_card.id == card.id || curr_card.color == card.color {
                    return Err("You must play a card if able.".to_string())
                }
            }
        }
        let card_num = self.deal(self.queue[0].id, 1);
        if draw_autoplay == 1 {
            let card = self.queue[0].hand.iter().find(|cards| cards.num == card_num).unwrap().clone();
            let curr_card = self.discard.last().unwrap();
            if curr_card.wild || curr_card.color.is_empty() || curr_card.id == card.id || curr_card.color == card.color {
                self.play(format!("{} {}", card.color, card.id)).expect("");
            }
        }
        self.next();
        Ok(format!("{}", card_num))
    }

    pub fn callout(&mut self, call_player_id: i32) -> Result<String,String> {
        let callouts = self.get_rule("Callouts").unwrap().value;
        if callouts == 0 {
           return Err("Callouts are not permitted in this game".to_string())
        }
        
        if self.called_out {
            return Err("A callout was already performed in this turn!".to_string());
        }
        
        let callout_penalty = self.get_rule("Callout Penalty").unwrap().value;
        let false_callout = self.get_rule("False Callout Penalty").unwrap().value;
        
        let mut called_out = false;
        let mut res = String::new();
        let mut calls: Vec<i32> = Vec::new();
        
        for player in &self.queue {
            if player.hand.len() == 1 && !player.called {
                calls.push(player.id);
                called_out = true;
                res.push_str(format!("{} you did not say UNO! Pick up {}",player.username.clone(),callout_penalty).as_str());
            }
        };
        
        for i in calls {
            self.deal(i,callout_penalty);
        }
        return if !called_out {
            self.deal(call_player_id, false_callout);
            self.called_out = true;
            Ok(format!("There was no one to call out! Pick up {}", callout_penalty))
        } else {
            self.called_out = true;
            Ok(res)
        }
        
    }
    
    pub fn uno(&mut self, call_player_id: i32) -> Result<String,String> {
        let player: &mut Player = self.queue.iter_mut().find(|ply| ply.id == call_player_id).unwrap();
        if player.hand.len() == 1 {
            return if player.called {
                Ok("You already said UNO!".to_string())
            } else {
                player.called = true;
                Ok("UNO!".to_string())
            }
        };
        Err("You have more than 1 card!".to_string())
    }
    
    pub fn table(&mut self) -> String {
        let last_card = self.discard.last().unwrap();
        let mut ext = format!("A {} {} has been played!\nIt is currently {}'s turn!\n\n", last_card.id.clone(), last_card.color.clone(), self.queue[0].username); 
        let mut idx = 1;
        for player in &self.queue {
            ext.push_str(format!("{}. {} - {} cards\n",idx,player.username, player.hand.len()).as_str());
            idx+=1
        }
        let mins = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()/60) as i64 - self.time_started;
        ext.push_str(format!("This game has lasted {} minutes and {} cards have been drawn", mins, self.drawn).as_str());
        ext
    }

}