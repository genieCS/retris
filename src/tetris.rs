use crate::board::Board;
use crate::current::BOARD_WIDTH;
use crate::queue::Queue;
use cursive::{
    event::{Callback, Event, EventResult, Key},
    views::Dialog,
    Printer, Vec2, View,
};

pub struct Tetris {
    pub board: Board,
    pub queue: Queue,
}

impl Default for Tetris {
    fn default() -> Self {
        Self::new()
    }
}

impl Tetris {
    pub fn new() -> Tetris {
        Tetris {
            board: Board::new(),
            queue: Queue::new(BOARD_WIDTH),
        }
    }

    pub fn on_down(&mut self, is_drop: bool) -> EventResult {
        let (gameover, merged) = self.board.on_down(is_drop);
        if gameover {
            return EventResult::Consumed(Some(Callback::from_fn(move |s| {
                s.add_layer(Dialog::info("Game Over!"));
                s.set_fps(0);
            })));
        }
        if merged {
            let block = self.queue.pop_and_spawn_new_block();
            self.board.insert_new_block(block);
        }
        EventResult::Consumed(None)
    }

    fn new_game(&mut self) -> EventResult {
        self.board = Board::new();
        self.queue = Queue::new(BOARD_WIDTH);
        let block = self.queue.pop_and_spawn_new_block();
        self.board.insert_new_block(block);
        EventResult::Consumed(Some(Callback::from_fn(|s| {
            s.set_fps(1);
        })))
    }
}

impl View for Tetris {
    fn draw(&self, printer: &Printer) {
        self.board.draw(printer);
        self.queue.draw(printer);
    }

    fn required_size(&mut self, constraints: Vec2) -> Vec2 {
        let board_size = self.board.required_size(constraints);
        let queue_size = self.queue.required_size(constraints);
        Vec2::new(board_size.x + queue_size.x, board_size.y)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Refresh | Event::Key(Key::Down) => self.on_down(false),
            Event::Char(' ') => self.on_down(true),
            Event::Char('n') => self.new_game(),
            _ => self.board.on_event(event),
        }
    }

    fn take_focus(
            &mut self,
            source: cursive::direction::Direction,
        ) -> Result<EventResult, cursive::view::CannotFocus> {
        self.board.take_focus(source)
    }
}
