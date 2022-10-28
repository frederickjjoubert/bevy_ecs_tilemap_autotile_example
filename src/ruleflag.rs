#![allow(clippy::unusual_byte_groupings)]

use bevy_ecs_tilemap::helpers::square_grid::neighbors::RectangularDirection;
use bevy_ecs_tilemap::tiles::TileTexture;
use bitflags::bitflags;

bitflags! {
    pub struct RuleFlags: u32 {
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

impl RuleFlags {
    #[inline]
    pub fn does_not_contain(&self, cannot_contain: RuleFlags) -> bool {
        (*self & cannot_contain).is_empty()
    }
}

impl From<RectangularDirection> for RuleFlags {
    fn from(direction: RectangularDirection) -> Self {
        match direction {
            RectangularDirection::East => RuleFlags::E,
            RectangularDirection::NorthEast => RuleFlags::NE,
            RectangularDirection::North => RuleFlags::N,
            RectangularDirection::NorthWest => RuleFlags::NW,
            RectangularDirection::West => RuleFlags::W,
            RectangularDirection::SouthWest => RuleFlags::SW,
            RectangularDirection::South => RuleFlags::S,
            RectangularDirection::SouthEast => RuleFlags::W,
        }
    }
}

/// A blob index in the style of Godot.
///
/// For more information, see [Godot Autotile docs](https://docs.godotengine.org/en/stable/tutorials/2d/using_tilemaps.html#autotiles)
/// and search for "Template Generic", which is pictured as: https://docs.godotengine.org/en/stable/_images/autotile_template_3x3_minimal.png.
#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct BlobIndex(u8);

impl From<RuleFlags> for Option<BlobIndex> {
    fn from(value: RuleFlags) -> Option<BlobIndex> {
        let ix = match value.bits {
            0b_000_010_010 => Some(0),
            0b_000_011_010 => Some(1),
            0b_000_111_010 => Some(2),
            0b_000_110_010 => Some(3),
            0b_110_111_010 => Some(4),
            0b_000_111_011 => Some(5),
            0b_000_111_110 => Some(6),
            0b_011_111_010 => Some(7),
            0b_000_011_011 => Some(8),
            0b_010_111_111 => Some(9),
            0b_000_111_111 => Some(10),
            0b_000_110_110 => Some(11),
            0b_010_010_010 => Some(12),
            0b_010_011_010 => Some(13),
            0b_010_111_010 => Some(14),
            0b_010_110_010 => Some(15),
            0b_010_011_011 => Some(16),
            0b_011_111_111 => Some(17),
            0b_110_111_111 => Some(18),
            0b_010_110_110 => Some(19),
            0b_011_011_011 => Some(20),
            0b_011_111_110 => Some(21),
            0b_000_000_000 => Some(22),
            0b_110_111_110 => Some(23),
            0b_010_010_000 => Some(24),
            0b_010_011_000 => Some(25),
            0b_010_111_000 => Some(26),
            0b_010_110_000 => Some(27),
            0b_011_011_010 => Some(28),
            0b_111_111_011 => Some(29),
            0b_111_111_110 => Some(30),
            0b_110_110_010 => Some(31),
            0b_011_111_011 => Some(32),
            0b_111_111_111 => Some(33),
            0b_110_111_011 => Some(34),
            0b_110_110_110 => Some(35),
            0b_000_010_000 => Some(36),
            0b_000_011_000 => Some(37),
            0b_000_111_000 => Some(38),
            0b_000_110_000 => Some(39),
            0b_010_111_110 => Some(40),
            0b_011_111_000 => Some(41),
            0b_110_111_000 => Some(42),
            0b_010_111_011 => Some(43),
            0b_011_011_000 => Some(44),
            0b_111_111_000 => Some(45),
            0b_111_111_010 => Some(46),
            0b_110_110_000 => Some(47),
            _ => None,
        };
        ix.map(BlobIndex)
    }
}

impl Into<TileTexture> for BlobIndex {
    fn into(self) -> TileTexture {
        TileTexture(self.0 as u32)
    }
}
