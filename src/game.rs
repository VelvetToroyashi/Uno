use std::marker::PhantomData;
use rand::prelude::SliceRandom;
use crate::card::{Card, Deck};
use crate::player::Player;

pub struct GameState<'a> {
    deck: Deck,
    discard: Vec<Card>,
    players: Vec<&'a mut dyn Player>,
    current_player: usize,
    direction: Direction,
}

pub struct Turn<'a> {
    hand: &'a mut Vec<Card>,
    draw_pile: &'a mut Deck,
    discard_pile: &'a mut Vec<Card>,
    player: &'a dyn Player,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Clockwise,
    CounterClockwise,
}

impl<'a> GameState<'a> {
    pub fn new(players: Vec<&'a mut dyn Player>) -> GameState {
        GameState {
            deck: Deck::generate(),
            discard: vec![],
            players,
            current_player: 0,
            direction: Direction::Clockwise,
        }
    }

    pub fn start(&mut self) -> ! {
        self.deck.shuffle();

        for player in self.players.iter() {

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

        let mut last_card = self.discard.last().unwrap();

        loop {

            let current_player = &*self.players[self.current_player];

            let turn = Turn {
                hand: current_player.hand(),
                draw_pile: &mut self.deck,
                discard_pile: &mut self.discard,
                player: current_player,
            };




        }
    }

    fn next_player(&'a mut self) -> &'a dyn Player {
        let mut index = self.current_player;
        let direction = self.direction;

        let next_player = match direction {
            Direction::Clockwise => {
                index = (index + 1) % self.players.len();

                &self.players[index]
            },
            Direction::CounterClockwise => {

                if index == 0 {
                    index = self.players.len() - 1;
                } else {
                    index -= 1;
                }

                &self.players[index]
            }
        };

        self.current_player = index;

        *next_player
    }
}