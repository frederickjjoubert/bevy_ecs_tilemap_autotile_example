use crate::ruleflag::RuleFlags;
use crate::{DirtTile, GrassTile, WaterTile};
use bevy::prelude::{Entity, Query, With, Without};
use bevy_ecs_tilemap::helpers::square_grid::neighbors::{Neighbors, SQUARE_DIRECTIONS};
use bevy_ecs_tilemap::prelude::{TilePos, TileTexture};

#[derive(Debug)]
pub struct TerrainRule {
    pub grass: RuleFlags,
    pub dirt: RuleFlags,
    pub water: RuleFlags,
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
        let mut grass = RuleFlags::empty();
        let mut dirt = RuleFlags::empty();
        let mut water = RuleFlags::empty();

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

        let terrain_rule = TerrainRule { grass, dirt, water };
        // println!("debug TerrainRule: {:?}", terrain_rule);

        return terrain_rule;
    }

    pub fn into_tile_texture(
        &self,
        grass_offset: u32,
        dirt_offset: u32,
        water_offset: u32,
    ) -> TileTexture {
        match (
            !self.grass.is_empty(),
            !self.dirt.is_empty(),
            !self.water.is_empty(),
        ) {
            (true, false, false) => {}
            (false, true, false) => {}
            (false, false, true) => {}
            (_, _, _) => panic!("More than one of grass/dirt/water is non-empty: {}"),
        }
    }
}
