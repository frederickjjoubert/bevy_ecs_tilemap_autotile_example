#![warn(clippy::all, clippy::pedantic)]

use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::math::Vec4Swizzles;
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::window::PresentMode;
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
        .add_startup_system(setup_rules.label(Setup::Rules))
        .add_startup_system(setup_active_rules.label(Setup::ActiveRules))
        .add_startup_system(setup_sprites.label(Setup::Sprites))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Sprite {
    Blank,
    // Grass Row 1
    Grass_000_010_010,
    Grass_000_011_010,
    Grass_000_111_010,
    Grass_000_110_010,
    Grass_110_111_010,
    Grass_000_111_011,
    Grass_000_111_110,
    Grass_011_111_010,
    Grass_000_011_011,
    Grass_010_111_111,
    Grass_000_111_111,
    Grass_000_110_110,
    // Grass Row 2
    Grass_010_010_010,
    Grass_010_011_010,
    Grass_010_111_010,
    Grass_010_110_010,
    Grass_010_011_011,
    Grass_011_111_111,
    Grass_110_111_111,
    Grass_010_110_110,
    Grass_011_011_011,
    Grass_011_111_110,
    Grass_000_000_000, // Blank
    Grass_110_111_110,
    // Grass Row 3
    Grass_010_010_000,
    Grass_010_011_000,
    Grass_010_111_000,
    Grass_010_110_000,
    Grass_011_011_010,
    Grass_111_111_011,
    Grass_111_111_110,
    Grass_110_110_010,
    Grass_011_111_011,
    Grass_111_111_111,
    Grass_110_111_011,
    Grass_110_110_110,
    // Grass Row 4
    Grass_000_010_000,
    Grass_000_011_000,
    Grass_000_111_000,
    Grass_000_110_000,
    Grass_010_111_110,
    Grass_011_111_000,
    Grass_110_111_000,
    Grass_010_111_011,
    Grass_011_011_000,
    Grass_111_111_000,
    Grass_111_111_010,
    Grass_110_110_000,
    // Dirt Row 1
    Dirt_000_010_010,
    Dirt_000_011_010,
    Dirt_000_111_010,
    Dirt_000_110_010,
    Dirt_110_111_010,
    Dirt_000_111_011,
    Dirt_000_111_110,
    Dirt_011_111_010,
    Dirt_000_011_011,
    Dirt_010_111_111,
    Dirt_000_111_111,
    Dirt_000_110_110,
    // Dirt Row 2
    Dirt_010_010_010,
    Dirt_010_011_010,
    Dirt_010_111_010,
    Dirt_010_110_010,
    Dirt_010_011_011,
    Dirt_011_111_111,
    Dirt_110_111_111,
    Dirt_010_110_110,
    Dirt_011_011_011,
    Dirt_011_111_110,
    Dirt_000_000_000, // Blank
    Dirt_110_111_110,
    // Dirt Row 3
    Dirt_010_010_000,
    Dirt_010_011_000,
    Dirt_010_111_000,
    Dirt_010_110_000,
    Dirt_011_011_010,
    Dirt_111_111_011,
    Dirt_111_111_110,
    Dirt_110_110_010,
    Dirt_011_111_011,
    Dirt_111_111_111,
    Dirt_110_111_011,
    Dirt_110_110_110,
    // Dirt Row 4
    Dirt_000_010_000,
    Dirt_000_011_000,
    Dirt_000_111_000,
    Dirt_000_110_000,
    Dirt_010_111_110,
    Dirt_011_111_000,
    Dirt_110_111_000,
    Dirt_010_111_011,
    Dirt_011_011_000,
    Dirt_111_111_000,
    Dirt_111_111_010,
    Dirt_110_110_000,
    // Water
    Water_0,
    Water_1,
    Water_2,
    Water_3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpriteType {
    Blank,
    Grass,
    Dirt,
    Water,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Slot {
    Empty,
    Filled { sprite_type: SpriteType },
    Any,
}

// === Struts ===
#[derive(Copy, Clone, Debug)]
pub struct Rule {
    pub nw_slot: Slot,
    pub n_slot: Slot,
    pub ne_slot: Slot,
    pub w_slot: Slot,
    pub c_slot: Slot,
    pub e_slot: Slot,
    pub sw_slot: Slot,
    pub s_slot: Slot,
    pub se_slot: Slot,
}

impl Eq for Rule {}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        let nw = self.nw_slot == other.nw_slot
            || self.nw_slot == Slot::Any
            || other.nw_slot == Slot::Any;
        let n =
            self.n_slot == other.n_slot || self.n_slot == Slot::Any || other.n_slot == Slot::Any;
        let ne = self.ne_slot == other.ne_slot
            || self.ne_slot == Slot::Any
            || other.ne_slot == Slot::Any;
        let w =
            self.w_slot == other.w_slot || self.w_slot == Slot::Any || other.w_slot == Slot::Any;
        let c =
            self.c_slot == other.c_slot || self.c_slot == Slot::Any || other.c_slot == Slot::Any;
        let e =
            self.e_slot == other.e_slot || self.e_slot == Slot::Any || other.e_slot == Slot::Any;
        let sw = self.sw_slot == other.sw_slot
            || self.sw_slot == Slot::Any
            || other.sw_slot == Slot::Any;
        let s =
            self.s_slot == other.s_slot || self.s_slot == Slot::Any || other.s_slot == Slot::Any;
        let se = self.se_slot == other.se_slot
            || self.se_slot == Slot::Any
            || other.se_slot == Slot::Any;
        nw && n && ne && w && c && e && sw && s && se
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
    pub selection: SpriteType,
}

pub struct Sprites {
    pub sprite_lookup_table: HashMap<Sprite, u32>,
}

pub struct Rules {
    pub rules: HashMap<SpriteType, Vec<(Rule, Sprite)>>,
}

pub struct ActiveRules {
    pub active_rules: HashMap<TilePos, Rule>,
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
    let sprites = Sprites {
        sprite_lookup_table: HashMap::from([
            // Grass Row 1
            (Sprite::Grass_000_010_010, 0),
            (Sprite::Grass_000_011_010, 1),
            (Sprite::Grass_000_111_010, 2),
            (Sprite::Grass_000_110_010, 3),
            (Sprite::Grass_110_111_010, 4),
            (Sprite::Grass_000_111_011, 5),
            (Sprite::Grass_000_111_110, 6),
            (Sprite::Grass_011_111_010, 7),
            (Sprite::Grass_000_011_011, 8),
            (Sprite::Grass_010_111_111, 9),
            (Sprite::Grass_000_111_111, 10),
            (Sprite::Grass_000_110_110, 11),
            // Grass Row 2
            (Sprite::Grass_010_010_010, 12),
            (Sprite::Grass_010_011_010, 13),
            (Sprite::Grass_010_111_010, 14),
            (Sprite::Grass_010_110_010, 15),
            (Sprite::Grass_010_011_011, 16),
            (Sprite::Grass_011_111_111, 17),
            (Sprite::Grass_110_111_111, 18),
            (Sprite::Grass_010_110_110, 19),
            (Sprite::Grass_011_011_011, 20),
            (Sprite::Grass_011_111_110, 21),
            (Sprite::Grass_000_000_000, 22),
            (Sprite::Grass_110_111_110, 23),
            // Grass Row 3
            (Sprite::Grass_010_010_000, 24),
            (Sprite::Grass_010_011_000, 25),
            (Sprite::Grass_010_111_000, 26),
            (Sprite::Grass_010_110_000, 27),
            (Sprite::Grass_011_011_010, 28),
            (Sprite::Grass_111_111_011, 29),
            (Sprite::Grass_111_111_110, 30),
            (Sprite::Grass_110_110_010, 31),
            (Sprite::Grass_011_111_011, 32),
            (Sprite::Grass_111_111_111, 33),
            (Sprite::Grass_110_111_011, 34),
            (Sprite::Grass_110_110_110, 35),
            // Grass Row 4
            (Sprite::Grass_000_010_000, 36),
            (Sprite::Grass_000_011_000, 37),
            (Sprite::Grass_000_111_000, 38),
            (Sprite::Grass_000_110_000, 39),
            (Sprite::Grass_010_111_110, 40),
            (Sprite::Grass_011_111_000, 41),
            (Sprite::Grass_110_111_000, 42),
            (Sprite::Grass_010_111_011, 43),
            (Sprite::Grass_011_011_000, 44),
            (Sprite::Grass_111_111_000, 45),
            (Sprite::Grass_111_111_010, 46),
            (Sprite::Grass_110_110_000, 47),
            // Dirt Row 1
            (Sprite::Dirt_000_010_010, 48),
            (Sprite::Dirt_000_011_010, 49),
            (Sprite::Dirt_000_111_010, 50),
            (Sprite::Dirt_000_110_010, 51),
            (Sprite::Dirt_110_111_010, 52),
            (Sprite::Dirt_000_111_011, 53),
            (Sprite::Dirt_000_111_110, 54),
            (Sprite::Dirt_011_111_010, 55),
            (Sprite::Dirt_000_011_011, 56),
            (Sprite::Dirt_010_111_111, 57),
            (Sprite::Dirt_000_111_111, 58),
            (Sprite::Dirt_000_110_110, 59),
            // Dirt Row 2
            (Sprite::Dirt_010_010_010, 60),
            (Sprite::Dirt_010_011_010, 61),
            (Sprite::Dirt_010_111_010, 62),
            (Sprite::Dirt_010_110_010, 63),
            (Sprite::Dirt_010_011_011, 64),
            (Sprite::Dirt_011_111_111, 65),
            (Sprite::Dirt_110_111_111, 66),
            (Sprite::Dirt_010_110_110, 67),
            (Sprite::Dirt_011_011_011, 68),
            (Sprite::Dirt_011_111_110, 69),
            (Sprite::Dirt_000_000_000, 70),
            (Sprite::Dirt_110_111_110, 71),
            // Dirt Row 3
            (Sprite::Dirt_010_010_000, 72),
            (Sprite::Dirt_010_011_000, 73),
            (Sprite::Dirt_010_111_000, 74),
            (Sprite::Dirt_010_110_000, 75),
            (Sprite::Dirt_011_011_010, 76),
            (Sprite::Dirt_111_111_011, 77),
            (Sprite::Dirt_111_111_110, 78),
            (Sprite::Dirt_110_110_010, 79),
            (Sprite::Dirt_011_111_011, 80),
            (Sprite::Dirt_111_111_111, 81),
            (Sprite::Dirt_110_111_011, 82),
            (Sprite::Dirt_110_110_110, 83),
            // Dirt Row 4
            (Sprite::Dirt_000_010_000, 84),
            (Sprite::Dirt_000_011_000, 85),
            (Sprite::Dirt_000_111_000, 86),
            (Sprite::Dirt_000_110_000, 87),
            (Sprite::Dirt_010_111_110, 88),
            (Sprite::Dirt_011_111_000, 89),
            (Sprite::Dirt_110_111_000, 90),
            (Sprite::Dirt_010_111_011, 91),
            (Sprite::Dirt_011_011_000, 92),
            (Sprite::Dirt_111_111_000, 93),
            (Sprite::Dirt_111_111_010, 94),
            (Sprite::Dirt_110_110_000, 95),
            // Water
            (Sprite::Water_0, 96),
            (Sprite::Water_1, 97),
            (Sprite::Water_2, 98),
            (Sprite::Water_3, 99),
            // Blank
            (Sprite::Blank, 102),
        ]),
    };

    commands.insert_resource(sprites);
}

pub fn setup_rules(mut commands: Commands) {
    let rules = Rules {
        rules: HashMap::from([(
            SpriteType::Grass,
            Vec::from([
                // Basic Rules
                // === Row 1 ===
                // 0
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Empty,
                        ne_slot: Slot::Any,
                        w_slot: Slot::Empty,
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Empty,
                        sw_slot: Slot::Any,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_000_010_010,
                ),
                // 1
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Empty,
                        ne_slot: Slot::Any,
                        w_slot: Slot::Empty,
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Any,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Empty,
                    },
                    Sprite::Grass_000_011_010,
                ),
                // 2
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Empty,
                        ne_slot: Slot::Any,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Empty,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Empty,
                    },
                    Sprite::Grass_000_111_010,
                ),
                // 3
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Empty,
                        ne_slot: Slot::Any,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Empty,
                        sw_slot: Slot::Empty,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_000_110_010,
                ),
                // 4
                (
                    Rule {
                        nw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Empty,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Empty,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Empty,
                    },
                    Sprite::Grass_110_111_010,
                ),
                // 5
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Empty,
                        ne_slot: Slot::Any,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Empty,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                    },
                    Sprite::Grass_000_111_011,
                ),
                // 6
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Empty,
                        ne_slot: Slot::Any,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Empty,
                    },
                    Sprite::Grass_000_111_110,
                ),
                // 7
                (
                    Rule {
                        nw_slot: Slot::Empty,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Empty,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Empty,
                    },
                    Sprite::Grass_011_111_010,
                ),
                // 8
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Empty,
                        ne_slot: Slot::Any,
                        w_slot: Slot::Empty,
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Any,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                    },
                    Sprite::Grass_000_011_011,
                ),
                // 9
                (
                    Rule {
                        nw_slot: Slot::Empty,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Empty,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                    },
                    Sprite::Grass_010_111_111,
                ),
                // 10
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Empty,
                        ne_slot: Slot::Any,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                    },
                    Sprite::Grass_000_111_111,
                ),
                // 11
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Empty,
                        ne_slot: Slot::Any,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Empty,
                        sw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_000_110_110,
                ),
                // === Row 2 ===
                // 12
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Any,
                        w_slot: Slot::Empty,
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Empty,
                        sw_slot: Slot::Any,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_010_010_010,
                ),
                // 13
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Empty,
                        w_slot: Slot::Empty,
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Any,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Empty,
                    },
                    Sprite::Grass_010_011_010,
                ),
                // 14
                (
                    Rule {
                        nw_slot: Slot::Empty,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Empty,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Empty,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Empty,
                    },
                    Sprite::Grass_010_111_010,
                ),
                // 15
                (
                    Rule {
                        nw_slot: Slot::Empty,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Any,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Empty,
                        sw_slot: Slot::Empty,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_010_110_010,
                ),
                // 16
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Empty,
                        w_slot: Slot::Empty,
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Any,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                    },
                    Sprite::Grass_010_011_011,
                ),
                // 17
                (
                    Rule {
                        nw_slot: Slot::Empty,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                    },
                    Sprite::Grass_011_111_111,
                ),
                // 18
                (
                    Rule {
                        nw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Empty,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                    },
                    Sprite::Grass_110_111_111,
                ),
                // 19
                (
                    Rule {
                        nw_slot: Slot::Empty,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Any,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Empty,
                        sw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_010_110_110,
                ),
                // 20
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        w_slot: Slot::Empty,
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Any,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                    },
                    Sprite::Grass_011_011_011,
                ),
                // 21
                (
                    Rule {
                        nw_slot: Slot::Empty,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Empty,
                    },
                    Sprite::Grass_011_111_110,
                ),
                // 22
                // No Rule for blank tile.
                // 23
                (
                    Rule {
                        nw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Empty,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Empty,
                    },
                    Sprite::Grass_110_111_110,
                ),
                // === Row 3 ===
                // 24
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Any,
                        w_slot: Slot::Empty,
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Empty,
                        sw_slot: Slot::Any,
                        s_slot: Slot::Empty,
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_010_010_000,
                ),
                // 25
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Empty,
                        w_slot: Slot::Empty,
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Any,
                        s_slot: Slot::Empty,
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_010_011_000,
                ),
                // 26
                (
                    Rule {
                        nw_slot: Slot::Empty,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Empty,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Any,
                        s_slot: Slot::Empty,
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_010_111_000,
                ),
                // 27
                (
                    Rule {
                        nw_slot: Slot::Empty,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Any,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Empty,
                        sw_slot: Slot::Any,
                        s_slot: Slot::Empty,
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_010_110_000,
                ),
                // 28
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        w_slot: Slot::Empty,
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Any,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Empty,
                    },
                    Sprite::Grass_011_011_010,
                ),
                // 29
                (
                    Rule {
                        nw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Empty,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                    },
                    Sprite::Grass_111_111_011,
                ),
                // 30
                (
                    Rule {
                        nw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Empty,
                    },
                    Sprite::Grass_111_111_110,
                ),
                // 31
                (
                    Rule {
                        nw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Any,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Empty,
                        sw_slot: Slot::Empty,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_110_110_010,
                ),
                // 32
                (
                    Rule {
                        nw_slot: Slot::Empty,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Empty,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                    },
                    Sprite::Grass_011_111_011,
                ),
                // 33
                (
                    Rule {
                        nw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                    },
                    Sprite::Grass_111_111_111,
                ),
                // 34
                (
                    Rule {
                        nw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Empty,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Empty,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                    },
                    Sprite::Grass_110_111_011,
                ),
                // 35
                (
                    Rule {
                        nw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Any,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Empty,
                        sw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_110_110_110,
                ),
                // === Row 4 ===
                // 36
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Empty,
                        ne_slot: Slot::Any,
                        w_slot: Slot::Empty,
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Empty,
                        sw_slot: Slot::Any,
                        s_slot: Slot::Empty,
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_000_010_000,
                ),
                // 37
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Empty,
                        ne_slot: Slot::Any,
                        w_slot: Slot::Empty,
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Any,
                        s_slot: Slot::Empty,
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_000_011_000,
                ),
                // 38
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Empty,
                        ne_slot: Slot::Any,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Any,
                        s_slot: Slot::Empty,
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_000_111_000,
                ),
                // 39
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Empty,
                        ne_slot: Slot::Any,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Empty,
                        sw_slot: Slot::Any,
                        s_slot: Slot::Empty,
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_000_110_000,
                ),
                // 40
                (
                    Rule {
                        nw_slot: Slot::Empty,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Empty,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Empty,
                    },
                    Sprite::Grass_010_111_110,
                ),
                // 41
                (
                    Rule {
                        nw_slot: Slot::Empty,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Any,
                        s_slot: Slot::Empty,
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_011_111_000,
                ),
                // 42
                (
                    Rule {
                        nw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Empty,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Any,
                        s_slot: Slot::Empty,
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_110_111_000,
                ),
                // 43
                (
                    Rule {
                        nw_slot: Slot::Empty,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Empty,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Empty,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                    },
                    Sprite::Grass_010_111_011,
                ),
                // 44
                (
                    Rule {
                        nw_slot: Slot::Any,
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        w_slot: Slot::Empty,
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Any,
                        s_slot: Slot::Empty,
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_011_011_000,
                ),
                // 45
                (
                    Rule {
                        nw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Any,
                        s_slot: Slot::Empty,
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_111_111_000,
                ),
                // 46
                (
                    Rule {
                        nw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        sw_slot: Slot::Empty,
                        s_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        se_slot: Slot::Empty,
                    },
                    Sprite::Grass_111_111_010,
                ),
                // 47
                (
                    Rule {
                        nw_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        n_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        ne_slot: Slot::Any,
                        w_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        c_slot: Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        },
                        e_slot: Slot::Empty,
                        sw_slot: Slot::Any,
                        s_slot: Slot::Empty,
                        se_slot: Slot::Any,
                    },
                    Sprite::Grass_110_110_000,
                ),
                // Custom Rules
            ]),
        )]),
    };
    commands.insert_resource(rules);
}

pub fn setup_active_rules(mut commands: Commands) {
    let active_rules = ActiveRules {
        active_rules: HashMap::new(),
    };
    commands.insert_resource(active_rules);
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
                    texture: TileTexture(102), // 102 is blank
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
            map_type: TilemapType::Square {
                diagonal_neighbors: true,
            },
            tile_size,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        });
}

pub fn setup_game(mut commands: Commands) {
    let game_state = GameState {
        selection: SpriteType::Grass,
    };
    commands.insert_resource(game_state);
}

// === Systems ===
pub fn place_tile(
    mut commands: Commands,
    mut update_tilemap_event_writer: EventWriter<UpdateTilemapEvent>,
    game_state: Res<GameState>,
    mouse: Res<Mouse>,
    mouse_input: Res<Input<MouseButton>>,
    tilemap_query: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
) {
    if mouse.holding_lmb {
        // if mouse_input.justs_pressed(MouseButton::Left) {
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
                        SpriteType::Grass => {
                            commands.entity(tile_entity).insert(GrassTile {});
                        }
                        SpriteType::Dirt => {
                            commands.entity(tile_entity).insert(DirtTile {});
                        }
                        SpriteType::Water => {
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
    mut tilemap_query: Query<(&TileStorage, &TilemapType)>,
    mut active_rules: ResMut<ActiveRules>,
) {
    for _ in update_tilemap_event_reader.iter() {
        if let Ok((tile_storage, tilemap_type)) = tilemap_query.get_single_mut() {
            // Clear Previous Active Rules
            active_rules.active_rules.clear();

            // Grass Tiles
            for tile_position in grass_tiles_query.iter() {
                // 1. Create current rule for tile neighbors.
                let neighbors = get_tile_neighbors(tile_position, tile_storage, tilemap_type);

                // NW
                let north_west_slot: Slot;
                if let Some(nw_neighbor) = neighbors.north_west {
                    // Can also change this system to only check for Grass Tiles and resolve others to Any.
                    if grass_tiles_query.contains(nw_neighbor) {
                        north_west_slot = Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        };
                    } else if dirt_tiles_query.contains(nw_neighbor) {
                        north_west_slot = Slot::Filled {
                            sprite_type: SpriteType::Dirt,
                        }
                    } else if water_tiles_query.contains(nw_neighbor) {
                        north_west_slot = Slot::Filled {
                            sprite_type: SpriteType::Water,
                        }
                    } else {
                        north_west_slot = Slot::Empty
                    }
                } else {
                    north_west_slot = Slot::Empty
                }

                // N
                let north_slot: Slot;
                if let Some(north_neighbor_entity) = neighbors.north {
                    // Can also change this system to only check for Grass Tiles and resolve others to Any.
                    if grass_tiles_query.contains(north_neighbor_entity) {
                        north_slot = Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        }
                    } else if dirt_tiles_query.contains(north_neighbor_entity) {
                        north_slot = Slot::Filled {
                            sprite_type: SpriteType::Dirt,
                        }
                    } else if water_tiles_query.contains(north_neighbor_entity) {
                        north_slot = Slot::Filled {
                            sprite_type: SpriteType::Water,
                        }
                    } else {
                        north_slot = Slot::Empty
                    }
                } else {
                    north_slot = Slot::Empty
                }

                // NE
                let north_east_slot: Slot;
                if let Some(north_east_neighbor_entity) = neighbors.north_east {
                    // Can also change this system to only check for Grass Tiles and resolve others to Any.
                    if grass_tiles_query.contains(north_east_neighbor_entity) {
                        north_east_slot = Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        }
                    } else if dirt_tiles_query.contains(north_east_neighbor_entity) {
                        north_east_slot = Slot::Filled {
                            sprite_type: SpriteType::Dirt,
                        }
                    } else if water_tiles_query.contains(north_east_neighbor_entity) {
                        north_east_slot = Slot::Filled {
                            sprite_type: SpriteType::Water,
                        }
                    } else {
                        north_east_slot = Slot::Empty
                    }
                } else {
                    north_east_slot = Slot::Empty
                }

                // W
                let west_slot: Slot;
                if let Some(west_neighbor_entity) = neighbors.west {
                    // Can also change this system to only check for Grass Tiles and resolve others to Any.
                    if grass_tiles_query.contains(west_neighbor_entity) {
                        west_slot = Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        }
                    } else if dirt_tiles_query.contains(west_neighbor_entity) {
                        west_slot = Slot::Filled {
                            sprite_type: SpriteType::Dirt,
                        }
                    } else if water_tiles_query.contains(west_neighbor_entity) {
                        west_slot = Slot::Filled {
                            sprite_type: SpriteType::Water,
                        }
                    } else {
                        west_slot = Slot::Empty
                    }
                } else {
                    west_slot = Slot::Empty
                }

                // C
                let center_slot = Slot::Filled {
                    sprite_type: SpriteType::Grass,
                };

                // E
                let east_slot: Slot;
                if let Some(east_neighbor_entity) = neighbors.east {
                    // Can also change this system to only check for Grass Tiles and resolve others to Any.
                    if grass_tiles_query.contains(east_neighbor_entity) {
                        east_slot = Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        }
                    } else if dirt_tiles_query.contains(east_neighbor_entity) {
                        east_slot = Slot::Filled {
                            sprite_type: SpriteType::Dirt,
                        }
                    } else if water_tiles_query.contains(east_neighbor_entity) {
                        east_slot = Slot::Filled {
                            sprite_type: SpriteType::Water,
                        }
                    } else {
                        east_slot = Slot::Empty
                    }
                } else {
                    east_slot = Slot::Empty
                }

                // SW
                let south_west_slot: Slot;
                if let Some(south_west_neighbor_entity) = neighbors.south_west {
                    // Can also change this system to only check for Grass Tiles and resolve others to Any.
                    if grass_tiles_query.contains(south_west_neighbor_entity) {
                        south_west_slot = Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        }
                    } else if dirt_tiles_query.contains(south_west_neighbor_entity) {
                        south_west_slot = Slot::Filled {
                            sprite_type: SpriteType::Dirt,
                        }
                    } else if water_tiles_query.contains(south_west_neighbor_entity) {
                        south_west_slot = Slot::Filled {
                            sprite_type: SpriteType::Water,
                        }
                    } else {
                        south_west_slot = Slot::Empty
                    }
                } else {
                    south_west_slot = Slot::Empty
                }

                // S
                let south_slot: Slot;
                if let Some(south_neighbor_entity) = neighbors.south {
                    // Can also change this system to only check for Grass Tiles and resolve others to Any.
                    if grass_tiles_query.contains(south_neighbor_entity) {
                        south_slot = Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        }
                    } else if dirt_tiles_query.contains(south_neighbor_entity) {
                        south_slot = Slot::Filled {
                            sprite_type: SpriteType::Dirt,
                        }
                    } else if water_tiles_query.contains(south_neighbor_entity) {
                        south_slot = Slot::Filled {
                            sprite_type: SpriteType::Water,
                        }
                    } else {
                        south_slot = Slot::Empty
                    }
                } else {
                    south_slot = Slot::Empty
                }

                // SE
                let south_east_slot: Slot;
                if let Some(south_east_neighbor_entity) = neighbors.south_east {
                    // Can also change this system to only check for Grass Tiles and resolve others to Any.
                    if grass_tiles_query.contains(south_east_neighbor_entity) {
                        south_east_slot = Slot::Filled {
                            sprite_type: SpriteType::Grass,
                        }
                    } else if dirt_tiles_query.contains(south_east_neighbor_entity) {
                        south_east_slot = Slot::Filled {
                            sprite_type: SpriteType::Dirt,
                        }
                    } else if water_tiles_query.contains(south_east_neighbor_entity) {
                        south_east_slot = Slot::Filled {
                            sprite_type: SpriteType::Water,
                        }
                    } else {
                        south_east_slot = Slot::Empty
                    }
                } else {
                    south_east_slot = Slot::Empty
                }

                let current_rule = Rule {
                    nw_slot: north_west_slot,
                    n_slot: north_slot,
                    ne_slot: north_east_slot,
                    w_slot: west_slot,
                    c_slot: center_slot,
                    e_slot: east_slot,
                    sw_slot: south_west_slot,
                    s_slot: south_slot,
                    se_slot: south_east_slot,
                };

                active_rules
                    .active_rules
                    .insert(*tile_position, current_rule);
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
    mut water_tiles_query: Query<
        (&TilePos, &mut TileTexture),
        (With<WaterTile>, Without<GrassTile>, Without<DirtTile>),
    >,
    sprites: Res<Sprites>,
    active_rules: Res<ActiveRules>,
    rules: Res<Rules>,
) {
    // Perform auto tiling based on neighbors and rules
    if active_rules.is_changed() {
        let possible_rules = rules.rules[&SpriteType::Grass].clone();
        for (tile_position, mut tile_texture) in grass_tiles_query.iter_mut() {
            if active_rules.active_rules.contains_key(tile_position) {
                let active_rule = active_rules.active_rules[tile_position];
                let mut new_sprite = Sprite::Blank;
                for (rule, sprite) in possible_rules.iter() {
                    if active_rule == *rule {
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
        game_state.selection = SpriteType::Blank;
        println!("Selection Updated: {:?}", game_state.selection);
    } else if keyboard.just_pressed(KeyCode::Key2) {
        game_state.selection = SpriteType::Grass;
        println!("Selection Updated: {:?}", game_state.selection);
    } else if keyboard.just_pressed(KeyCode::Key3) {
        game_state.selection = SpriteType::Dirt;
        println!("Selection Updated: {:?}", game_state.selection);
    } else if keyboard.just_pressed(KeyCode::Key4) {
        game_state.selection = SpriteType::Water;
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
