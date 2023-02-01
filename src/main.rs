#![feature(slice_group_by)]

use std::io::{stdout, Write};
use rand::thread_rng;
use crate::game::GameState;
use crate::player::{AIDifficulty, Player};

mod card;
mod player;
mod game;

fn main() {

    let difficulty = get_difficulty();

    enable_ansi_support::enable_ansi_support().unwrap();

    println!("Welcome to Uno (CLI Edition!). The game will start shortly.");

    std::thread::sleep(std::time::Duration::from_millis(1500));

    let mut rng = &mut thread_rng();
    let mut rng2 = &mut thread_rng();
    let mut rng3 = &mut thread_rng();

    let ai_one = &mut player::Ai::new(&mut rng, AIDifficulty::Easy);
    let ai_two = &mut player::Ai::new(&mut rng2, AIDifficulty::Medium);
    let ai_three = &mut player::Ai::new(&mut rng3, AIDifficulty::Hard);

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

fn get_difficulty() -> AIDifficulty {
    if std::env::args().len() > 1 {
        let arg = std::env::args().nth(1).unwrap();

        return match arg.to_lowercase().as_str() {
            "e" | "easy" => AIDifficulty::Easy,
            "m" | "medium" => AIDifficulty::Medium,
            "h" | "hard" => AIDifficulty::Hard,
            _ => {
                println!("Invalid difficulty. Defaulting to Medium.");
                AIDifficulty::Medium
            }
        }
    }

    let mut input = String::new();

    loop {
        println!("Choose a difficulty: [E]asy, [M]edium, [H]ard");

        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "e" | "easy" => return AIDifficulty::Easy,
            "m" | "medium" => return AIDifficulty::Medium,
            "h" | "hard" => return AIDifficulty::Hard,
            _ => {
                println!("Invalid input. Please try again.");
                input.clear();
            }
        }
    }
}
