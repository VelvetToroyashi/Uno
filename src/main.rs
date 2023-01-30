#![feature(slice_group_by)]

use rand::thread_rng;
use crate::game::GameState;
use crate::player::Player;

mod card;
mod player;
mod game;

fn main() {

    println!("Welcome to Uno (CLI Edition!). The game will start shortly.");

    std::thread::sleep(std::time::Duration::from_millis(1500));

    let ai_one = &mut player::Ai::new(thread_rng());
    let ai_two = &mut player::Ai::new(thread_rng());
    let ai_three = &mut player::Ai::new(thread_rng());

    println!("Lets start with your name: ");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();

    let human = &mut player::Human::new(name.trim().to_string());

    let players: Vec<&mut dyn Player> = vec![ai_one, human, ai_two, ai_three];

    let mut game = GameState::new(players);

    game.start();
}
