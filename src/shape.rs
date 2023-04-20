use cursive::Vec2;
use rand::Rng;

type Positions = Vec<Vec2>;

trait ShapeTr {
    fn vectors(&self) -> Positions;

    fn shuffle(&self) -> Shape;

    fn rotate(&self) -> Shape;
}

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    S1(S1),
    S2(S2),
    S3(S3),
    S4(S4),
}

impl Default for Shape {
    fn default() -> Self {
        let shapes: [Shape; 4] = [Shape::S1(S1::S1_1), Shape::S2(S2::S2_1), Shape::S3(S3::S3_1), Shape::S4(S4::S4_1)];
        let mut shape = shapes[rand::thread_rng().gen_range(0..shapes.len())];
        let repeat = rand::thread_rng().gen_range(0..8);
        for _ in 0..repeat {
            shape = shape.shuffle();
        }
        shape as Shape
    }
}

impl Shape {
    pub fn vectors(&self) -> Positions {
        match self {
            Shape::S1(shape) => shape.vectors(),
            Shape::S2(shape) => shape.vectors(),
            Shape::S3(shape) => shape.vectors(),
            Shape::S4(shape) => shape.vectors(),
        }
    }

    fn shuffle(&self) -> Shape {
        match self {
            Shape::S1(shape) => shape.shuffle(),
            Shape::S2(shape) => shape.shuffle(),
            Shape::S3(shape) => shape.shuffle(),
            Shape::S4(shape) => shape.shuffle(),
        }
    }

    fn rotate(&self) -> Shape {
        match self {
            Shape::S1(shape) => shape.rotate(),
            Shape::S2(shape) => shape.rotate(),
            Shape::S3(shape) => shape.rotate(),
            Shape::S4(shape) => shape.rotate(),
        }
    }
}

//    S1_1      S1_2
//  _ _ _ _      _
// |_|_|_|_| ,  |_|
//              |_|
//              |_|
//              |_|
#[derive(Debug, Clone, Copy)]
pub enum S1 {
    S1_1,
    S1_2,
}

impl ShapeTr for S1 {
    fn vectors(&self) -> Positions {
        match self {
            S1::S1_1 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(2,0),Vec2::new(3,0)],
            S1::S1_2 => vec![Vec2::new(0,0),Vec2::new(0,1),Vec2::new(0,2),Vec2::new(0,3)],
        }
    }

    fn shuffle(&self) -> Shape {
        match self {
            S1::S1_1 => Shape::S1(S1::S1_2),
            S1::S1_2 => Shape::S1(S1::S1_1),
        }
    }

    fn rotate(&self) -> Shape {
        match self {
            S1::S1_1 => Shape::S1(S1::S1_2),
            S1::S1_2 => Shape::S1(S1::S1_1),
        }
    }
}

//  S2_1     S2_2    S2_3     S2_4   
//  _         _ _      _      _ _
// |_|_     _|_|_|   _|_|    |_|_|_
// |_|_|   |_|_|    |_|_|      |_|_|
//   |_|,         , |_|    ,
#[derive(Debug, Clone, Copy)]
pub enum S2 {
    S2_1,
    S2_2,
    S2_3,
    S2_4,
}

impl ShapeTr for S2 {
    fn vectors(&self) -> Positions {
        match self {
            S2::S2_1 => vec![Vec2::new(0,0),Vec2::new(0,1),Vec2::new(1,1),Vec2::new(1,2)],
            S2::S2_2 => vec![Vec2::new(1,0),Vec2::new(2,1),Vec2::new(0,1),Vec2::new(1,1)],
            S2::S2_3 => vec![Vec2::new(1,0),Vec2::new(0,1),Vec2::new(1,1),Vec2::new(0,2)],
            S2::S2_4 => vec![Vec2::new(0,0),Vec2::new(1,1),Vec2::new(0,1),Vec2::new(2,1)],
        }
    }

    fn shuffle(&self) -> Shape {
        match self {
            S2::S2_1 => Shape::S2(S2::S2_2),
            S2::S2_2 => Shape::S2(S2::S2_3),
            S2::S2_3 => Shape::S2(S2::S2_4),
            S2::S2_4 => Shape::S2(S2::S2_1),
        }
    }

    fn rotate(&self) -> Shape {
        match self {
            S2::S2_1 => Shape::S2(S2::S2_2),
            S2::S2_2 => Shape::S2(S2::S2_1),
            S2::S2_3 => Shape::S2(S2::S2_4),
            S2::S2_4 => Shape::S2(S2::S2_3),
        }
    }
}

//  S3_1
//  _ _ 
// |_|_|
// |_|_|
#[derive(Debug, Clone, Copy)]
pub enum S3 {
    S3_1,
}

impl ShapeTr for S3 {
    fn vectors(&self) -> Positions {
        match self {
            S3::S3_1 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(0,1),Vec2::new(1,1)],
        }
    }

    fn shuffle(&self) -> Shape {
        match self {
            S3::S3_1 => Shape::S3(S3::S3_1),
        }
    }

    fn rotate(&self) -> Shape {
        match self {
            S3::S3_1 => Shape::S3(S3::S3_1),
        }
    }
}

//    S4_1     S4_2    S4_3         S4_4    S4_5      S4_6    S4_7        S4_8
//  _           _ _    _ _ _ _        _          _     _ _    _ _ _ _    _
// |_|_ _ _    |_|_|  |_|_|_|_|      |_|   _ _ _|_|   |_|_|  |_|_|_|_|  |_|
// |_|_|_|_|   |_|          |_|      |_|  |_|_|_|_|     |_|  |_|        |_|
//             |_|                  _|_|                |_|             |_|_
//           , |_|   ,          ,  |_|_|  ,         ,   |_| ,         , |_|_|
#[derive(Debug, Clone, Copy)]
pub enum S4 {
    S4_1,
    S4_2,
    S4_3,
    S4_4,
    S4_5,
    S4_6,
    S4_7,
    S4_8,
}

impl ShapeTr for S4 {
    fn vectors(&self) -> Positions {
        match self {
            S4::S4_1 => vec![Vec2::new(0,0),Vec2::new(0,1),Vec2::new(1,1),Vec2::new(2,1), Vec2::new(3,1)],
            S4::S4_2 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(0,1),Vec2::new(0,2), Vec2::new(0,3)],
            S4::S4_3 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(2,0),Vec2::new(3,0), Vec2::new(3,1)],
            S4::S4_4 => vec![Vec2::new(1,0),Vec2::new(1,1),Vec2::new(1,2),Vec2::new(1,3), Vec2::new(0,3)],
            S4::S4_5 => vec![Vec2::new(3,0),Vec2::new(0,1),Vec2::new(1,1),Vec2::new(2,1), Vec2::new(3,1)],
            S4::S4_6 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(1,1),Vec2::new(1,2), Vec2::new(1,3)],
            S4::S4_7 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(2,0),Vec2::new(3,0), Vec2::new(0,1)],
            S4::S4_8 => vec![Vec2::new(0,0),Vec2::new(0,1),Vec2::new(0,2),Vec2::new(0,3), Vec2::new(1,3)],
        }
    }

    fn shuffle(&self) -> Shape {
        match self {
            S4::S4_1 => Shape::S4(S4::S4_2),
            S4::S4_2 => Shape::S4(S4::S4_3),
            S4::S4_3 => Shape::S4(S4::S4_4),
            S4::S4_4 => Shape::S4(S4::S4_5),
            S4::S4_5 => Shape::S4(S4::S4_6),
            S4::S4_6 => Shape::S4(S4::S4_7),
            S4::S4_7 => Shape::S4(S4::S4_8),
            S4::S4_8 => Shape::S4(S4::S4_1),
        }
    }

    fn rotate(&self) -> Shape {
        match self {
            S4::S4_1 => Shape::S4(S4::S4_2),
            S4::S4_2 => Shape::S4(S4::S4_3),
            S4::S4_3 => Shape::S4(S4::S4_4),
            S4::S4_4 => Shape::S4(S4::S4_1),
            S4::S4_5 => Shape::S4(S4::S4_8),
            S4::S4_6 => Shape::S4(S4::S4_5),
            S4::S4_7 => Shape::S4(S4::S4_6),
            S4::S4_8 => Shape::S4(S4::S4_7),
        }
    }
}