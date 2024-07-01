
use uno;
fn main(){
    let mut game = uno::uno_game::UnoGame::new();
    game.add_player("test");
    game.start();
    println!("TEST")
}