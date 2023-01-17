use crate::card::{Card, CardColor};
use rand::Rng;

pub struct HumanPlayer {
    name: String,
    hand: Vec<Card>,
}

pub struct AiPlayer {
    name: String,
    hand: Vec<Card>,
}

pub enum Player {
    Human(HumanPlayer),
    Ai(AiPlayer),
}

const AI_NAMES: [&str; 12] = [
    "Yukii", "Kurisu", "Mayuri", "Makise", "Misa", "Rin", "Miku", "Shinobu", "Shiro", "Rem",
    "Asuna", "Yui",
];

impl Player {
    pub fn new(name: String) -> Player {
        Player::Human(HumanPlayer { name, hand: vec![] })
    }

    pub fn new_ai() -> Player {
        let ran = rand::thread_rng().gen_range(0..AI_NAMES.len());
        Player::Ai(AiPlayer {
            name: AI_NAMES[ran].to_string(),
            hand: vec![],
        })
    }

    pub fn draw_card(&mut self, card: Card) {
        match self {
            Player::Ai(ai) => ai.hand.push(card),
            Player::Human(human) => human.hand.push(card),
        }
    }

    pub fn can_play(&self) -> bool {
        !match self {
            Player::Ai(ai) => ai.hand.is_empty(),
            Player::Human(human) => human.hand.is_empty(),
        }
    }

    pub fn play(&self, previous: &Player, card: Card) {
        match self {
            Player::Ai(ai) => ai.play(card),
            Player::Human(human) => human.play(previous, card),
        }
    }
}

impl HumanPlayer {
    pub fn play(&self, previous: &Player, card: Card) {
        println!(
            "{} played a {} {:?}.",
            self.name,
            card.color().unwrap(),
            card
        );
    }

    fn input_loop(&self, card: Card) {
        loop {
            println!("What would you like to do? (1) Play a card, (2) Draw a card");

            let parsed;
            loop {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();

                if let Ok(num) = input.trim().parse::<u8>() {
                    parsed = num;
                    break;
                } else {
                    println!("Please enter a valid number.");
                }
            }
        }
    }

    fn play_card(&mut self, card: &Card) -> Option<Card> {
        println!("Which card would you like to play?");

        for (i, c) in self.hand.iter().filter(|x| x.can_play(card)).enumerate() {
            println!("{}: {} {:?}", i, c.color().unwrap(), c);
        }

        let mut parsed;
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            let parsed_input = input.trim().parse::<usize>();
            if parsed_input.is_ok() && parsed_input.to_owned().unwrap() < self.hand.len() {
                parsed = parsed_input.unwrap();
                break;
            } else if input.trim().to_lowercase() == "back" {
                return None;
            } else {
                println!("Please enter a valid number.");
                continue;
            }
        }

        let mut card = &self.hand.remove(parsed);

        let selection = match card {
            Card::Wild { color: _ } => Self::get_card_color_from_input(&card),
            Card::DrawFour { color: _ } => Self::get_card_color_from_input(&card),
            _ => None,
        };

        if let Some(color) = selection {
            // We're dealing with a special card; this is safe.
            return Some(card.with_color(&color).unwrap());
        }

        None
    }

    fn get_card_color_from_input(card: &Card) -> Option<CardColor> {
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            print!(
                "({:?}) Pick a color (Red, Green, Blue, Yellow): ",
                card.color().unwrap()
            );

            let input = input.trim().to_lowercase();

            match input.as_str() {
                "red" => return Some(CardColor::Red),
                "green" => return Some(CardColor::Green),
                "blue" => return Some(CardColor::Blue),
                "yellow" => return Some(CardColor::Yellow),
                "back" => return None,
                _ => println!("Please enter a valid color."),
            }
        }
    }
}

impl AiPlayer {
    pub fn play(&self, card: Card) {
        todo!()
    }
}
