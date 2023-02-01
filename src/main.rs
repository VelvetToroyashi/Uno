#![feature(slice_group_by)]

use std::io::{stdout, Write};
use rand::thread_rng;
use crate::game::GameState;
use crate::player::Player;

mod card;
mod player;
mod game;

fn main() {

    enable_ansi_support::enable_ansi_support().unwrap();

    println!("Welcome to Uno (CLI Edition!). The game will start shortly.");

    std::thread::sleep(std::time::Duration::from_millis(1500));

    let ai_one = &mut player::Ai::new(thread_rng());
    let ai_two = &mut player::Ai::new(thread_rng());
    let ai_three = &mut player::Ai::new(thread_rng());

    println!("Lets start with your name: ");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();

    let human = &mut player::Human::new(name.trim().to_string());

    loop {
        let players: Vec<&mut dyn Player> = vec![ai_one, human, ai_two, ai_three];
        let mut game = GameState::new(players);

        let winner  = game.start();

        if winner == human.name() {
            println!("You won!");
        } else {
            println!("{winner} won!");
        }

        std::thread::sleep(std::time::Duration::from_millis(1500));

        let mut input = String::new();

        println!("[P]lay again or [Q]uit?");

        std::io::stdin().read_line(&mut input).unwrap();

        if matches!(input.trim().to_lowercase().as_str(), "q" | "quit") {
            break;
        }

        print!("\x1B[2J\x1B[1;1H");
    }

}
