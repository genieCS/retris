use cursive::{
    Vec2,
    theme::{ColorStyle},
};
use std::ops::Index;
use crate::current::Current;

pub struct ColorGrid {
    colors: Vec<Vec<ColorStyle>>,
    pub background_color: ColorStyle,
    pub warning_color: ColorStyle,
}

impl ColorGrid {
    pub fn new(background_color: ColorStyle, warning_color: ColorStyle, width: usize, height: usize) -> Self {
        Self {
            colors: vec![vec![background_color; width]; height],
            background_color,
            warning_color,
        }
    }

    pub fn width(&self) -> usize {
        self.colors[0].len()
    }

    pub fn height(&self) -> usize {
        self.colors.len()
    }

    pub fn clear(&mut self) {
        for row in &mut self.colors {
            for color in row.iter_mut() {
                *color = self.background_color;
            }
        }
    }

    pub fn is_occupied(&self, pos: Vec2) -> bool {
        self.colors[pos.y][pos.x] != self.background_color && self.colors[pos.y][pos.x] != self.warning_color
    }

    pub fn merge_block(&mut self, current: &Current) -> bool {
        self.fill_board_with_block(current);
        self.remove_rows_if_possible();
        true
    }

    fn fill_board_with_block(&mut self, current: &Current) {
        for cell in current {
            self.colors[cell.y][cell.x] = current.color();
        }
    }

    fn remove_rows_if_possible(&mut self) {
        let mut rows_to_remove = Vec::new();
        for (i, row) in self.colors.iter_mut().enumerate().rev() {
            let mut remove = true;
            for cell in row.iter() {
                if cell == &self.background_color {
                    remove = false;
                    break;
                }
            }
            if remove {
                rows_to_remove.push(i);
            }
        }
        self.remove_rows(rows_to_remove);
    }

    fn remove_rows(&mut self, rows_to_remove: Vec<usize>) {
        if rows_to_remove.is_empty() {
            return;
        }
        let mut fill_y = self.height() - 1;
        let mut check_y = self.height() - 1;
        for row in rows_to_remove {
            while check_y > row {
                if fill_y != check_y {
                    self.colors[fill_y] = self.colors[check_y].clone();
                }
                fill_y -= 1;
                check_y -= 1;
            }
            check_y = row - 1;
        }
        while check_y > 0 {
            self.colors[fill_y] = self.colors[check_y].clone();
            fill_y -= 1;
            check_y -= 1;
        }
        while fill_y > 0 {
            self.colors[fill_y] = vec![self.background_color; self.width()];
            fill_y -= 1;
        }
    }
}

impl Index<usize> for ColorGrid {
    type Output = Vec<ColorStyle>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.colors[index]
    }
}