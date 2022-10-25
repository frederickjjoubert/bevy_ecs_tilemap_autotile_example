#![warn(clippy::all, clippy::pedantic)]

use bevy::math::Vec4Swizzles;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::texture::ImageSettings;
use bevy::window::PresentMode;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use std::collections::HashMap;

pub const MAP_WIDTH: i32 = 64;
pub const MAP_HEIGHT: i32 = 64;
pub const TILE_SIZE: i32 = 16;

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
        .add_startup_system(setup_sprites.label(Setup::Sprites))
        .add_startup_system(setup_tilemap.label(Setup::Tilemap))
        .add_startup_system(
            setup_game
                .label(Setup::Game)
                .after(Setup::Sprites)
                .after(Setup::Tilemap),
        )
        .add_system(update_selection)
        .add_system(update_mouse)
        .add_system(place_tile)
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
    Sprites,
    Tilemap,
    Game,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Sprite {
    Blank,
    Grass,
    Dirt,
    Water,
}

#[derive(Debug)]
pub enum Selection {
    Blank,
    Grass,
    Dirt,
    Water,
}

// === Resources ===
pub struct Mouse {
    pub is_in_window: bool,
    pub window_position: Vec2,
    pub world_position: Vec3,
    pub holding_lmb: bool,
}

pub struct GameState {
    pub selection: Selection,
}

pub struct Sprites {
    pub sprite_lookup_table: HashMap<Sprite, u32>,
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

pub fn setup_sprites(mut commands: Commands) {
    // TODO: Populate this for all sprites, first need to combine all sprites into one big file (Pain Point).
    let sprites = Sprites {
        sprite_lookup_table: HashMap::from([
            (Sprite::Blank, 0),
            (Sprite::Grass, 1),
            (Sprite::Dirt, 2),
            (Sprite::Water, 3),
        ]),
    };

    commands.insert_resource(sprites);
}

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
                    texture: TileTexture(0),
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
    let image_handle: Handle<Image> = asset_server.load("sprites/basic_sprites.png");
    // let image_handle: Handle<Image> = asset_server.load(
    //     "sprites/Sprout Lands - Sprites - Basic pack/Tilesets/ground tiles/old tiles/Grass.png",
    // );
    let tilemap_texture = TilemapTexture::Single(image_handle);

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size,
            size: tilemap_size,
            storage: tile_storage,
            texture: tilemap_texture,
            tile_size,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        });
}

pub fn setup_game(mut commands: Commands) {
    let game_state = GameState {
        selection: Selection::Grass,
    };
    commands.insert_resource(game_state);
}

// === Systems ===
pub fn place_tile(
    mut commands: Commands,
    mut update_tilemap_event_writer: EventWriter<UpdateTilemapEvent>,
    game_state: Res<GameState>,
    mouse: Res<Mouse>,
    tilemap_query: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
) {
    if mouse.holding_lmb {
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
                // My code
                if let Some(tile_entity) = tile_storage.get(&tile_position) {
                    commands.entity(tile_entity).remove::<GrassTile>();
                    commands.entity(tile_entity).remove::<DirtTile>();
                    commands.entity(tile_entity).remove::<WaterTile>();
                    match game_state.selection {
                        Selection::Grass => {
                            commands.entity(tile_entity).insert(GrassTile {});
                        }
                        Selection::Dirt => {
                            commands.entity(tile_entity).insert(DirtTile {});
                        }
                        Selection::Water => {
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

pub fn update_tilemap(
    mut update_tilemap_event_reader: EventReader<UpdateTilemapEvent>,
    mut blank_tiles_query: Query<
        (Entity, &TilePos, &mut TileTexture),
        (Without<GrassTile>, Without<DirtTile>, Without<WaterTile>),
    >,
    mut grass_tiles_query: Query<
        (Entity, &TilePos, &mut TileTexture),
        (With<GrassTile>, Without<DirtTile>, Without<WaterTile>),
    >,
    mut dirt_tiles_query: Query<
        (Entity, &TilePos, &mut TileTexture),
        (With<DirtTile>, Without<GrassTile>, Without<WaterTile>),
    >,
    mut water_tiles_query: Query<
        (Entity, &TilePos, &mut TileTexture),
        (With<WaterTile>, Without<GrassTile>, Without<DirtTile>),
    >,
    sprites: Res<Sprites>,
) {
    // Perform auto tiling based on neighbors and rules
    for _ in update_tilemap_event_reader.iter() {
        for (entity, tile_position, mut tile_texture_index) in blank_tiles_query.iter_mut() {
            tile_texture_index.0 = sprites.sprite_lookup_table[&Sprite::Blank];
        }
        for (entity, tile_position, mut tile_texture_index) in grass_tiles_query.iter_mut() {
            tile_texture_index.0 = sprites.sprite_lookup_table[&Sprite::Grass];
        }
        for (entity, tile_position, mut tile_texture_index) in dirt_tiles_query.iter_mut() {
            tile_texture_index.0 = sprites.sprite_lookup_table[&Sprite::Dirt];
        }
        for (entity, tile_position, mut tile_texture_index) in water_tiles_query.iter_mut() {
            tile_texture_index.0 = sprites.sprite_lookup_table[&Sprite::Water];
        }
    }
}

pub fn update_selection(keyboard: Res<Input<KeyCode>>, mut game_state: ResMut<GameState>) {
    if keyboard.just_pressed(KeyCode::Key1) {
        game_state.selection = Selection::Blank;
        println!("Selection Updated: {:?}", game_state.selection);
    } else if keyboard.just_pressed(KeyCode::Key2) {
        game_state.selection = Selection::Grass;
        println!("Selection Updated: {:?}", game_state.selection);
    } else if keyboard.just_pressed(KeyCode::Key3) {
        game_state.selection = Selection::Dirt;
        println!("Selection Updated: {:?}", game_state.selection);
    } else if keyboard.just_pressed(KeyCode::Key4) {
        game_state.selection = Selection::Water;
        println!("Selection Updated: {:?}", game_state.selection);
    }
}

pub fn update_mouse(
    mut mouse: ResMut<Mouse>,
    mut mouse_input: Res<Input<MouseButton>>,
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
