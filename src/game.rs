

use crate::card::{Card, Deck};
use crate::player::Player;

pub struct GameState<'a> {
    deck: Deck,
    discard: Vec<Card>,
    players: Vec<(&'a mut dyn Player, Vec<Card>)>,
    current_player: usize,
    direction: Direction,
    to_draw: u8,
}

pub struct Turn<'a> {
    pub to_draw: u8,
    pub hand: &'a mut Vec<Card>,
    draw_pile: &'a mut Deck,
    discard_pile: &'a mut Vec<Card>,
}

pub enum TurnResult {
    Played(Card),
    Drew,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Clockwise,
    CounterClockwise,
}


impl<'a> GameState<'a> {
    pub fn new(players: Vec<&'a mut dyn Player>) -> GameState<'a> {
        GameState {
            deck: Deck::generate(),
            discard: vec![],
            players: players.into_iter().map(|p| (p, vec![])).collect(),
            current_player: 0,
            direction: Direction::Clockwise,
            to_draw: 0,
        }
    }

    pub fn start(&mut self) -> ! {
        self.deck.shuffle();

        for (_, hand) in self.players.iter_mut() {

            let insert = self.deck.draw_multiple(7);
            hand.extend(insert);
        }

        loop {
            let top_card = self.deck.draw().unwrap();

            match top_card {
                Card::Wild { color: _ } => {
                    self.deck.reinsert_random(top_card);
                }
                Card::DrawFour { color: _ } => {
                    self.deck.reinsert_random(top_card);
                }
                _ => {
                    self.discard.push(top_card);
                    println!("The top card is: {}", self.discard.last().unwrap());
                    break;
                }
            }
        }

        loop {

            std::thread::sleep(std::time::Duration::from_millis(600));

            self.current_player = self.next_player();
            let (current_player, player_hand) = self.players.get_mut(self.current_player).unwrap();

            let playable_player_hand = &mut Self::get_playable_hand(player_hand, self.discard.last().unwrap(), self.to_draw);

            let turn = Turn {
                hand: playable_player_hand,
                draw_pile: &mut self.deck,
                discard_pile: &mut self.discard,
                to_draw: self.to_draw,
            };

            let card_selection = current_player.execute_turn(&turn);

            match card_selection {
                TurnResult::Played(card) => {
                    player_hand.remove(player_hand.iter().position(|c| *c == card).unwrap());
                    self.discard.push(card);

                    println!("{} played {}", current_player.name(), card);
                }
                TurnResult::Drew => {
                    let to_draw = if self.to_draw > 0 {
                        self.to_draw
                    } else {
                        1
                    };

                    let cards = &self.deck.draw_multiple(to_draw);
                    self.to_draw = 0;

                    player_hand.extend(cards);

                    println!("{} drew {} cards", current_player.name(), to_draw);

                    current_player.observe_turn_skip(Some(cards.iter().collect()));
                }
            }

            if player_hand.is_empty() {
                println!("{} won!", current_player.name());

                std::thread::sleep(std::time::Duration::from_secs(7));
                std::process::exit(0);
            }

            if self.deck.is_empty() || self.to_draw > self.deck.cards.len() as u8 {
                self.deck.reinsert(self.discard.drain(..self.discard.len()).collect());
            }

            if matches!(self.discard.last(), Some(Card::Skip { .. }))  {

                self.current_player = self.next_player();
                let next_player = &self.players.get_mut(self.current_player).unwrap().0;

                next_player.observe_turn_skip(None);

                println!("{}'s turn was skipped", next_player.name());
                continue;
            }

            if matches!(card_selection, TurnResult::Played(_)) {
                self.to_draw += match self.discard.last().unwrap() {
                    Card::DrawTwo { .. } => 2,
                    Card::DrawFour { .. } => 4,
                    _ => 0,
                };
            }

            if matches!(self.discard.last(), Some(Card::DrawTwo { .. })) || matches!(self.discard.last(), Some(Card::DrawFour { .. })) {

                let next_player_index = self.next_player();
                let (next_player, next_hand) = self.players.get_mut(next_player_index).unwrap();

                let should_skip = !self.to_draw != 0 && !Self::contains_special_card(next_hand, self.discard.last().unwrap());

                if should_skip {
                    let cards = self.deck.draw_multiple(self.to_draw);

                    next_player.observe_turn_skip(Some(cards.iter().collect()));

                    next_hand.extend(cards);

                    if self.to_draw > 0 {
                        println!("{} drew {} cards", next_player.name(), self.to_draw);
                    }
                    else
                    {
                        println!("{}'s turn was skipped", next_player.name());
                    }

                    self.to_draw = 0;
                    self.current_player = self.next_player();
                }
            }

            if matches!(self.discard.last(), Some(Card::Reverse { .. })) {
                self.direction = match self.direction {
                    Direction::Clockwise => Direction::CounterClockwise,
                    Direction::CounterClockwise => Direction::Clockwise,
                };
            }
        }
    }

    fn get_playable_hand(hand: &[Card], card: &Card, to_draw: u8) -> Vec<Card> {

        if to_draw > 0 && matches!(card, Card::DrawTwo { .. } | Card::DrawFour { .. }) {
            return hand.iter().filter(|c| **c == *card).copied().collect::<Vec<Card>>();
        }

        hand.iter()
            .filter(|c| c.can_play_on(card))
            .copied()
            .collect::<Vec<Card>>()
    }

    fn contains_special_card(hand: &[Card], card: &Card) -> bool {
        hand.iter().any(|c| *c == *card)
    }
    fn can_play(hand: &mut Vec<Card>, card: &Card) -> bool {
        hand.iter().any(|c| c.can_play_on(card))
    }

    fn next_player(&self) -> usize{
        let mut index = self.current_player;
        let direction = self.direction;

        match direction {
            Direction::Clockwise => {
                index = (index + 1) % self.players.len()
            },
            Direction::CounterClockwise => {

                if index == 0 {
                    index = self.players.len() - 1;
                } else {
                    index -= 1;
                }
            }
        };

        index
    }
}