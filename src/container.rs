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
        let y_padding = 5;
        let x_padding = 1;
        for i in 0..10 {
            printer.print((2*i + 1 + x_padding, y_padding), "_");
        }
        for j in 0..20 {
            for i in 0..10 {
                printer.print((2*i + x_padding, j + y_padding + 1), "|_");
            }
            printer.print((20+x_padding, j + y_padding + 1), "|");
        }
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        cursive::Vec2::new(23, 30)
    }
}