use std::{
    io::Stdout,
    sync::mpsc::Receiver,
    sync::{Arc, Condvar, Mutex},
    thread,
    time::{Duration, Instant}
};
use crossterm::{
    event,
    execute,
    event::{Event, KeyCode, KeyEventKind},
    terminal::{enable_raw_mode, EnterAlternateScreen, SetTitle}
};
use tui::{
    backend::{CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    Frame, Terminal,
    text::{Span, Text},
    widgets::{Block, List, ListItem, Widget}
};
use tui::layout::Corner;

use crate::{StatefulList};

type RenderFunc<'a> = Box<dyn FnMut(&mut Frame<CrosstermBackend<Stdout>>, Vec<Rect>) + Send + 'a>;

type RenderFunctions<'a> = Vec<(RenderFunc<'a>, Option<Box<dyn FnMut(KeyCode) -> bool + Send + 'a>>)>;

pub struct Ui<'a> {
    render_stack: Arc<Mutex<RenderFunctions<'a>>>,
    event_log: Arc<Mutex<Vec<Span<'a>>>>,
    player_hand: Arc<Mutex<Vec<Span<'a>>>>,
}

impl<'a> Ui<'a> {
    pub fn new() -> Self {
        Ui {
            render_stack: Arc::new(Mutex::new(vec![])),
            event_log: Arc::new(Mutex::new(vec![])),
            player_hand: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn init(&self) -> Result<Terminal<CrosstermBackend<Stdout>>, std::io::Error> {
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        execute!(terminal.backend_mut(), EnterAlternateScreen, crossterm::terminal::SetTitle("UNO CLI Edition by VelvetThePanda"))?;
        enable_raw_mode()?;

         Ok(terminal)
    }

    pub fn push_menu_widget<F>(&self, func: F, key_handler: Option<impl FnMut(KeyCode) -> bool + Send + 'a>)
        where F: FnMut(&mut Frame<CrosstermBackend<Stdout>>, Vec<Rect>) + 'a + Send {
        let mut stack = self.render_stack.lock().unwrap();

        stack.push((Box::new(func), key_handler.map(|f| Box::new(f) as Box<dyn FnMut(KeyCode) -> bool + Send + 'a>)));
    }

    pub fn update_player_hand(&self, hand: Vec<Span<'a>>) {
        let mut player_hand = self.player_hand.lock().unwrap();
        *player_hand = hand;
    }

    pub fn push_event(&self, event: Span<'a>) {
        let mut event_log = self.event_log.lock().unwrap();
        event_log.push(event);
    }

    pub fn run(&self, mut terminal: Terminal<CrosstermBackend<Stdout>>, events: Receiver<UiEvent>) -> Result<(), std::io::Error> {
        loop {
            if let Ok(event) = events.try_recv() {
                match event {
                    UiEvent::Rerender(fps) => {
                        execute!(terminal.backend_mut(), SetTitle(format!("UNO CLI Edition by VelvetThePanda | {} FPS", fps)))?;

                        terminal.draw(|f| {
                            let size = f.size();

                            let main_layout = Layout::default()
                                .direction(Direction::Horizontal)
                                .constraints([Constraint::Min(20), Constraint::Min(80)].as_ref())
                                .split(size);

                            let play_area = Layout::default()
                                .direction(Direction::Vertical)
                                .constraints([Constraint::Min(20), Constraint::Min(20)].as_ref())
                                .split(main_layout[0]);

                            let events_area = Layout::default()
                                .direction(Direction::Vertical)
                                .constraints([Constraint::Min(20), Constraint::Min(20)].as_ref())
                                .split(main_layout[1]);

                            let events = Block::default().title("Events").borders(tui::widgets::Borders::ALL);
                            let event_log = self.event_log.lock().unwrap();

                            let list = List::new(event_log.iter().rev().take(20).cloned().map(|i| ListItem::new(Text::from(i))).collect::<Vec<_>>())
                                .block(events)
                                .start_corner(Corner::BottomLeft)
                                .highlight_style(tui::style::Style::default().fg(tui::style::Color::Yellow));

                            // Don't deadlock if the render hook wants to push an event
                            drop(event_log);

                            if let Ok(mut stack) = self.render_stack.lock() {
                                if let Some((render_func, _)) = stack.last_mut() {
                                    render_func(f, play_area);
                                }
                            }
                            else {

                                let player_hand_block = Block::default().title("Player Hand").borders(tui::widgets::Borders::ALL);
                                let player_hand_list = self.player_hand.lock().unwrap();

                                let player_hand = List::new(player_hand_list.iter().cloned().map(|i| ListItem::new(Text::from(i))).collect::<Vec<_>>())
                                    .block(player_hand_block)
                                    .highlight_style(tui::style::Style::default().fg(tui::style::Color::Yellow));


                                f.render_widget(player_hand, play_area[1]);
                            }

                            f.render_widget(list, events_area[0]);

                        })?;
                    }
                    UiEvent::Input(key) => {
                        match key {
                            KeyCode::Down | KeyCode::Up | KeyCode::Enter | KeyCode::Esc => {
                                if let Ok(mut stack) = self.render_stack.lock() {
                                    if let Some((_, Some(func))) = stack.last_mut() {
                                        if func(key) {
                                            continue;
                                        }
                                        else {
                                            // We're done with the menu; should only be returned on enter
                                            stack.pop();
                                        }
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

pub enum UiEvent {
    Input(KeyCode),
    Rerender(u64)
}

pub struct Events;

impl Events {
    pub fn start() -> Receiver<UiEvent> {
        let (tx, rx) = std::sync::mpsc::channel();

        let mut now = Instant::now();
        let delta = Duration::from_millis(1000 / 24);

        thread::spawn(move || {
            loop {

                if event::poll(Duration::from_millis(1000 / 30)).unwrap_or(false) {
                    if let Ok(Event::Key(key)) = event::read() {
                        if key.kind == KeyEventKind::Press && tx.send(UiEvent::Input(key.code)).is_err() {
                            break;
                        }
                    }
                }

                let frame_delta = Instant::now().duration_since(now);
                if frame_delta >= delta {
                    now = Instant::now();

                    let fps = (1f32 / frame_delta.as_secs_f32()) as u64;

                    if tx.send(UiEvent::Rerender(fps)).is_err() {
                        break;
                    }
                }
            }
        });

        rx
    }
}