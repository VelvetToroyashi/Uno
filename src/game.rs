use std::borrow::{BorrowMut};
use crate::card::{Card, Deck};
use crate::player::Player;

pub struct GameState {
    deck: Deck,
    discard: Vec<Card>,
    players: Vec<Box<dyn Player>>,
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
    Drew(u8),
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Clockwise,
    CounterClockwise,
}


impl GameState {
    pub fn new(players: Vec<Box<dyn Player>>) -> GameState {
        GameState {
            deck: Deck::generate(),
            discard: vec![],
            players,
            current_player: 0,
            direction: Direction::Clockwise,
            to_draw: 0,
        }
    }

    pub fn start(&mut self) -> ! {
        self.deck.shuffle();

        for player in self.players.iter_mut() {

            let insert = self.deck.draw_multiple(7);
            player.hand().extend(insert);
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
                    break;
                }
            }
        }

        loop {

            self.current_player = self.next_player();
            let mut current_player = self.players[self.current_player].as_mut();

            let turn = Turn {
                hand: current_player.hand(),
                draw_pile: &mut self.deck,
                discard_pile: &mut self.discard,
                to_draw: self.to_draw,
            };

            let card_selection = current_player.execute_turn(&turn);

            match card_selection {
                TurnResult::Played(card) => {
                    self.discard.push(card);
                    current_player.hand().remove(current_player.hand().iter().position(|c| c == &card).unwrap());
                }
                TurnResult::Drew(_) => {
                    self.to_draw = 0;
                }
            }

            if current_player.hand().is_empty() {
                println!("{} won!", current_player.name());

                std::thread::sleep(std::time::Duration::from_secs(7));
                std::process::exit(0);
            }

            if self.deck.is_empty() || self.to_draw > self.deck.cards.len() as u8 {
                self.deck.reinsert(self.discard.drain(..self.discard.len()).collect());
            }

            if matches!(self.discard.last(), Some(Card::Skip { .. }))  {

                self.current_player = self.next_player();
                current_player = self.players[self.current_player].as_mut();

                current_player.observe_turn_skip(None);
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
                let next_player = self.players[next_player_index].as_mut();

                let should_skip = !self.to_draw != 0 && !Self::can_play(next_player, self.discard.last().unwrap());

                if should_skip {
                    let cards = self.deck.draw_multiple(self.to_draw);

                    next_player.observe_turn_skip(Some(cards.iter().collect()));

                    next_player.borrow_mut().hand().extend(cards);

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

    fn can_play(player: &mut dyn Player, card: &Card) -> bool {
        player.hand().iter().any(|c| c.can_play_on(card))
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