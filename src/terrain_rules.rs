use crate::dirt_rules::DirtRule;
use crate::grass_rules::GrassRule;
use crate::ruleflag::RuleFlag;
use crate::water_rules::WaterRule;
use crate::{DirtTile, GrassTile, WaterTile};
use bevy::prelude::{Entity, Query, With, Without};
use bevy_ecs_tilemap::helpers::square_grid::neighbors::{Neighbors, SQUARE_DIRECTIONS};
use bevy_ecs_tilemap::prelude::TilePos;

pub struct TerrainRule {
    grass: GrassRule,
    dirt: DirtRule,
    water: WaterRule,
}

impl PartialEq for TerrainRule {
    fn eq(&self, other: &Self) -> bool {
        self.grass == other.grass && self.dirt == other.dirt && self.water == other.water
    }
}

impl TerrainRule {
    pub fn from_neighbors(
        neighbors: &Neighbors<Entity>,
        grass_tiles_query: &Query<
            &TilePos,
            (With<GrassTile>, Without<DirtTile>, Without<WaterTile>),
        >,
        dirt_tiles_query: &Query<
            &TilePos,
            (With<DirtTile>, Without<GrassTile>, Without<WaterTile>),
        >,
        water_tiles_query: &Query<
            &TilePos,
            (With<WaterTile>, Without<GrassTile>, Without<DirtTile>),
        >,
    ) -> TerrainRule {
        let mut grass_rule_flag = RuleFlag::empty();
        let mut dirt_rule_flag = RuleFlag::empty();
        let mut water_rule_flag = RuleFlag::empty();

        for &direction in SQUARE_DIRECTIONS.iter() {
            if let Some(&entity) = neighbors.get(direction) {
                if grass_tiles_query.get(entity).is_ok() {
                    grass_rule_flag.insert(direction.into());
                }

                if dirt_tiles_query.get(entity).is_ok() {
                    dirt_rule_flag.insert(direction.into());
                }

                if water_tiles_query.get(entity).is_ok() {
                    water_rule_flag.insert(direction.into());
                }
            }
        }


        TerrainRule {
            grass: GrassRule { rule_flag: grass_rule_flag },
            dirt: DirtRule { rule_flag: dirt_rule_flag },
            water: WaterRule { rule_flag: water_rule_flag },
        }
    }
}
