use crate::board::Board;
use crate::manual::Manual;
use crate::queue::Queue;
use cursive::{
    event::{Callback, Event, EventResult, Key},
    views::Dialog,
    Printer, Vec2, View,
    theme::{Color, ColorStyle},
};

pub struct Tetris {
    manual: Manual,
    board: Board,
    queue: Queue,
    manual_size: Vec2,
    board_size: Vec2,
    is_paused: bool,
}

impl Default for Tetris {
    fn default() -> Self {
        Self::new()
    }
}

impl Tetris {
    pub fn new() -> Tetris {
        let background_color = (
            ColorStyle::new(Color::Rgb(0,0,0), Color::Rgb(0,0,0)),
            ColorStyle::new(Color::Rgb(0,0,0), Color::Rgb(30,30,30)),
        );
        let warning_color = ColorStyle::new(Color::Rgb(0,0,0), Color::Rgb(200,200,0));
        let mut manual = Manual::new();
        let mut board = Board::new(background_color, warning_color, 10, 20);
        let manual_size = manual.required_size(Vec2::new(0,0));
        let board_size = board.required_size(Vec2::new(0,0));
        Tetris {
            manual,
            board,
            queue: Queue::new(),
            manual_size,
            board_size,
            is_paused: false,
        }
    }

    fn on_down(&mut self, is_drop: bool) -> EventResult {
        if self.is_paused {
            return EventResult::Consumed(None);
        }
        let (gameover, merged) = self.board.on_down(is_drop);
        if gameover {
            self.is_paused = true;
            return EventResult::Consumed(Some(Callback::from_fn(move |s| {
                s.add_layer(Dialog::info("Game Over!"));
            })));
        }
        if merged {
            let block = self.queue.pop_and_spawn_new_block();
            self.board.insert(block);
        }
        EventResult::Consumed(None)
    }

    fn new_game(&mut self) -> EventResult {
        self.board.renew();
        self.queue.renew();
        self.is_paused = false;
        EventResult::Consumed(None)
    }

    fn stop_and_resume(&mut self) -> EventResult {
        self.is_paused = !self.is_paused;
        EventResult::Consumed(None)
    }

    fn pass_event_to_board(&mut self, event: Event) -> EventResult {
        if self.is_paused {
            EventResult::Consumed(None)
        } else {
            self.board.on_event(event)
        }
    }
}

impl View for Tetris {
    fn draw(&self, printer: &Printer) {
        let x_padding = 1;
        let y_padding = 1;
        let manual_padding = Vec2::new(x_padding, y_padding);
        let board_padding = Vec2::new(x_padding + self.manual_size.x, y_padding);
        let queue_padding = Vec2::new(x_padding + self.manual_size.x + self.board_size.x, y_padding);

        let manual_printer = printer.offset(manual_padding);
        let board_printer = printer.offset(board_padding);
        let queue_printer = printer.offset(queue_padding);

        self.manual.draw(&manual_printer);
        self.board.draw(&board_printer);
        self.queue.draw(&queue_printer);
    }

    fn required_size(&mut self, constraints: Vec2) -> Vec2 {
        let manual_size = self.manual.required_size(constraints);
        let board_size = self.board.required_size(constraints);
        let queue_size = self.queue.required_size(constraints);
        Vec2::new(manual_size.x + board_size.x + queue_size.x + 10, board_size.y)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Refresh | Event::Key(Key::Down) => self.on_down(false),
            Event::Char(' ') => self.on_down(true),
            Event::Char('n') => self.new_game(),
            Event::Char('s') => self.stop_and_resume(),
            _ => self.pass_event_to_board(event),
        }
    }
}
