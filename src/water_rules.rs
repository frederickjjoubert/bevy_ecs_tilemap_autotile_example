#![allow(clippy::unusual_byte_groupings)]
use crate::ruleflag::RuleFlag;
use bevy_ecs_tilemap::tiles::TileTexture;

#[derive(Clone, Copy, Debug, Hash)]
pub struct WaterRule {
    rule_flag: RuleFlag,
}

impl From<&WaterRule> for TileTexture {
    fn from(water: &WaterRule) -> Self {
        // Please fix, I have not tried to actually match rules :(
        if water.rule_flag.contains(RuleFlag::N) {
            return TileTexture(96);
        }

        if water.rule_flag.contains(RuleFlag::S) {
            return TileTexture(97);
        }

        if water.rule_flag.contains(RuleFlag::E) {
            return TileTexture(98);
        }

        if water.rule_flag.contains(RuleFlag::W) {
            return TileTexture(99);
        }
        panic!("Unknown Water: {water:?}");
    }
}
