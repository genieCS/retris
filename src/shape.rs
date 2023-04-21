use cursive::Vec2;
use rand::Rng;

type Positions = Vec<Vec2>;

trait ShapeTr {
    fn vectors(&self) -> Positions;

    fn rotate(&self) -> Shape;
}

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    SI(SI),
    SO(SO),
    ST(ST),
    SS(SS),
    SZ(SZ),
    SJ(SJ),
    SL(SL),
}

impl Default for Shape {
    fn default() -> Self {
        let shapes: [Shape; 7] = [Shape::SI(SI::SI_1), Shape::SO(SO::SO_1), Shape::ST(ST::ST_1), Shape::SS(SS::SS_1),  Shape::SZ(SZ::SZ_1), Shape::SJ(SJ::SJ_1), Shape::SL(SL::SL_1)];
        shapes[rand::thread_rng().gen_range(0..shapes.len())]
    }
}

impl Shape {
    pub fn vectors(&self) -> Positions {
        match self {
            Shape::SI(shape) => shape.vectors(),
            Shape::SO(shape) => shape.vectors(),
            Shape::ST(shape) => shape.vectors(),
            Shape::SS(shape) => shape.vectors(),
            Shape::SZ(shape) => shape.vectors(),
            Shape::SJ(shape) => shape.vectors(),
            Shape::SL(shape) => shape.vectors(),
        }
    }

    pub fn rotate(&self) -> Shape {
        match self {
            Shape::SI(shape) => shape.rotate(),
            Shape::SO(shape) => shape.rotate(),
            Shape::ST(shape) => shape.rotate(),
            Shape::SS(shape) => shape.rotate(),
            Shape::SZ(shape) => shape.rotate(),
            Shape::SJ(shape) => shape.rotate(),
            Shape::SL(shape) => shape.rotate(),
        }
    }
}

//    SI_1      SI_2
//  _ _ _ _      _
// |_|_|_|_| ,  |_|
//              |_|
//              |_|
//              |_|
#[derive(Debug, Clone, Copy)]
pub enum SI {
    SI_1,
    SI_2,
}

impl ShapeTr for SI {
    fn vectors(&self) -> Positions {
        match self {
            SI::SI_1 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(2,0),Vec2::new(3,0)],
            SI::SI_2 => vec![Vec2::new(0,0),Vec2::new(0,1),Vec2::new(0,2),Vec2::new(0,3)],
        }
    }

    fn rotate(&self) -> Shape {
        match self {
            SI::SI_1 => Shape::SI(SI::SI_2),
            SI::SI_2 => Shape::SI(SI::SI_1),
        }
    }
}

//  SO_1
//  _ _ 
// |_|_|
// |_|_|
#[derive(Debug, Clone, Copy)]
pub enum SO {
    SO_1,
}

impl ShapeTr for SO {
    fn vectors(&self) -> Positions {
        match self {
            SO::SO_1 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(0,1),Vec2::new(1,1)],
        }
    }

    fn rotate(&self) -> Shape {
        match self {
            SO::SO_1 => Shape::SO(SO::SO_1),
        }
    }
}


//  ST_1      ST_2     ST_3      ST_4   
//    _       _                    _
//  _|_|_    |_|_     _ _ _      _|_|
// |_|_|_|   |_|_|   |_|_|_|    |_|_|
//        ,  |_|   ,   |_|    ,   |_|
#[derive(Debug, Clone, Copy)]
pub enum ST {
    ST_1,
    ST_2,
    ST_3,
    ST_4,
}

impl ShapeTr for ST {
    fn vectors(&self) -> Positions {
        match self {
            ST::ST_1 => vec![Vec2::new(0,1),Vec2::new(1,0),Vec2::new(1,1),Vec2::new(2,1)],
            ST::ST_2 => vec![Vec2::new(0,0),Vec2::new(0,1),Vec2::new(1,1),Vec2::new(0,2)],
            ST::ST_3 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(2,0),Vec2::new(1,1)],
            ST::ST_4 => vec![Vec2::new(1,0),Vec2::new(0,1),Vec2::new(1,1),Vec2::new(1,2)],
        }
    }

    fn rotate(&self) -> Shape {
        match self {
            ST::ST_1 => Shape::ST(ST::ST_2),
            ST::ST_2 => Shape::ST(ST::ST_3),
            ST::ST_3 => Shape::ST(ST::ST_4),
            ST::ST_4 => Shape::ST(ST::ST_1),
        }
    }
}

//  SS_1     SS_2 
//  _         _ _ 
// |_|_     _|_|_|
// |_|_|   |_|_|  
//   |_|,         
#[derive(Debug, Clone, Copy)]
pub enum SS {
    SS_1,
    SS_2,
}

impl ShapeTr for SS {
    fn vectors(&self) -> Positions {
        match self {
            SS::SS_1 => vec![Vec2::new(0,0),Vec2::new(0,1),Vec2::new(1,1),Vec2::new(1,2)],
            SS::SS_2 => vec![Vec2::new(1,0),Vec2::new(2,0),Vec2::new(0,1),Vec2::new(1,1)],
        }
    }

    fn rotate(&self) -> Shape {
        match self {
            SS::SS_1 => Shape::SS(SS::SS_2),
            SS::SS_2 => Shape::SS(SS::SS_1),
        }
    }
}

//   SZ_1       SZ_2
//   _ _         _
//  |_|_|_     _|_|
//    |_|_|   |_|_|
//          , |_|
#[derive(Debug, Clone, Copy)]
pub enum SZ {
    SZ_1,
    SZ_2,
}

impl ShapeTr for SZ {
    fn vectors(&self) -> Positions {
        match self {
            SZ::SZ_1 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(1,1),Vec2::new(2,1)],
            SZ::SZ_2 => vec![Vec2::new(1,0),Vec2::new(0,1),Vec2::new(1,1),Vec2::new(0,2)],
        }
    }

    fn rotate(&self) -> Shape {
        match self {
            SZ::SZ_1 => Shape::SZ(SZ::SZ_2),
            SZ::SZ_2 => Shape::SZ(SZ::SZ_1),
        }
    }
}

//    SJ_1     SJ_2    SJ_3         SJ_4   
//  _           _ _    _ _ _ _        _    
// |_|_ _ _    |_|_|  |_|_|_|_|      |_|    
// |_|_|_|_|   |_|          |_|      |_|  
//             |_|                  _|_|    
//           , |_|   ,          ,  |_|_|    
#[derive(Debug, Clone, Copy)]
pub enum SJ {
    SJ_1,
    SJ_2,
    SJ_3,
    SJ_4,
}

impl ShapeTr for SJ {
    fn vectors(&self) -> Positions {
        match self {
            SJ::SJ_1 => vec![Vec2::new(0,0),Vec2::new(0,1),Vec2::new(1,1),Vec2::new(2,1), Vec2::new(3,1)],
            SJ::SJ_2 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(0,1),Vec2::new(0,2), Vec2::new(0,3)],
            SJ::SJ_3 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(2,0),Vec2::new(3,0), Vec2::new(3,1)],
            SJ::SJ_4 => vec![Vec2::new(1,0),Vec2::new(1,1),Vec2::new(1,2),Vec2::new(1,3), Vec2::new(0,3)],
        }
    }

    fn rotate(&self) -> Shape {
        match self {
            SJ::SJ_1 => Shape::SJ(SJ::SJ_2),
            SJ::SJ_2 => Shape::SJ(SJ::SJ_3),
            SJ::SJ_3 => Shape::SJ(SJ::SJ_4),
            SJ::SJ_4 => Shape::SJ(SJ::SJ_1),
        }
    }
}


//   SL_1      SL_2     SL_3    SL_4
//      _      _ _    _ _ _      _ 
//  _ _|_|    |_|_|  |_|_|_|    |_|
// |_|_|_|      |_|  |_|        |_|_
//          ,   |_| ,         , |_|_|
 
#[derive(Debug, Clone, Copy)]
pub enum SL {
    SL_1,
    SL_2,
    SL_3,
    SL_4,
}

impl ShapeTr for SL {
    fn vectors(&self) -> Positions {
        match self {
            SL::SL_1 => vec![Vec2::new(0,1),Vec2::new(1,1),Vec2::new(2,1),Vec2::new(2,1), Vec2::new(3,1)],
            SL::SL_2 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(0,1),Vec2::new(0,2), Vec2::new(0,3)],
            SL::SL_3 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(2,0),Vec2::new(3,0), Vec2::new(3,1)],
            SL::SL_4 => vec![Vec2::new(1,0),Vec2::new(1,1),Vec2::new(1,2),Vec2::new(1,3), Vec2::new(0,3)],
        }
    }

    fn rotate(&self) -> Shape {
        match self {
            SL::SL_1 => Shape::SL(SL::SL_4),
            SL::SL_2 => Shape::SL(SL::SL_1),
            SL::SL_3 => Shape::SL(SL::SL_2),
            SL::SL_4 => Shape::SL(SL::SL_3),
        }
    }
}
