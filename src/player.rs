use std::io::stdin;
use crate::card::{Card, CardColor};
use rand::{Rng, RngCore};
use crate::game::{Turn, TurnResult};

pub struct Human {
    name: String,
}

pub struct Ai<R: RngCore> {
    ran: R,
    name: String,
}

pub trait Player {
    fn name(&self) -> &str;
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

        let index = loop {
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

                    break index;
                },
                Err(_) => {
                    println!("Invalid input. Please try again.");
                    continue;
                }
            }
        };

        let card = turn.hand[index];

        return match card {
            Card::Wild { .. } => {
                let color = self.get_color();

                if color.is_some() { Some(TurnResult::Played(Card::Wild { color })) } else { None }
            },
            Card::DrawFour { .. } => {
                let color = self.get_color();

                if color.is_some() { Some(TurnResult::Played(Card::DrawFour { color })) } else { None }
            },
            _ => {
                Some(TurnResult::Played(card))
            }
        }
    }

    fn get_color(&self) -> Option<CardColor> {
        return loop {
            let mut input = String::new();

            println!("Enter a color to choose, or type 'back' to go back to the decision screen.");

            stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            if input == "back" {
                break None;
            }

            match input.as_str() {
                "red" => break Some(CardColor::Red),
                "blue" => break Some(CardColor::Blue),
                "green" => break Some(CardColor::Green),
                "yellow" => break Some(CardColor::Yellow),
                _ => {
                    println!("Invalid input. Please try again.");
                    continue;
                }
            }
        };
    }
}

impl Player for Human {
    fn name(&self) -> &str {
        &self.name
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
        println!("{} played a {}.", other.name(), card);
    }

    fn observe_turn_skip(&self, observed_cards: Option<Vec<&Card>>) {
        if let Some(observed_cards) = observed_cards {
            if observed_cards.len() == 1 {
                println!("You drew a {}.", observed_cards[0]);
            }
            else {
                println!("You drew {} cards. ({:?})", observed_cards.len(), observed_cards);
            }
        }
        else {
            println!("You have been skipped!");
        }
    }
}

impl HumanPlayer for Human {}

impl<R> Ai<R> where R: RngCore {

    pub fn new(mut ran: R) -> Ai<R> {
        let name = AI_NAMES[ran.gen_range(0..AI_NAMES.len())].to_string();

        Ai {
            ran,
            name,
        }
    }
}

impl<R> Player for Ai<R> where R : RngCore {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_turn(&mut self, turn: &Turn) -> TurnResult {
        let index = self.ran.gen_range(0..turn.hand.len());

        unimplemented!("AI not implemented yet")
    }

    fn observe_turn(&self, other: &dyn Player, card: &Card) {
        // Nothing to do here.
    }

    fn observe_turn_skip(&self, observed_cards: Option<Vec<&Card>>) {
       // Nothing to do; the game loop handles insertion
    }
}
