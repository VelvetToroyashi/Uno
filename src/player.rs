use std::intrinsics::likely;
use std::io::stdin;
use std::marker::PhantomData;
use crate::card::{Card, CardColor};
use rand::Rng;
use crate::game::{GameState, Turn, TurnResult};

pub struct Human {
    name: String,
    hand: Vec<Card>,
}

pub struct Ai {
    name: String,
    hand: Vec<Card>,
}

pub trait Player {
    fn name(&self) -> &str;
    fn hand(&mut self) -> &mut Vec<Card>;
    fn execute_turn(&mut self, turn: &Turn) -> TurnResult;
    fn observe_turn(&self, other: &dyn Player, card: &Card);
    fn observe_turn_skip(&self, observed_cards: Option<Vec<&Card>>);
}
pub trait AiPlayer: Player {}
pub trait HumanPlayer: Player {}

const AI_NAMES: [&str; 20] = [
    "Yukii", "Kurisu", "Mayuri", "Makise", "Misa", "Rin", "Miku", "Shinobu", "Shiro", "Rem",
    "Asuna", "Kirito", "Kazuto", "Shana", "Yoshino", "Yui", "Touka", "Rize", "Mikasa", "Levi",
];

impl<'h> Human {
    pub fn new(name: String) -> Human {
        Human {
            name,
            hand: vec![],
        }
    }

    fn get_action(&mut self, turn: &Turn) -> TurnResult {
        let mut input = String::new();

        loop {
            stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            match input {
                "Play" => {
                    let action = self.get_card(turn);

                    if let Some(play) = action {
                        return play;
                    }

                    continue;
                },
                "Draw" => {
                    let draw = if turn.to_draw > 0 { turn.to_draw } else { 1 };

                    return TurnResult::Drew(draw);
                },
                _ => {
                    println!("Invalid input. Please try again.");
                }
            }
        }
    }

    fn get_card(&mut self, turn: &Turn) -> Option<TurnResult> {
        let mut input = String::new();

        println!("You can play the following cards:");

        for (i, card) in turn.hand.iter().enumerate() {
            println!("{}: {}", i, card);
        }

        println!("Enter a number to select a card, or type 'back' to go back to the decision screen.");

        loop {
            stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            if input == "back" {
                return None;
            }

            // TODO: Clear?
            match input.parse::<usize>() {
                Ok(index) => {
                    if index >= turn.hand.len() {
                        println!("Invalid input. Please try again.");
                        continue;
                    }

                },
                Err(_) => {
                    println!("Invalid input. Please try again.");
                    continue;
                }
            }
        }
    }
}

impl Player for Human {
    fn name(&self) -> &str {
        &self.name
    }

    fn hand(&mut self) -> &mut Vec<Card> {
        &mut self.hand
    }

    fn execute_turn(&mut self, turn: &Turn) -> TurnResult {
        println!("Its your turn.");

        let can_play = turn.hand.len() > 0;

        if can_play {
            println!("What would you like to do? [Play], [Draw]")
        }
        else {
            println!("You have no cards to play. You must draw.");
        }

        if can_play {
            self.get_action(turn)
        }
        else {
            TurnResult::Drew(1)
        }
    }

    fn observe_turn(&self, other: &dyn Player, card: &Card) {
        todo!()
    }

    fn observe_turn_skip(&self, observed_cards: Option<Vec<&Card>>) {
        todo!()
    }
}

impl HumanPlayer for Human {}

impl Ai {

    pub fn new() -> Ai {
        let mut rng = rand::thread_rng();
        let name = AI_NAMES[rng.gen_range(0..AI_NAMES.len())].to_string();

        Ai {
            name,
            hand: vec![],
        }
    }
}
