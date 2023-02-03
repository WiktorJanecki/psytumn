use glam::{Vec2, IVec2};

pub struct Camera{
    pub position: Vec2,
}

impl Camera{
    pub fn new() -> Self{
        Self{
            position: Vec2::ZERO,
        }
    }
    pub fn x(self: &Self) -> i32{
        -self.position.x as i32
    }
    pub fn y(self: &Self) -> i32{
        -self.position.y as i32
    }
}

pub struct Tile{
    pub filename: &'static str,
}

pub struct Tilemap{
    pub values: Vec<Vec<Option<Tile>>>,
    pub tile_width: u32,
    pub tile_height: u32,
    tilemap_width: u32,
    tilemap_height: u32,
}

impl Tilemap{
    pub fn new(width: u32, height: u32, tile_width: u32, tile_height: u32) -> Self{
        let mut values = vec![];
        for _x in 0..width{
            let mut row = vec![];
            for _y in 0..height{
                row.push(None);
            }
            values.push(row);
        }
        Self { values, tile_width, tile_height, tilemap_height:height*tile_height , tilemap_width: width * tile_width }
    }
    pub fn _get(self: &Self, x: usize, y: usize) -> Option<&Tile>{
        return self.values.get(x).unwrap().get(y).unwrap().as_ref();
    }

    pub fn set(self: &mut Self, x: usize, y: usize, value: Option<Tile>) {
        self.values.get_mut(x).unwrap().insert(y, value);
    }
    pub fn position(self: &Self) -> IVec2{
        return IVec2::new(-(self.tilemap_width as i32/2), -(self.tilemap_height as i32/2) );
    }
}

