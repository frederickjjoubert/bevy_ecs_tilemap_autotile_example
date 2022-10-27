use bevy_ecs_tilemap::helpers::square_grid::neighbors::SquareDirection;
use bitflags::bitflags;

bitflags! {
    pub struct RuleFlag: u32 {
        const NW = 256; // equivalent to 0b_100_000_000;
        const N = 128; // equivalent to 0b_010_000_000;
        const NE = 64; // equivalent to 0b_001_000_000;
        const W = 32; // equivalent to 0b_000_100_000;
        const C = 16; // equivalent to 0b_000_010_000;
        const E = 8; // equivalent to 0b_000_001_000;
        const SW = 4; // equivalent to 0b_000_000_100;
        const S = 2; // equivalent to 0b_000_000_010;
        const SE = 1; // equivalent to 0b_000_000_001;
    }
}

impl RuleFlag {
    #[inline]
    pub fn does_not_contain(&self, cannot_contain: RuleFlag) -> bool {
        (*self & cannot_contain).is_empty()
    }
}

impl From<SquareDirection> for RuleFlag {
    fn from(direction: SquareDirection) -> Self {
        match direction {
            SquareDirection::East => RuleFlag::E,
            SquareDirection::NorthEast => RuleFlag::NE,
            SquareDirection::North => RuleFlag::N,
            SquareDirection::NorthWest => RuleFlag::NW,
            SquareDirection::West => RuleFlag::W,
            SquareDirection::SouthWest => RuleFlag::SW,
            SquareDirection::South => RuleFlag::S,
            SquareDirection::SouthEast => RuleFlag::W,
        }
    }
}
