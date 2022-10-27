#![allow(clippy::unusual_byte_groupings)]

use crate::ruleflag::RuleFlag;
use bevy::prelude::Entity;
use bevy_ecs_tilemap::helpers::square_grid::neighbors::{Neighbors, SQUARE_DIRECTIONS};
use bevy_ecs_tilemap::tiles::TileTexture;

#[derive(Clone, Copy, Debug, Hash)]
pub struct DirtRule {
    rule_flag: RuleFlag,
}

impl From<&DirtRule> for TileTexture {
    fn from(dirt: &DirtRule) -> Self {
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_010_010).unwrap())
        {
            return TileTexture(48);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_011_010).unwrap())
        {
            return TileTexture(49);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_111_010).unwrap())
        {
            return TileTexture(50);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_110_010).unwrap())
        {
            return TileTexture(51);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_110_111_010).unwrap())
        {
            return TileTexture(52);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_111_011).unwrap())
        {
            return TileTexture(53);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_111_110).unwrap())
        {
            return TileTexture(54);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_011_111_010).unwrap())
        {
            return TileTexture(55);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_011_011).unwrap())
        {
            return TileTexture(56);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_111_111).unwrap())
        {
            return TileTexture(57);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_111_111).unwrap())
        {
            return TileTexture(58);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_110_110).unwrap())
        {
            return TileTexture(59);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_010_010).unwrap())
        {
            return TileTexture(60);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_011_010).unwrap())
        {
            return TileTexture(61);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_111_010).unwrap())
        {
            return TileTexture(62);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_110_010).unwrap())
        {
            return TileTexture(63);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_011_011).unwrap())
        {
            return TileTexture(64);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_011_111_111).unwrap())
        {
            return TileTexture(65);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_110_111_111).unwrap())
        {
            return TileTexture(66);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_110_110).unwrap())
        {
            return TileTexture(67);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_011_011_011).unwrap())
        {
            return TileTexture(68);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_011_111_110).unwrap())
        {
            return TileTexture(69);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_000_000).unwrap())
        {
            return TileTexture(70);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_110_111_110).unwrap())
        {
            return TileTexture(71);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_010_000).unwrap())
        {
            return TileTexture(72);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_011_000).unwrap())
        {
            return TileTexture(73);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_111_000).unwrap())
        {
            return TileTexture(74);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_110_000).unwrap())
        {
            return TileTexture(75);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_011_011_010).unwrap())
        {
            return TileTexture(76);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_111_111_011).unwrap())
        {
            return TileTexture(77);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_111_111_110).unwrap())
        {
            return TileTexture(78);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_110_110_010).unwrap())
        {
            return TileTexture(79);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_011_111_011).unwrap())
        {
            return TileTexture(80);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_111_111_111).unwrap())
        {
            return TileTexture(81);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_110_111_011).unwrap())
        {
            return TileTexture(82);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_110_110_110).unwrap())
        {
            return TileTexture(83);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_010_000).unwrap())
        {
            return TileTexture(84);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_011_000).unwrap())
        {
            return TileTexture(85);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_111_000).unwrap())
        {
            return TileTexture(86);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_000_110_000).unwrap())
        {
            return TileTexture(87);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_111_110).unwrap())
        {
            return TileTexture(88);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_011_111_000).unwrap())
        {
            return TileTexture(89);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_110_111_000).unwrap())
        {
            return TileTexture(90);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_010_111_011).unwrap())
        {
            return TileTexture(91);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_011_011_000).unwrap())
        {
            return TileTexture(92);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_111_111_000).unwrap())
        {
            return TileTexture(93);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_111_111_010).unwrap())
        {
            return TileTexture(94);
        }
        if dirt
            .rule_flag
            .contains(RuleFlag::from_bits(0b_110_110_000).unwrap())
        {
            return TileTexture(95);
        }

        panic!("Unknown Dirt: {dirt:?}");
    }
}
