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
    pub is_paused: bool,
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
            is_paused: false,
        }
    }

    pub fn on_down(&mut self, is_drop: bool) -> EventResult {
        if self.is_paused {
            return EventResult::Consumed(None);
        }
        let (gameover, merged) = self.board.on_down(is_drop);
        if gameover {
            self.is_paused = true;
            self.board.toggle_pause(true);
            return EventResult::Consumed(Some(Callback::from_fn(move |s| {
                s.add_layer(Dialog::info("Game Over!"));
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
        self.is_paused = false;
        self.board.toggle_pause(false);
        EventResult::Consumed(None)
    }

    fn stop_and_resume(&mut self) -> EventResult {
        self.is_paused = !self.is_paused;
        self.board.toggle_pause(self.is_paused);
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
            Event::Char('n') => self.new_game(),
            Event::Char('s') => self.stop_and_resume(),
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
