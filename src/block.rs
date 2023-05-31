use cursive::{
    Vec2,
    theme::{BaseColor, Color, ColorStyle},
};
use rand::thread_rng;
use rand::seq::SliceRandom;

type Pos = (i32, i32);


#[derive(Clone, Debug)]
pub struct BlockWithPos {
    pub block: Block,
    pub pos: Vec2,
}

impl BlockWithPos {
    pub fn new(block: Block, pos: Vec2) -> Self {
        Self {
            block,
            pos,
        }
    }

    pub fn cells(&self) -> Vec<Vec2> {
        self.block.cells().into_iter().map(|cell| {
            let x = self.pos.x as i32 + cell.0;
            let y = self.pos.y as i32 + cell.1;
            Vec2::new(x as usize, y as usize)
        }).collect::<Vec<Vec2>>()
    }

    pub fn color(&self) -> ColorStyle {
        self.block.color()
    }
}

#[derive(Clone, Debug)]
pub struct Block {
    shape: Shape,
    rotation: Rotation,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            shape: Shape::random(),
            rotation: Rotation::R0,
        }
    }
}

impl Block {
    pub fn new(shape: Shape) -> Self {
        Self {
            shape,
            rotation: Rotation::R0,
        }
    }

    pub fn cells(&self) -> Vec<Pos> {
        match self.rotation {
            Rotation::R0 => self.shape.cells(),
            Rotation::R90 => self.shape.cells().into_iter().map(|(x,y)| (-y,x)).collect(),
            Rotation::R180 => self.shape.cells().into_iter().map(|(x,y)| (-x,-y)).collect(),
            Rotation::R270 => self.shape.cells().into_iter().map(|(x,y)| (y,-x)).collect(),
        }
    }

    pub fn rotate(&self, clockwise: bool) -> Block {
        if clockwise {
            self.rotate_clockwise()
        } else {
            self.rotate_counter_clockwise()
        }
    }

    fn rotate_clockwise(&self) -> Block {
        match (&self.shape, &self.rotation) {
            (Shape::O, _) => self.clone(),
            (_,Rotation::R0) => Block { shape: self.shape.clone(), rotation: Rotation::R90 },
            (_,Rotation::R90) => Block { shape: self.shape.clone(), rotation: Rotation::R180 },
            (_,Rotation::R180) => Block { shape: self.shape.clone(), rotation: Rotation::R270 },
            (_,Rotation::R270) => Block { shape: self.shape.clone(), rotation: Rotation::R0 },
        }
    }

    fn rotate_counter_clockwise(&self) -> Block {
        match (&self.shape, &self.rotation) {
            (Shape::O, _) => self.clone(),
            (_,Rotation::R0) => Block { shape: self.shape.clone(), rotation: Rotation::R270 },
            (_,Rotation::R90) => Block { shape: self.shape.clone(), rotation: Rotation::R0 },
            (_,Rotation::R180) => Block { shape: self.shape.clone(), rotation: Rotation::R90 },
            (_,Rotation::R270) => Block { shape: self.shape.clone(), rotation: Rotation::R180 },
        }
    }

    pub fn color(&self) -> ColorStyle {
        match self.shape {
            Shape::I => ColorStyle::new(Color::Dark(BaseColor::Blue), Color::Dark(BaseColor::Blue)),
            Shape::O => ColorStyle::new(Color::Dark(BaseColor::Yellow), Color::Dark(BaseColor::Yellow)),
            Shape::T => ColorStyle::new(Color::Dark(BaseColor::Magenta), Color::Dark(BaseColor::Magenta)),
            Shape::S => ColorStyle::new(Color::Dark(BaseColor::Green), Color::Dark(BaseColor::Green)),
            Shape::Z => ColorStyle::new(Color::Dark(BaseColor::Red), Color::Dark(BaseColor::Red)),
            Shape::J => ColorStyle::new(Color::Dark(BaseColor::Cyan), Color::Dark(BaseColor::Cyan)),
            Shape::L => ColorStyle::new(Color::Dark(BaseColor::White), Color::Dark(BaseColor::White)),
        }
    }
}

//       I         O         T           S           Z          J        L
//                    
//   _ _ _ _      _ _        _          _ _       _ _        _             _  
//  |_|_|_|_|    |_|_|     _|_|_      _|_|_|     |_|_|_     |_|_ _     _ _|_|  
//               |_|_|    |_|_|_|    |_|_|         |_|_|    |_|_|_|   |_|_|_| 
#[derive(Clone, Debug)]
pub enum Shape {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}  

impl Shape {
    fn random() -> Self {
        Self::all().pop().unwrap()
    }

    pub fn all() -> Vec<Shape> {
        let mut shapes = vec![Shape::I, Shape::O, Shape::T, Shape::S, Shape::Z, Shape::J, Shape::L];
        shapes.shuffle(&mut thread_rng());
        shapes
    }

    fn cells(&self) -> Vec<Pos> {
        match self {
            Shape::I => vec![(0,0),(-2,0),(-1,0),(1,0)],
            Shape::O => vec![(0,0),(-1,-1),(0,-1),(-1,0)],
            Shape::T => vec![(0,0),(-1,0),(0,-1),(1,0)],
            Shape::S => vec![(0,0),(-1,0),(0,-1),(1,-1)],
            Shape::Z => vec![(0,0),(1,0),(0,-1),(-1,-1)],
            Shape::J => vec![(0,0),(-1,-1),(-1,0),(1,0)],
            Shape::L => vec![(0,0),(-1,0),(1,0),(1,-1)],
        }
    }
}

#[derive(Clone, Debug)]
enum Rotation {
    R0,
    R90,
    R180,
    R270,
}