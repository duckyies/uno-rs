
use uno;
fn main(){
    let mut game = uno::uno_game::UnoGame::new();
    game.start();
    println!("{}",game.show_rule("must play"));
    game.set_rule("must play",1);
    println!("{}",game.show_rule("must play"));
    println!("TEST")
}