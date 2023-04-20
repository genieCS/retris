use cursive::{
    theme::{Color, ColorStyle},
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
        printer.with_color(ColorStyle::new(Color::Rgb(0,0,0), Color::Rgb(255,255,0)), |printer| {
            printer.print((0, y_padding), &format!(" {:12} ", "Manual"));
            printer.print((0 ,y_padding + 1), &format!(" {:12} ", "↑: rotate"));
            printer.print((0, y_padding + 2), &format!(" {:12} ", "↓: down"));
            printer.print((0, y_padding + 3), &format!(" {:12} ", "←: left"));
            printer.print((0, y_padding + 4), &format!(" {:12} ", "→: right"));
            printer.print((0, y_padding + 5), &format!(" {:12} ", "space: drop"));
        });
    }

    fn required_size(&mut self, _constraints: Vec2) -> Vec2 {
        Vec2::new(14, 10)
    }
}