use cursive::{
    event::{Event, EventResult},
    Printer, View, Vec2,
};

pub struct Manual {
}

impl Default for Manual {
    fn default() -> Self {
        Self::new()
    }
}

impl Manual {
    pub fn new() -> Manual {
        Manual {}
    }
}

impl View for Manual {
    fn draw(&self, printer: &Printer) {
        let y_padding = 6;
            printer.print((0, y_padding), &format!(" {:18} ", "Manual"));
            printer.print((0 ,y_padding + 1), &format!(" {:18} ", "↑,e: rotate clockwise"));
            printer.print((0 ,y_padding + 2), &format!(" {:18} ", "w: rotate counterclockwise"));
            printer.print((0, y_padding + 3), &format!(" {:18} ", "↓: speed up"));
            printer.print((0, y_padding + 4), &format!(" {:18} ", "←: left"));
            printer.print((0, y_padding + 5), &format!(" {:18} ", "a: left most"));
            printer.print((0, y_padding + 6), &format!(" {:18} ", "→: right"));
            printer.print((0, y_padding + 7), &format!(" {:18} ", "d: right most"));
            printer.print((0, y_padding + 8), &format!(" {:18} ", "space: hard drop"));
            printer.print((0, y_padding + 9), &format!(" {:18} ", "m: stop and resume"));
            printer.print((0, y_padding + 10), &format!(" {:18} ", "n: new game"));
    }

    fn required_size(&mut self, _constraints: Vec2) -> Vec2 {
        Vec2::new(30, 16)
    }

    fn on_event(&mut self, _: Event) -> EventResult {
        EventResult::Ignored
    }
}