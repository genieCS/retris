use cursive::{
    event::{Event, EventResult, Key},
    View,
    Vec2, Printer,
};
use crate::board::Board;
use crate::current::BOARD_WIDTH;
use crate::queue::Queue;

pub struct Tetris {
    pub board: Board,
    pub queue: Queue,
}

impl Tetris {
    pub fn new() -> Tetris {
        Tetris {
            board: Board::new(),
            queue: Queue::new(BOARD_WIDTH),
        }
    }

    pub fn on_down(&mut self, is_drop: bool) -> EventResult {
        let stopped = self.board.on_down(is_drop);
        if stopped {
            let block = self.queue.pop_and_spawn_new_block();
            self.board.insert_new_block(block);
        }
        EventResult::Consumed(None)
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
            _ => self.board.on_event(event)
        }
    }
}