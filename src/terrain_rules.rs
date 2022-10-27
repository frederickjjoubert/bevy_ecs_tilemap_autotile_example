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
        let mut grass = RuleFlag::empty();
        let mut dirt = RuleFlag::empty();
        let mut water = RuleFlag::empty();

        for &direction in SQUARE_DIRECTIONS.iter() {
            if let Some(&entity) = neighbors.get(direction) {
                if grass_tiles_query.get(entity).is_ok() {
                    grass.insert(direction.into());
                }

                if dirt_tiles_query.get(entity).is_ok() {
                    dirt.insert(direction.into());
                }

                if water_tiles_query.get(entity).is_ok() {
                    water.insert(direction.into());
                }
            }
        }

        TerrainRule { grass, dirt, water }
    }
}
