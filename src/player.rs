use std::io::stdin;
use std::ops::Deref;
use std::str::FromStr;
use crate::card::{Card, CardColor};
use rand::{Rng, RngCore};
use rand::distributions::WeightedIndex;
use crate::game::{GameState, Turn, TurnResult};

pub struct Human {
    name: String,
}

#[derive(Debug, Clone, Copy)]
pub enum AIDifficulty {
    Easy,
    Medium,
    Hard,
}

pub struct Ai<'a, R: RngCore> {
    ran: &'a mut R,
    name: String,
    difficulty: AIDifficulty,
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

impl Human {
    pub fn new(name: String) -> Human {
        Human {
            name,
        }
    }

fn get_action(&mut self, turn: &Turn) -> Option<TurnResult> {
    let mut input = String::new();

    loop {
        stdin().read_line(&mut input).unwrap();
        let cur_input = input.trim().to_lowercase();

        match cur_input.as_str() {
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

        for (i, card) in turn.playable_hand.iter().enumerate() {
            let card = GameState::get_colorized_card_name(*card);
            println!("{i}: {card}");
        }

        println!("Enter a number to select a card, or type 'back' to go back to the decision screen.");

        let index = loop {
            stdin().read_line(&mut input).unwrap();
            let cur_input = input.trim().to_lowercase();

            if cur_input == "back" {
                return None;
            }

            // TODO: Clear?
            match cur_input.parse::<usize>() {
                Ok(index) => {
                    if index >= turn.playable_hand.len() {
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

        let mut card = turn.playable_hand[index];

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
            let cur_input = input.trim().to_lowercase();

            if cur_input == "back" {
                return None;
            }

            if let Ok(color) = CardColor::from_str(&cur_input) {
                return Some(color);
            } else {
                println!("{cur_input} is not a valid color!");
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

        let can_play = !turn.playable_hand.is_empty();

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
                println!("You drew a {}.", GameState::get_colorized_card_name(*observed_cards[0]));
            }
            else {
                println!("You drew {} cards. [{}]", observed_cards.len(), observed_cards
                    .iter()
                    .skip(1)
                    .fold(GameState::get_colorized_card_name(*observed_cards[0]), |acc, card| acc + &format!(", {}", GameState::get_colorized_card_name(**card))))
            }
        }
        else {
            println!("You have been skipped!");
        }
    }
}

impl HumanPlayer for Human {}

impl<'a, R> Ai<'a, R> where R: RngCore {

    pub fn new(ran: &'a mut R, difficulty: AIDifficulty) -> Ai<'a, R> {
        let name = AI_NAMES[ran.gen_range(0..AI_NAMES.len())].to_string();

        Ai {
            ran,
            name,
            difficulty
        }
    }

    fn get_preferred_color(&self, turn: &Turn) -> CardColor {
        // order the collection by length of the group
        turn.playable_hand
            .iter()
            .filter_map(|c| c.color())
            .collect::<Vec<CardColor>>()
            .group_by(|c, n| c == n)
            .max_by_key(|item| item.len())
            .map_or(CardColor::Red, |color| color[0])
    }

    // Picks at random.
fn easy(&mut self, turn: &Turn) -> TurnResult {
    let index = self.ran.gen_range(0..turn.playable_hand.len());

    let mut picked_card = turn.playable_hand[index];

    // order the collection by length of the group
    let preferred_color = Self::get_preferred_color(self, turn);

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

    // Picks the card that will get rid of the most cards.
    fn medium(&mut self, turn: &Turn) -> TurnResult {
        let preferred_color = turn.full_hand
            .iter()
            .filter_map(|c| c.color())
            .collect::<Vec<CardColor>>()
            .group_by(|c, n| c == n)
            .max_by_key(|item| item.len())
            .map_or(CardColor::Red, |color| color[0]);

        let weights = vec![0.3, 0.2, 0.05, 0.2, 0.25];
        let card_preferences = [Card::DrawTwo { color: CardColor::Red }, Card::Skip { color: CardColor::Red }, Card::DrawFour { color: None }, Card::Reverse { color: CardColor::Red }, Card::Wild { color: None }];

        let weight_idx = &WeightedIndex::new(&weights).unwrap();
        let mut weight_iter = self.ran.sample_iter(weight_idx);

        let mut picked_card: Card = turn.playable_hand[0];

        if turn.playable_hand.iter().any(|c| matches!(c, Card::Wild { .. } | Card::DrawFour { .. } | Card::Reverse { .. } | Card::Skip { .. } | Card::DrawTwo { .. })) {
            for _ in 0..=10 {
                let selection = weight_iter.next().unwrap(); // Safe; method is guaranteed to return a value.

                let mut card = card_preferences[selection];

                let of_type = turn.playable_hand
                    .iter()
                    .find
                    (
                        |c|
                            if c.color().is_none() { **c == card }
                            else { card.with_color(c.color().unwrap()).unwrap() == *c }
                    );

                if let Some(card) = of_type {
                    picked_card = *card;
                    break;
                }

            }
        }
        else {
            picked_card = turn.playable_hand[self.ran.gen_range(0..turn.playable_hand.len())];
        }

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

    // Picks the card that will be most effective dependent on hand size,
    // last played card, and other factors.
    fn hard(&mut self, turn: &Turn) -> TurnResult {
        let full_hand_size = turn.full_hand.len();
        let last_color = turn.last_card.color().unwrap();

        let should_stack = matches!(turn.last_card, Card::DrawTwo { .. } | Card::DrawFour { .. });

        if should_stack {
            let preferred_color = Self::get_preferable_color(turn.full_hand, last_color);

            if matches!(turn.last_card, Card::DrawFour { .. })
            {
                // We create a card here because we don't need to iterate the hand to find the card;
                // the fact we got here means we have a draw four, and the server will pluck it from
                // our hand when we return, so this is fine.
                return TurnResult::Played(Card::DrawFour { color: Some(preferred_color) });
            }
            else {
                let preferred_card = turn.playable_hand
                    .iter()
                    .find(|c| matches!(*c, Card::DrawTwo { color: preferred_color }))
                    .map_or(turn.playable_hand.iter().find(|c| matches!(*c, Card::DrawTwo { .. })), |c| Some(c));

                if let Some(card) = preferred_card {
                    return TurnResult::Played(*card);
                }
            }
        }

        let can_afford_change = turn.playable_hand
            .iter()
            .filter_map(|c| c.color())
            .filter(|c| *c != last_color)
            .collect::<Vec<CardColor>>()
            .group_by(|c, n| c == n)
            .fold((usize::MAX, Vec::<&[CardColor]>::new()), |(max, mut list), acc_list| {
                if acc_list.len() > max {
                    let index = list.iter().position(|c| c.len() == acc_list.len()).unwrap();
                    list.remove(index);
                    list.push(acc_list);

                    (acc_list.len(), list)
                }
                else {
                    list.push(acc_list);
                    (max, list)
                }
            })
            .1
            .iter()
            .max_by_key(|item| item.len())
            .map_or(false, |color| color.len() > full_hand_size / 2);

        let mut color_changing_cards = turn.playable_hand
            .iter()
            .copied()
            .filter(|c| matches!(c, Card::Wild { .. } | Card::DrawFour { .. } | Card::DrawTwo { .. }))
            .collect::<Vec<Card>>();

        let mut special_cards = turn.playable_hand
            .iter()
            .copied()
            .filter(|c| matches!(c, Card::Reverse { .. } | Card::Skip { .. } | Card::DrawTwo { .. } | Card::Wild { .. } | Card::DrawFour { .. }))
            .collect::<Vec<Card>>();


        let plan_to_change = self.ran.gen_range(0..=100) % std::cmp::max(50 - (turn.full_hand.len() * 2), 1) == 0;

        let weights = vec![0.4, 0.1, 0.35, 0.05, 0.15];

        let card_types = [Card::DrawTwo { color: CardColor::Red }, Card::Skip { color: CardColor::Red }, Card::DrawFour { color: None }, Card::Reverse { color: CardColor::Red }, Card::Wild { color: None }];

        if can_afford_change && plan_to_change {
            let index = self.ran.gen_range(0..color_changing_cards.len());
            let mut picked_card = color_changing_cards[index];

            let preferred_color = Self::get_preferable_color(turn.full_hand, last_color);

            let weight_idx = &WeightedIndex::new(&weights).unwrap();
            let mut weight_iter = self.ran.sample_iter(weight_idx);

            for _ in 0..10 {
                let index = weight_iter.next().unwrap();
                let card = card_types.get(index).unwrap();

                let playable_card = special_cards
                    .iter()
                    .filter(|c| c.color().is_some())
                    .find(|c| c.is_equivalent(card));

                if let Some(card) = playable_card {
                    picked_card = *card;
                    break;
                }
            }

            let picked_card = match picked_card.with_color(preferred_color) {
                Some(card) => *card,
                None => picked_card
            };

            return TurnResult::Played(picked_card);
        }

        // If we can neither afford to nor want to change colors, play the best available numeric card.

        let current_color_cards = turn.playable_hand
            .iter()
            .filter(|c| c.color().is_some())
            .filter(|c| c.color().unwrap() == last_color)
            .copied()
            .collect::<Vec<Card>>();

        if !current_color_cards.is_empty() {
            let index = self.ran.gen_range(0..current_color_cards.len());
            TurnResult::Played(current_color_cards[index])
        } else {
            let index = self.ran.gen_range(0..turn.playable_hand.len());

            let card = turn.playable_hand[index];

            // N.B. This may be disadvantageous if the "preferable" color happens to be something we have
            // few of, but the goal of the "hard" AI is to make the game as frustrating for the player as possible
            // even if it comes at the cost of us making a bad move like this.
            let preferable_color = Self::get_preferable_color(turn.full_hand, last_color);

            if let Card::DrawFour { .. } = card {
                return TurnResult::Played(Card::DrawFour { color: Some(preferable_color) });
            }
            else if let Card::Wild { .. } = card {
                return TurnResult::Played(Card::Wild { color: Some(preferable_color) });
            }

            TurnResult::Played(card)
        }
    }

    /// Attempts to get the most preferable card color (e.g. the color the player has the most of, that isn't the current color).
    fn get_preferable_color(hand: &Vec<Card>, last_color: CardColor) -> CardColor {

        hand.iter()
            .filter_map(|c| c.color())
            .filter(|c| *c != last_color)
            .collect::<Vec<CardColor>>()
            .group_by(|c, n| c == n)
            .max_by_key(|item| item.len())
            .map_or(last_color, |color| color[0])

    }
}


impl<'a, R> Player for Ai<'a, R> where R : RngCore {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_turn(&mut self, turn: &Turn) -> TurnResult {
        let draw_or_pick = self.ran.gen_range(0..=100);

        let draw_modifier = match self.difficulty {
            AIDifficulty::Easy => 0,
            AIDifficulty::Medium => 10,
            AIDifficulty::Hard => 15
        };

        // Math is hard.
        if draw_or_pick % (20 + draw_modifier) == 0 || turn.playable_hand.is_empty() {
            return TurnResult::Drew;
        }

        match self.difficulty {
            AIDifficulty::Easy => {
                self.easy(turn)
            },
            AIDifficulty::Medium => {
                self.medium(turn)
            },
            AIDifficulty::Hard => {
                self.hard(turn)
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
