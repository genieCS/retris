use cursive::{
    Vec2,
    theme::{Color, ColorStyle},
};
use rand::Rng;

type Positions = Vec<Vec2>;

trait BlockTr {
    fn vectors(&self) -> Positions;

    fn rotate(&self) -> Block;

    fn color(&self) -> ColorStyle;
}

#[derive(Debug, Clone, Copy)]
pub enum Block {
    SI(SI),
    SO(SO),
    ST(ST),
    SS(SS),
    SZ(SZ),
    SJ(SJ),
    SL(SL),
}

impl Default for Block {
    fn default() -> Self {
        let blocks: [Block; 7] = [
            Block::SI(SI::SI_1),
            Block::SO(SO::SO_1),
            Block::ST(ST::ST_1),
            Block::SS(SS::SS_1),
            Block::SZ(SZ::SZ_1),
            Block::SJ(SJ::SJ_1),
            Block::SL(SL::SL_1),
            ];
        blocks[rand::thread_rng().gen_range(0..blocks.len())]
    }
}

impl Block {
    pub fn vectors(&self) -> Positions {
        match self {
            Block::SI(block) => block.vectors(),
            Block::SO(block) => block.vectors(),
            Block::ST(block) => block.vectors(),
            Block::SS(block) => block.vectors(),
            Block::SZ(block) => block.vectors(),
            Block::SJ(block) => block.vectors(),
            Block::SL(block) => block.vectors(),
        }
    }

    pub fn rotate(&self) -> Block {
        match self {
            Block::SI(block) => block.rotate(),
            Block::SO(block) => block.rotate(),
            Block::ST(block) => block.rotate(),
            Block::SS(block) => block.rotate(),
            Block::SZ(block) => block.rotate(),
            Block::SJ(block) => block.rotate(),
            Block::SL(block) => block.rotate(),
        }
    }

    pub fn color(&self) -> ColorStyle {
        match self {
            Block::SI(block) => block.color(),
            Block::SO(block) => block.color(),
            Block::ST(block) => block.color(),
            Block::SS(block) => block.color(),
            Block::SZ(block) => block.color(),
            Block::SJ(block) => block.color(),
            Block::SL(block) => block.color(),
        }
    }
}

impl IntoIterator for Block {
    type Item = Vec2;
    type IntoIter = std::vec::IntoIter<Vec2>;

    fn into_iter(self) -> Self::IntoIter {
        self.vectors().into_iter()
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

impl BlockTr for SI {
    fn vectors(&self) -> Positions {
        match self {
            SI::SI_1 => vec![Vec2::new(0,0),Vec2::new(0,1),Vec2::new(0,2),Vec2::new(0,3)],
            SI::SI_2 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(2,0),Vec2::new(3,0)],
        }
    }

    fn rotate(&self) -> Block {
        match self {
            SI::SI_1 => Block::SI(SI::SI_2),
            SI::SI_2 => Block::SI(SI::SI_1),
        }
    }

    fn color(&self) -> ColorStyle {
        ColorStyle::new(Color::Rgb(0,0,0), Color::Rgb(0,255,0)) // Green
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

impl BlockTr for SO {
    fn vectors(&self) -> Positions {
        match self {
            SO::SO_1 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(0,1),Vec2::new(1,1)],
        }
    }

    fn rotate(&self) -> Block {
        match self {
            SO::SO_1 => Block::SO(SO::SO_1),
        }
    }

    fn color(&self) -> ColorStyle {
        ColorStyle::new(Color::Rgb(0,0,0), Color::Rgb(255,255,0)) // Orange
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

impl BlockTr for ST {
    fn vectors(&self) -> Positions {
        match self {
            ST::ST_1 => vec![Vec2::new(0,1),Vec2::new(1,0),Vec2::new(1,1),Vec2::new(2,1)],
            ST::ST_2 => vec![Vec2::new(0,0),Vec2::new(0,1),Vec2::new(1,1),Vec2::new(0,2)],
            ST::ST_3 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(2,0),Vec2::new(1,1)],
            ST::ST_4 => vec![Vec2::new(1,0),Vec2::new(0,1),Vec2::new(1,1),Vec2::new(1,2)],
        }
    }

    fn rotate(&self) -> Block {
        match self {
            ST::ST_1 => Block::ST(ST::ST_2),
            ST::ST_2 => Block::ST(ST::ST_3),
            ST::ST_3 => Block::ST(ST::ST_4),
            ST::ST_4 => Block::ST(ST::ST_1),
        }
    }

    fn color(&self) -> ColorStyle {
        ColorStyle::new(Color::Rgb(0,0,0), Color::Rgb(0,0,255)) // Blue
    }
}

//   SS_1       SS_2
//    _ _       _
//  _|_|_|     |_|_ 
// |_|_|       |_|_|  
//         ,     |_|
#[derive(Debug, Clone, Copy)]
pub enum SS {
    SS_1,
    SS_2,
}

impl BlockTr for SS {
    fn vectors(&self) -> Positions {
        match self {
            SS::SS_1 => vec![Vec2::new(1,0),Vec2::new(2,0),Vec2::new(0,1),Vec2::new(1,1)],
            SS::SS_2 => vec![Vec2::new(0,0),Vec2::new(0,1),Vec2::new(1,1),Vec2::new(1,2)],
        }
    }

    fn rotate(&self) -> Block {
        match self {
            SS::SS_1 => Block::SS(SS::SS_2),
            SS::SS_2 => Block::SS(SS::SS_1),
        }
    }

    fn color(&self) -> ColorStyle {
        ColorStyle::new(Color::Rgb(0,0,0), Color::Rgb(255,0,255)) // Magenta
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

impl BlockTr for SZ {
    fn vectors(&self) -> Positions {
        match self {
            SZ::SZ_1 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(1,1),Vec2::new(2,1)],
            SZ::SZ_2 => vec![Vec2::new(1,0),Vec2::new(0,1),Vec2::new(1,1),Vec2::new(0,2)],
        }
    }

    fn rotate(&self) -> Block {
        match self {
            SZ::SZ_1 => Block::SZ(SZ::SZ_2),
            SZ::SZ_2 => Block::SZ(SZ::SZ_1),
        }
    }

    fn color(&self) -> ColorStyle {
        ColorStyle::new(Color::Rgb(0,0,0), Color::Rgb(0,255,255)) // Cyan
    }
}

//    SJ_1     SJ_2    SJ_3    SJ_4   
//  _         _ _    _ _ _        _    
// |_|_ _    |_|_|  |_|_|_|      |_|    
// |_|_|_|   |_|        |_|     _|_|  
//         , |_|   ,        ,  |_|_|    

#[derive(Debug, Clone, Copy)]
pub enum SJ {
    SJ_1,
    SJ_2,
    SJ_3,
    SJ_4,
}

impl BlockTr for SJ {
    fn vectors(&self) -> Positions {
        match self {
            SJ::SJ_1 => vec![Vec2::new(0,0),Vec2::new(0,1),Vec2::new(1,1),Vec2::new(2,1)],
            SJ::SJ_2 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(0,1),Vec2::new(0,2)],
            SJ::SJ_3 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(2,0),Vec2::new(2,1)],
            SJ::SJ_4 => vec![Vec2::new(1,0),Vec2::new(1,1),Vec2::new(1,2),Vec2::new(0,2)],
        }
    }

    fn rotate(&self) -> Block {
        match self {
            SJ::SJ_1 => Block::SJ(SJ::SJ_2),
            SJ::SJ_2 => Block::SJ(SJ::SJ_3),
            SJ::SJ_3 => Block::SJ(SJ::SJ_4),
            SJ::SJ_4 => Block::SJ(SJ::SJ_1),
        }
    }

    fn color(&self) -> ColorStyle {
        ColorStyle::new(Color::Rgb(0,0,0), Color::Rgb(0,128,128)) // Teal
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

impl BlockTr for SL {
    fn vectors(&self) -> Positions {
        match self {
            SL::SL_1 => vec![Vec2::new(0,1),Vec2::new(1,1),Vec2::new(2,1), Vec2::new(2,0)],
            SL::SL_2 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(1,1),Vec2::new(1,2)],
            SL::SL_3 => vec![Vec2::new(0,0),Vec2::new(1,0),Vec2::new(2,0), Vec2::new(0,1)],
            SL::SL_4 => vec![Vec2::new(0,0),Vec2::new(0,1),Vec2::new(0,2),Vec2::new(1,2)],
        }
    }

    fn rotate(&self) -> Block {
        match self {
            SL::SL_1 => Block::SL(SL::SL_4),
            SL::SL_2 => Block::SL(SL::SL_1),
            SL::SL_3 => Block::SL(SL::SL_2),
            SL::SL_4 => Block::SL(SL::SL_3),
        }
    }

    fn color(&self) -> ColorStyle {
        ColorStyle::new(Color::Rgb(0,0,0), Color::Rgb(128,0,128)) // Purple
    }
}
