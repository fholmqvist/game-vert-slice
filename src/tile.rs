#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Ground01 = 0,
    Ground02 = 1,
    Ground03 = 2,
    WallTop01 = 16,
    WallTop02 = 17,
    WallTop03 = 18,
    WallTop04 = 19,
    WallTop05 = 20,
    WallSide01 = 32,
    WallSide02 = 33,
    WallSide03 = 34,
    WallSide04 = 35,
    WallSide05 = 36,
    Red = 96,
}

impl Tile {
    pub fn is_wall(self) -> bool {
        matches!(
            self,
            Tile::WallTop01
                | Tile::WallTop02
                | Tile::WallTop03
                | Tile::WallTop04
                | Tile::WallTop05
                | Tile::WallSide01
                | Tile::WallSide02
                | Tile::WallSide03
                | Tile::WallSide04
                | Tile::WallSide05
        )
    }

    pub fn is_ground(self) -> bool {
        matches!(self, Tile::Ground01 | Tile::Ground02 | Tile::Ground03)
    }

    pub fn is_blocked(self) -> bool {
        self.is_wall()
    }
}
