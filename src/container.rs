use cursive::{
    View,
    Printer,
};

pub struct Container {
}

impl Container {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Container {
    fn draw(&self, printer: &Printer) {
        let padding = 5;
        for i in 0..10 {
            printer.print((2*i + 2, padding), "_");
        }
        for j in 0..20 {
            for i in 0..10 {
                printer.print((2*i + 1, j + padding + 1), "|_");
            }
            printer.print((20+1, j + padding + 1), "|");
        }
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        cursive::Vec2::new(23, 30)
    }
}