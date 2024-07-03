use std::thread::sleep;
use uno;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
fn main(){
    let mut game = uno::uno_game::UnoGame::new();
    game.add_player("test");
    game.add_player("new test");
    game.start();
    println!("{}",game.scoreboard());
    sleep(Duration::new(65, 0));
    println!("{}",game.scoreboard());
    println!("TEST")
}