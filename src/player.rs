use std::io::stdin;
use std::str::FromStr;
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

fn get_action(&mut self, turn: &Turn) -> Option<TurnResult> {
    let mut input = String::new();

    loop {
        stdin().read_line(&mut input).unwrap();
        let mut input = input.trim().to_lowercase();

        match input.as_str() {
            "play" => {
                break self.get_card(turn)
            },
            "draw" => {
                return Some(TurnResult::Drew);
            },
            _ => {
                input.clear();
                println!("Invalid input. Please try again.");
            }
        }
    }
}

    fn get_card(&mut self, turn: &Turn) -> Option<TurnResult> {
        let mut input = String::new();

        println!("You can play the following cards:");

        for (i, card) in turn.hand.iter().enumerate() {
            println!("{i}: {card}");
        }

        println!("Enter a number to select a card, or type 'back' to go back to the decision screen.");

        let index = loop {
            stdin().read_line(&mut input).unwrap();
            let mut input = input.trim().to_lowercase();

            if input == "back" {
                return None;
            }

            // TODO: Clear?
            match input.parse::<usize>() {
                Ok(index) => {
                    if index >= turn.hand.len() {
                        input.clear();
                        println!("Invalid input. Please try again.");
                        continue;
                    }

                    break index;
                },
                Err(_) => {
                    input.clear();
                    println!("Invalid input. Please try again.");
                    continue;
                }
            }
        };

        let mut card = turn.hand[index];

        match card {
            Card::Wild { .. } => {
                let color = Human::get_color();

                color.map(|color| TurnResult::Played(*card.with_color(color).unwrap()))
            },
            Card::DrawFour { .. } => {
                let color = Human::get_color();

                color.map(|color| TurnResult::Played(*card.with_color(color).unwrap()))
            },
            _ => {
                Some(TurnResult::Played(card))
            }
        }
    }

    fn get_color() -> Option<CardColor> {
        let mut input = String::new();
        loop {
            println!("Enter a color to choose, or type 'back' to go back to the decision screen.");

            stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            if input == "back" {
                return None;
            }

            if let Ok(color) = CardColor::from_str(&input) {
                return Some(color);
            } else {
                println!("{input} is not a valid color!");
                continue;
            }
        }
    }
}

impl Player for Human {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_turn(&mut self, turn: &Turn) -> TurnResult {
        println!("Its your turn.");

        let can_play = !turn.hand.is_empty();

        loop {
            if can_play {
                println!("What would you like to do? [Play], [Draw]");
            }
            else {
                println!("You have no cards to play. You must draw.");
            }

            if can_play {
                let action = self.get_action(turn);

                if let Some(action) = action {
                    break action;
                }
            }
            else {
                break TurnResult::Drew
            }
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
                println!("You drew {} cards. [{}]", observed_cards.len(), observed_cards.iter().skip(1).fold(observed_cards[0].to_string(), |acc, card| acc + &format!(", {card}")));
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
        let draw_or_pick = self.ran.gen_range(0..=100);

        if draw_or_pick <= 30 || turn.hand.is_empty() {
            return TurnResult::Drew;
        }

        let index = self.ran.gen_range(0..turn.hand.len());

        let mut picked_card = turn.hand[index];

        // order the collection by length of the group
        let preferred_color = turn.hand
            .iter()
            .filter_map(|c| c.color())
            .collect::<Vec<CardColor>>()
            .group_by(|c, n| c == n)
            .max_by_key(|item| item.len())
            .map_or(CardColor::Red, |color| color[0]);

        match picked_card {
            Card::Wild { .. } => {
                TurnResult::Played(*picked_card.with_color(preferred_color).unwrap())
            },
            Card::DrawFour { .. } => {
                TurnResult::Played(*picked_card.with_color(preferred_color).unwrap())
            },
            _ => {
                TurnResult::Played(picked_card)
            }
        }
    }

    fn observe_turn(&self, _other: &dyn Player, _card: &Card) {
        // Nothing to do here.
    }

    fn observe_turn_skip(&self, _observed_cards: Option<Vec<&Card>>) {
       // Nothing to do; the game loop handles insertion
    }
}
