use cursive::{
    Vec2,
    theme::{BaseColor, Color, ColorStyle},
};
use rand::Rng;

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
    pub fn cells(&self) -> Vec<Pos> {
        match self.rotation {
            Rotation::R0 => self.shape.cells(),
            Rotation::R90 => self.shape.cells().into_iter().map(|(x,y)| (-y,x)).collect(),
            Rotation::R180 => self.shape.cells().into_iter().map(|(x,y)| (-x,-y)).collect(),
            Rotation::R270 => self.shape.cells().into_iter().map(|(x,y)| (y,-x)).collect(),
        }
    }

    pub fn rotate(&self) -> Block {
        match (&self.shape, &self.rotation) {
            (Shape::O, _) => self.clone(),
            (_,Rotation::R0) => Block { shape: self.shape.clone(), rotation: Rotation::R90 },
            (_,Rotation::R90) => Block { shape: self.shape.clone(), rotation: Rotation::R180 },
            (_,Rotation::R180) => Block { shape: self.shape.clone(), rotation: Rotation::R270 },
            (_,Rotation::R270) => Block { shape: self.shape.clone(), rotation: Rotation::R0 },
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
enum Shape {
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
        match rand::thread_rng().gen_range(0..7) {
            0 => Shape::I,
            1 => Shape::O,
            2 => Shape::T,
            3 => Shape::S,
            4 => Shape::Z,
            5 => Shape::J,
            6 => Shape::L,
            _ => panic!("Invalid shape"),
        }
    }

    fn cells(&self) -> Vec<Pos> {
        match self {
            Shape::I => vec![(-2,0),(-1,0),(0,0),(1,0)],
            Shape::O => vec![(-1,-1),(0,-1),(0,0),(-1,0)],
            Shape::T => vec![(-1,0),(0,-1),(0,0),(1,0)],
            Shape::S => vec![(-1,0),(0,0),(0,-1),(1,-1)],
            Shape::Z => vec![(0,0),(1,0),(0,-1),(-1,-1)],
            Shape::J => vec![(-1,-1),(-1,0),(0,0),(1,0)],
            Shape::L => vec![(-1,0),(0,0),(1,0),(1,-1)],
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