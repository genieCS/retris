use cursive::{
    theme::{ColorStyle},
};
use std::ops::Index;
use crate::block::BlockWithPos;

pub struct ColorGrid {
    colors: Vec<Vec<ColorStyle>>,
    pub background_color: (ColorStyle, ColorStyle),
    pub warning_color: ColorStyle,
}

impl ColorGrid {
    pub fn new(background_color: (ColorStyle, ColorStyle), warning_color: ColorStyle, width: usize, height: usize) -> Self {
        let mut colors = Vec::with_capacity(height);
        for h in 0..height {
            let mut row = Vec::with_capacity(width);
            for w in 0..width {
                    let color = if (w + h) % 2 == 0 {
                        background_color.0
                    } else {
                        background_color.1
                    };
                    row.push(color);
            }
            colors.push(row);
        }
        Self {
            colors,
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

    fn set_background(&mut self, x: usize, y: usize) {
        let color = if (x + y) % 2 == 0 {
            self.background_color.0
        } else {
            self.background_color.1
        };
        self.colors[y][x] = color;
    }

    pub fn clear(&mut self) {
        for x in 0..self.width() {
            for y in 0..self.height() {
                self.set_background(x, y);
            }
        }
    }

    pub fn is_occupied(&self, x: usize, y: usize) -> bool {
        self.colors[y][x] != self.background_color.0 && 
        self.colors[y][x] != self.background_color.1 &&
        self.colors[y][x] != self.warning_color
    }

    pub fn merge_block(&mut self, block: &BlockWithPos) -> bool {
        self.fill_board_with_block(block);
        self.remove_rows_if_possible();
        true
    }

    fn fill_board_with_block(&mut self, block: &BlockWithPos) {
        for cell in block.cells() {
            self.colors[cell.y][cell.x] = block.color();
        }
    }

    fn remove_rows_if_possible(&mut self) {
        let mut rows_to_remove = Vec::new();
        for _y in 0..self.height() {
            let y = self.height() - _y - 1;
            let mut remove = true;
            for x in 0..self.width() {
                if !self.is_occupied(x, y) {
                    remove = false;
                    break;
                }
            }
            if remove {
                rows_to_remove.push(y);
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
            for x in 0..self.width() {
                self.set_background(x, fill_y);
            }
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