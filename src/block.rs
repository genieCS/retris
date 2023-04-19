use cursive::{
    theme::{ColorStyle, Color},
    Vec2,
};
use rand::Rng;

pub struct Block {
    pub lefttop_pos: Vec2,
    pub shape: Vec<Vec2>,
    pub color: ColorStyle,
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

impl Block {
    pub fn new() -> Self {
        Self {
            lefttop_pos: Vec2::new(0, 0),
            shape: Self::shape(),
            color: Self::color(),
        }
    }

    fn shape() -> Vec<Vec2> {
        let shapes = Self::shapes();
        let mut shape: Vec<Vec2> = shapes[rand::thread_rng().gen_range(0..shapes.len())].clone();
        let lr_swap = rand::thread_rng().gen::<bool>();
        if lr_swap {
            let max_x = shape.iter().map(|v| v.x).max().unwrap();
            for s in &mut shape {
                s.x = max_x - s.x;
            }
     
        }
        let rotation = rand::thread_rng().gen_range(0..4);
        for _ in 0..rotation {
            let max_y = shape.iter().map(|v| v.y).max().unwrap();
            for mut s in &mut shape {
                let prev_x = s.x;
                s.x = max_y - s.y;
                s.y = prev_x;
            }
        }
        shape
    }


    //  _ _ _ _   _      _ _    _
    // |_|_|_|_| |_|_   |_|_|  |_|_ _ _
    //           |_|_|  |_|_|  |_|_|_|_|
    //          ,  |_|,      ,       
    fn shapes() -> [Vec<Vec2>; 4] {[
        vec![Vec2::new(0,0), Vec2::new(1, 0), Vec2::new(2, 0), Vec2::new(3, 0)], 
        vec![Vec2::new(0,0),Vec2::new(0,1),Vec2::new(1,1),Vec2::new(1,2)],                                   
        vec![Vec2::new(0,0), Vec2::new(1,0), Vec2::new(0,1), Vec2::new(1,1)],
        vec![Vec2::new(0,0),Vec2::new(0,1),Vec2::new(1,1), Vec2::new(2,1), Vec2::new(3,1)],
        ]
    }

    fn color() -> ColorStyle {
        let colors = Self::colors();
        colors[rand::thread_rng().gen_range(0..colors.len())]
    }

    fn colors() -> [ColorStyle; 6] { [
        ColorStyle::new(Color::Rgb(0,0,0),Color::Rgb(255, 0, 0)),     // Red
        ColorStyle::new(Color::Rgb(0,0,0),Color::Rgb(0, 255, 0)),     // Green
        ColorStyle::new(Color::Rgb(0,0,0),Color::Rgb(255, 255, 0)),   // Yellow
        ColorStyle::new(Color::Rgb(0,0,0),Color::Rgb(0, 0, 255)),     // Blue
        ColorStyle::new(Color::Rgb(0,0,0),Color::Rgb(255, 0, 255)),   // Magenta
        ColorStyle::new(Color::Rgb(0,0,0),Color::Rgb(0, 255, 255)),   // Cyan
    ]
    }

}