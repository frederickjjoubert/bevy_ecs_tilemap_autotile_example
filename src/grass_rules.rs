#![allow(clippy::unusual_byte_groupings)]

use crate::ruleflag::RuleFlag;
use bevy_ecs_tilemap::tiles::TileTexture;

#[derive(Clone, Copy, Debug, Hash)]
pub struct GrassRule {
    pub(crate) rule_flag: RuleFlag,
}

impl PartialEq for GrassRule {
    fn eq(&self, other: &Self) -> bool {
        self.rule_flag == other.rule_flag
    }
}

impl From<&GrassRule> for TileTexture {
    fn from(grass: &GrassRule) -> Self {
        // Ideally, we would have an algorithm to convert from rule flag to texture index, but this
        // requires having control over the order in which TileTextures are supplied (i.e. either
        // using a `TextureVec` or `TextureContainer`

        // This is one example of how to test the rules
        if grass.rule_flag.contains(RuleFlag::C | RuleFlag::S)
            && grass
            .rule_flag
            .does_not_contain(RuleFlag::N | RuleFlag::W | RuleFlag::SE)
        {
            return TileTexture(0);
        }

        // Another example showing a different way in which rule flags can be supplied, in this case
        // from binary value
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_111_010).unwrap())
            && grass
            .rule_flag
            .does_not_contain(RuleFlag::from_bits(0b_000_111_010).unwrap())
        {
            return TileTexture(1);
        }

        // Now I will be lazy and use multi-cursor magic
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_111_010).unwrap())
        {
            return TileTexture(2);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_110_010).unwrap())
        {
            return TileTexture(3);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_110_111_010).unwrap())
        {
            return TileTexture(4);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_111_011).unwrap())
        {
            return TileTexture(5);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_111_110).unwrap())
        {
            return TileTexture(6);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_011_111_010).unwrap())
        {
            return TileTexture(7);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_011_011).unwrap())
        {
            return TileTexture(8);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_111_111).unwrap())
        {
            return TileTexture(9);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_111_111).unwrap())
        {
            return TileTexture(10);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_110_110).unwrap())
        {
            return TileTexture(11);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_010_010).unwrap())
        {
            return TileTexture(12);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_011_010).unwrap())
        {
            return TileTexture(13);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_111_010).unwrap())
        {
            return TileTexture(14);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_110_010).unwrap())
        {
            return TileTexture(15);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_011_011).unwrap())
        {
            return TileTexture(16);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_011_111_111).unwrap())
        {
            return TileTexture(17);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_110_111_111).unwrap())
        {
            return TileTexture(18);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_110_110).unwrap())
        {
            return TileTexture(19);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_011_011_011).unwrap())
        {
            return TileTexture(20);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_011_111_110).unwrap())
        {
            return TileTexture(21);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_000_000).unwrap())
        {
            return TileTexture(22);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_110_111_110).unwrap())
        {
            return TileTexture(23);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_010_000).unwrap())
        {
            return TileTexture(24);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_011_000).unwrap())
        {
            return TileTexture(25);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_111_000).unwrap())
        {
            return TileTexture(26);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_110_000).unwrap())
        {
            return TileTexture(27);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_011_011_010).unwrap())
        {
            return TileTexture(28);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_111_111_011).unwrap())
        {
            return TileTexture(29);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_111_111_110).unwrap())
        {
            return TileTexture(30);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_110_110_010).unwrap())
        {
            return TileTexture(31);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_011_111_011).unwrap())
        {
            return TileTexture(32);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_111_111_111).unwrap())
        {
            return TileTexture(33);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_110_111_011).unwrap())
        {
            return TileTexture(34);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_110_110_110).unwrap())
        {
            return TileTexture(35);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_010_000).unwrap())
        {
            return TileTexture(36);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_011_000).unwrap())
        {
            return TileTexture(37);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_111_000).unwrap())
        {
            return TileTexture(38);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_110_000).unwrap())
        {
            return TileTexture(39);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_111_110).unwrap())
        {
            return TileTexture(40);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_011_111_000).unwrap())
        {
            return TileTexture(41);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_110_111_000).unwrap())
        {
            return TileTexture(42);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_111_011).unwrap())
        {
            return TileTexture(43);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_011_011_000).unwrap())
        {
            return TileTexture(44);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_111_111_000).unwrap())
        {
            return TileTexture(45);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_111_111_010).unwrap())
        {
            return TileTexture(46);
        }
        if grass
            .rule_flag
            .contains(RuleFlag::from_bits(0b_110_110_000).unwrap())
        {
            return TileTexture(47);
        }

        panic!("Unknown Grass: {grass:?}");
    }
}
