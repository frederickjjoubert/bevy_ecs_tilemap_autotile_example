mod dirt_rules;
mod grass_rules;
mod ruleflag;
mod terrain_rules;
mod water_rules;

use crate::ruleflag::RuleFlag;
use crate::terrain_rules::TerrainRule;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::math::Vec4Swizzles;
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::window::PresentMode;
use bevy_ecs_tilemap::helpers::square_grid::neighbors::Neighbors;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use std::collections::HashMap;

pub const MAP_WIDTH: i32 = 64;
pub const MAP_HEIGHT: i32 = 64;
pub const TILE_SIZE: i32 = 16;

pub const CAMERA_MIN_ZOOM: f32 = 0.1;
pub const CAMERA_MAX_ZOOM: f32 = 2.5;
pub const CAMERA_MOVEMENT_SPEED: f32 = 10.0;
pub const CAMERA_SCROLL_SPEED: f32 = 0.1;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Bevy ECS Tilemap AutoTile Example".to_string(),
            width: 1600.0,
            height: 900.0,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_plugins(DefaultPlugins) // bevy
        .add_plugin(WorldInspectorPlugin::new()) // bevy_inspector_egui
        .add_plugin(TilemapPlugin) // bevy_ecs_tilemap
        .add_event::<UpdateTilemapEvent>()
        .add_startup_system(setup_camera)
        .add_startup_system(setup_mouse)
        .add_startup_system(setup_active_rules.label(Setup::ActiveRules))
        .add_startup_system(setup_tilemap.label(Setup::Tilemap))
        .add_startup_system(
            setup_game
                .label(Setup::Game)
                .after(Setup::Rules)
                .after(Setup::ActiveRules)
                .after(Setup::Sprites)
                .after(Setup::Tilemap),
        )
        .add_system(update_camera_movement)
        .add_system(update_camera_zoom)
        .add_system(update_selection)
        .add_system(update_mouse)
        .add_system(place_tile)
        .add_system(update_active_rules)
        .add_system(update_tilemap)
        .run();
}

// === Components ===
#[derive(Component, Debug)]
pub struct GrassTile {}

#[derive(Component, Debug)]
pub struct DirtTile {}

#[derive(Component, Debug)]
pub struct WaterTile {}

// === Events ===
pub struct UpdateTilemapEvent {}

// === Enums ===
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum Setup {
    Rules,
    ActiveRules,
    Sprites,
    Tilemap,
    Game,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tile {
    Blank,
    // Water
    Water_0,
    Water_1,
    Water_2,
    Water_3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TerrainType {
    Blank,
    Grass,
    Dirt,
    Water,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Slot {
    Empty,
    Filled { terrain_type: TerrainType },
    Any,
}

// === Structs ===
#[derive(Copy, Clone, Debug)]
pub struct Rule {
    rule_flag: RuleFlag,
}

impl Eq for Rule {}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.rule_flag == other.rule_flag
    }
}

// === Resources ===
pub struct Mouse {
    pub is_in_window: bool,
    pub window_position: Vec2,
    pub world_position: Vec3,
    pub holding_lmb: bool,
}

pub struct GameState {
    pub selection: TerrainType,
}

pub struct Sprites {
    pub sprite_lookup_table: HashMap<Tile, u32>,
}

pub struct Rules {
    pub rules: HashMap<TerrainType, Vec<(TerrainRule, Tile)>>,
}

pub struct ActiveRules {
    pub active_rules: HashMap<TilePos, TerrainRule>,
}

// === Startup Systems ===
pub fn setup_camera(mut commands: Commands) {
    let x = MAP_WIDTH as f32 / 2.0 * TILE_SIZE as f32;
    let y = MAP_HEIGHT as f32 / 2.0 * TILE_SIZE as f32;

    let position = Transform::from_xyz(x, y, 1000.0);
    commands
        .spawn_bundle(Camera2dBundle {
            transform: position,
            ..default()
        })
        .insert(OrthographicProjection { ..default() })
        .insert(Name::new("Camera"));
}

pub fn setup_mouse(mut commands: Commands) {
    commands.insert_resource(Mouse {
        is_in_window: false,
        window_position: Default::default(),
        world_position: Default::default(),
        holding_lmb: false,
    })
}

pub fn setup_active_rules(mut commands: Commands) {
    let active_rules = ActiveRules {
        active_rules: HashMap::new(),
    };
    commands.insert_resource(active_rules);
}

pub const TILE_TEXTURE_BLANK: TileTexture = TileTexture(102);

pub fn setup_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tilemap_size = TilemapSize {
        x: MAP_WIDTH as u32,
        y: MAP_HEIGHT as u32,
    };
    let tilemap_entity = commands.spawn().id(); // Need one per layer.
    let mut tile_storage = TileStorage::empty(tilemap_size); // Need one per tilemap_entity.

    // Spawn the elements of the tilemap.
    for y in 0..tilemap_size.y {
        for x in 0..tilemap_size.x {
            let tile_position = TilePos { x, y };
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_position,
                    texture: TILE_TEXTURE_BLANK,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..default()
                })
                .id();
            tile_storage.set(&tile_position, tile_entity);
        }
    }

    let grid_size = TilemapGridSize {
        x: TILE_SIZE as f32,
        y: TILE_SIZE as f32,
    };
    let tile_size = TilemapTileSize {
        x: TILE_SIZE as f32,
        y: TILE_SIZE as f32,
    };
    let image_handle: Handle<Image> = asset_server.load("sprites/all_sprites.png");
    let tilemap_texture = TilemapTexture::Single(image_handle);

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size,
            size: tilemap_size,
            storage: tile_storage,
            texture: tilemap_texture,
            map_type: TilemapType::Square,
            tile_size,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        });
}

pub fn setup_game(mut commands: Commands) {
    let game_state = GameState {
        selection: TerrainType::Grass,
    };
    commands.insert_resource(game_state);
}

// === Systems ===
pub fn place_tile(
    mut commands: Commands,
    mut update_tilemap_event_writer: EventWriter<UpdateTilemapEvent>,
    game_state: Res<GameState>,
    mouse: Res<Mouse>,
    _mouse_input: Res<Input<MouseButton>>,
    tilemap_query: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
) {
    if mouse.holding_lmb {
        // if mouse_input.just_pressed(MouseButton::Left) {
        for (map_size, grid_size, map_type, tile_storage, map_transform) in tilemap_query.iter() {
            // Grab the cursor position from the `Res<CursorPos>`
            let cursor_pos: Vec3 = mouse.world_position;
            // We need to make sure that the cursor's world position is correct relative to the map
            // due to any map transformation.
            let cursor_in_map_pos: Vec2 = {
                // Extend the cursor_pos vec3 by 1.0
                let cursor_pos = Vec4::from((cursor_pos, 1.0));
                let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
                cursor_in_map_pos.xy()
            };
            // Once we have a world position we can transform it into a possible tile position.
            if let Some(tile_position) =
            TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
            {
                if let Some(tile_entity) = tile_storage.get(&tile_position) {
                    commands.entity(tile_entity).remove::<GrassTile>();
                    commands.entity(tile_entity).remove::<DirtTile>();
                    commands.entity(tile_entity).remove::<WaterTile>();
                    match game_state.selection {
                        TerrainType::Grass => {
                            commands.entity(tile_entity).insert(GrassTile {});
                        }
                        TerrainType::Dirt => {
                            commands.entity(tile_entity).insert(DirtTile {});
                        }
                        TerrainType::Water => {
                            commands.entity(tile_entity).insert(WaterTile {});
                        }
                        _ => {
                            // Do Nothing
                        }
                    }
                    update_tilemap_event_writer.send(UpdateTilemapEvent {});
                }
            }
        }
    }
}

pub fn update_active_rules(
    mut update_tilemap_event_reader: EventReader<UpdateTilemapEvent>,
    grass_tiles_query: Query<&TilePos, (With<GrassTile>, Without<DirtTile>, Without<WaterTile>)>,
    dirt_tiles_query: Query<&TilePos, (With<DirtTile>, Without<GrassTile>, Without<WaterTile>)>,
    water_tiles_query: Query<&TilePos, (With<WaterTile>, Without<GrassTile>, Without<DirtTile>)>,
    mut tilemap_query: Query<(&TileStorage, &TilemapSize)>,
    mut active_rules: ResMut<ActiveRules>,
) {
    for _ in update_tilemap_event_reader.iter() {
        if let Ok((tile_storage, map_size)) = tilemap_query.get_single_mut() {
            // Clear Previous Active Rules
            active_rules.active_rules.clear();

            // Grass Tiles
            for tile_position in grass_tiles_query.iter() {
                let neighbors =
                    Neighbors::get_square_neighboring_positions(tile_position, map_size, true)
                        .entities(tile_storage);

                let terrain_rule = TerrainRule::from_neighbors(
                    &neighbors,
                    &grass_tiles_query,
                    &dirt_tiles_query,
                    &water_tiles_query,
                );

                active_rules
                    .active_rules
                    .insert(*tile_position, terrain_rule);
            }

            // Dirt Tiles
            for tile_position in dirt_tiles_query.iter() {
                let neighbors =
                    Neighbors::get_square_neighboring_positions(tile_position, map_size, true)
                        .entities(tile_storage);

                let terrain_rule = TerrainRule::from_neighbors(
                    &neighbors,
                    &grass_tiles_query,
                    &dirt_tiles_query,
                    &water_tiles_query,
                );

                active_rules
                    .active_rules
                    .insert(*tile_position, terrain_rule);
            }
        }
    }
}

pub fn update_tilemap(
    mut grass_tiles_query: Query<
        (&TilePos, &mut TileTexture),
        (With<GrassTile>, Without<DirtTile>, Without<WaterTile>),
    >,
    mut dirt_tiles_query: Query<
        (&TilePos, &mut TileTexture),
        (With<DirtTile>, Without<GrassTile>, Without<WaterTile>),
    >,
    mut _water_tiles_query: Query<
        (&TilePos, &mut TileTexture),
        (With<WaterTile>, Without<GrassTile>, Without<DirtTile>),
    >,
    sprites: Res<Sprites>,
    active_rules: Res<ActiveRules>,
    rules: Res<Rules>,
) {
    // Perform auto tiling based on neighbors and rules
    if active_rules.is_changed() {
        let possible_rules = &rules.rules[&TerrainType::Grass];
        for (tile_position, mut tile_texture) in grass_tiles_query.iter_mut() {
            if active_rules.active_rules.contains_key(tile_position) {
                let active_rule = &active_rules.active_rules[tile_position];
                let mut new_sprite = Tile::Blank;
                for (rule, sprite) in possible_rules.iter() {
                    if *active_rule == *rule {
                        new_sprite = sprite.clone();
                        break;
                    }
                }
                tile_texture.0 = sprites.sprite_lookup_table[&new_sprite];
            }
        }
        let possible_rules = &rules.rules[&TerrainType::Dirt];
        for (tile_position, mut tile_texture) in dirt_tiles_query.iter_mut() {
            if active_rules.active_rules.contains_key(tile_position) {
                let active_rule = &active_rules.active_rules[tile_position];
                let mut new_sprite = Tile::Blank;
                for (rule, sprite) in possible_rules.iter() {
                    if *active_rule == *rule {
                        new_sprite = sprite.clone();
                        break;
                    }
                }
                tile_texture.0 = sprites.sprite_lookup_table[&new_sprite];
            }
        }
    }
}

pub fn update_selection(keyboard: Res<Input<KeyCode>>, mut game_state: ResMut<GameState>) {
    if keyboard.just_pressed(KeyCode::Key1) {
        game_state.selection = TerrainType::Blank;
        println!("Selection Updated: {:?}", game_state.selection);
    } else if keyboard.just_pressed(KeyCode::Key2) {
        game_state.selection = TerrainType::Grass;
        println!("Selection Updated: {:?}", game_state.selection);
    } else if keyboard.just_pressed(KeyCode::Key3) {
        game_state.selection = TerrainType::Dirt;
        println!("Selection Updated: {:?}", game_state.selection);
    } else if keyboard.just_pressed(KeyCode::Key4) {
        game_state.selection = TerrainType::Water;
        println!("Selection Updated: {:?}", game_state.selection);
    }
}

pub fn update_mouse(
    mut mouse: ResMut<Mouse>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera_q: Query<(&Transform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    for cursor_moved in cursor_moved_events.iter() {
        // To get the mouse's world position, we have to transform its window position by
        // any transforms on the camera. This is done by projecting the cursor position into
        // camera space (world space).
        for (cam_t, cam) in camera_q.iter() {
            let cursor_pos = cursor_pos_in_world(&windows, cursor_moved.position, cam_t, cam);
            mouse.world_position = cursor_pos;
        }
    }
    // Left Mouse Button Held
    if mouse_input.just_pressed(MouseButton::Left) {
        mouse.holding_lmb = true;
    } else if mouse_input.just_released(MouseButton::Left) {
        mouse.holding_lmb = false;
    }
}

pub fn update_camera_movement(
    keyboard: Res<Input<KeyCode>>,
    windows: ResMut<Windows>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let mut camera_transform = camera_query.single_mut();

    // Update the camera position based on the keyboard input.
    let mut movement_delta = Vec3::new(0.0, 0.0, 0.0);
    // Up
    if keyboard.pressed(KeyCode::W) {
        movement_delta.y += 1.0;
    }
    // Down
    else if keyboard.pressed(KeyCode::S) {
        movement_delta.y -= 1.0;
    }
    // Left
    if keyboard.pressed(KeyCode::A) {
        movement_delta.x -= 1.0;
    }
    // Right
    else if keyboard.pressed(KeyCode::D) {
        movement_delta.x += 1.0;
    }

    if movement_delta != Vec3::ZERO {
        // Normalize
        movement_delta /= movement_delta.length();
        movement_delta *= CAMERA_MOVEMENT_SPEED;
    }
    camera_transform.translation += movement_delta;

    // Get the primary window.
    let window = windows.get_primary().unwrap();
    // Get the size of the window.
    let window_width = window.width();
    let window_height = window.height();

    let buffer = 4096.0;
    let min_x = 0.0 + (window_width / 2.0) - buffer;
    let min_y = 0.0 + (window_height / 2.0) - buffer;
    let max_x = (MAP_WIDTH as f32 * TILE_SIZE as f32) - (window_width / 2.0) + buffer;
    let max_y = (MAP_HEIGHT as f32 * TILE_SIZE as f32) - (window_height / 2.0) + buffer;
    // println!("min_x: {}, min_y: {}, max_x: {}, max_y: {}", min_x, min_y, max_x, max_y);

    // Bound the Camera Movement
    camera_transform.translation.x = max_x.min(min_x.max(camera_transform.translation.x));
    camera_transform.translation.y = max_y.min(min_y.max(camera_transform.translation.y));

    // println!("Camera Position: {:?}", camera_transform.translation);
}

pub fn update_camera_zoom(
    mut scroll_events: EventReader<MouseWheel>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera2d>>,
) {
    for event in scroll_events.iter() {
        for mut orthographic_projection in camera_query.iter_mut() {
            let scroll_sensitivity: f32;
            match event.unit {
                MouseScrollUnit::Line => {
                    // Mice
                    scroll_sensitivity = 1.0;
                }
                MouseScrollUnit::Pixel => {
                    // Track Pads
                    scroll_sensitivity = 1.0;
                }
            }
            let mut log_scale = orthographic_projection.scale.ln();

            // Scroll Direction
            log_scale -= event.y * CAMERA_SCROLL_SPEED * scroll_sensitivity;

            let new_scale = log_scale.exp();

            if new_scale > CAMERA_MAX_ZOOM {
                orthographic_projection.scale = CAMERA_MAX_ZOOM;
            } else if new_scale < CAMERA_MIN_ZOOM {
                orthographic_projection.scale = CAMERA_MIN_ZOOM;
            } else {
                orthographic_projection.scale = new_scale;
            }
        }
    }
}

// === Helper Functions ===
pub fn world_position_to_index(position: Vec2) -> (i32, i32) {
    let x_index = position.x / TILE_SIZE as f32;
    let y_index = position.y / TILE_SIZE as f32;
    (x_index as i32, y_index as i32)
}

// Converts the cursor position into a world position, taking into account any transforms applied
// the camera.
pub fn cursor_pos_in_world(
    windows: &Windows,
    cursor_pos: Vec2,
    cam_t: &Transform,
    cam: &Camera,
) -> Vec3 {
    let window = windows.primary();

    let window_size = Vec2::new(window.width(), window.height());

    // Convert screen position [0..resolution] to ndc [-1..1]
    // (ndc = normalized device coordinates)
    let ndc_to_world = cam_t.compute_matrix() * cam.projection_matrix().inverse();
    let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;
    ndc_to_world.project_point3(ndc.extend(0.0))
}
