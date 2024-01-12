#![no_std]
#![allow(non_upper_case_globals)]

//Include the Arduboy Library
#[allow(unused_imports)]
use arduboy_rust::prelude::*;

#[allow(dead_code)]
const arduboy: Arduboy2 = Arduboy2::new();

const TILE_SIZE: i16 = 16;
const PLAYER_SIZE: i16 = 16;
const PLAYER_X_OFFSET: i16 = WIDTH as i16 / 2 - PLAYER_SIZE / 2;
const PLAYER_Y_OFFSET: i16 = HEIGHT as i16 / 2 - PLAYER_SIZE / 2;


const GRASS: u8 = 0;
const WATER: u8 = 1;
const TREES: u8 = 2;
const STONE: u8 = 3;
const WORLD_WIDTH: usize = 6;
const WORLD_HEIGHT: usize = 6;

const world: [[u8; WORLD_WIDTH]; WORLD_HEIGHT] = [
    [WATER, WATER, WATER, WATER, WATER, WATER],
    [TREES, STONE, GRASS, GRASS, GRASS, TREES],
    [TREES, GRASS, GRASS, GRASS, GRASS, TREES],
    [TREES, GRASS, GRASS, GRASS, GRASS, TREES],
    [TREES, GRASS, GRASS, GRASS, GRASS, TREES],
    [TREES, TREES, TREES, TREES, TREES, TREES],
];

// Progmem data
progmem!(
    static monster_sprites: [u8; _] = [
        16, 16,
        0x00, 0xE0, 0xF0, 0x18, 0x18, 0xFC, 0xFC, 0x7C, 0x7C, 0xFC, 0xFC, 0x18, 0x18, 0xF0, 0xE0, 0x00,
        0x00, 0x01, 0x03, 0x07, 0x24, 0x37, 0xFE, 0x7A, 0x5A, 0xFA, 0x37, 0x24, 0x07, 0x03, 0x01, 0x00
    ];
    static player_sprites: [u8; _] = [
        16, 16,
        0x00, 0x00, 0x00, 0xF0, 0xF8, 0x3C, 0xDC, 0xBC, 0x7C, 0xFC, 0xFC, 0xF8, 0xF0, 0x00, 0x00, 0x00,
        0x00, 0x18, 0x34, 0x3E, 0x1D, 0xBA, 0xBA, 0x3B, 0x13, 0xBA, 0xBA, 0x1C, 0x3E, 0x3C, 0x18, 0x00
    ];
    static tiles: [u8; _] = [
        16, 16, // width, height,
        //Grass
        0xff, 0x7f, 0xfb, 0xff, 0xff, 0xbf, 0xff, 0xff, 0xf7, 0xff, 0xfd, 0xff, 0xff, 0xf7, 0x7f,
        0xff, 0xdf, 0xff, 0xff, 0xfb, 0x7f, 0xff, 0xff, 0xff, 0xef, 0xfe, 0xff, 0xff, 0xfb, 0xff,
        0x7f, 0xff, //Water
        0x08, 0x10, 0x10, 0x08, 0x10, 0x08, 0x10, 0x10, 0x10, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x20, 0x40, 0x40, 0x20, 0x00, 0x01, 0x02, 0x02, 0x01, 0x02, 0x02, 0x01, 0x02, 0x21,
        0x40, 0x40, //Tree
        0xff, 0x1f, 0x5b, 0x3f, 0xeb, 0xdd, 0xff, 0xf7, 0xbb, 0xef, 0xfd, 0x7f, 0xe3, 0xcb, 0xe3,
        0xff, 0xff, 0xc7, 0x96, 0xc7, 0xff, 0xff, 0xef, 0xfd, 0xff, 0xe3, 0xcb, 0xe3, 0xff, 0xff,
        0x7b, 0xff, //Stone
        0xff, 0xdf, 0x7b, 0x3f, 0x9f, 0x6f, 0x77, 0xab, 0xdb, 0xd7, 0xcd, 0x5f, 0xbf, 0x77, 0xff,
        0xff, 0xff, 0xc1, 0xdc, 0xd3, 0xaf, 0x9f, 0xae, 0xb0, 0xbb, 0xbd, 0xbd, 0xba, 0xd7, 0xcc,
        0x63, 0xff,
    ];
);

// dynamic ram variables
static mut player: Player = Player::new();
const rpgSolo: RpgSolo = RpgSolo { state: GameSate::Title, debug: false };

enum GameSate {
    Title,
    Gameplay,
}

struct RpgSolo {
    state: GameSate,
    debug: bool,
}

enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

struct Player {
    world_x: i16,
    world_y: i16,
    screen_x: i16,
    screen_y: i16,
}

impl Player {
    pub const fn new() -> Player {
        Player { world_x: 1 * TILE_SIZE, world_y: 1 * TILE_SIZE, screen_x: 64, screen_y: 32 }
    }

    fn movement(&mut self, direction: Direction) {
        match direction {
            Direction::UP => self.world_y += 1,
            Direction::DOWN => self.world_y -= 1,
            Direction::LEFT => self.world_x += 1,
            Direction::RIGHT => self.world_x -= 1,
        }
    }
}

// The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    arduboy.clear();
    arduboy.set_frame_rate(60);
}

// The loop() function repeats forever after setup() is done
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    // put your main code here, to run repeatedly:
    if !arduboy.next_frame() {
        return;
    }

    update();
    display();
}

unsafe fn update() {
    arduboy.poll_buttons();
    if arduboy.pressed(UP) && player.screen_y > -8 {
        player.movement(Direction::UP);
    }
    if arduboy.pressed(DOWN) {
        player.movement(Direction::DOWN);
    }
    if arduboy.pressed(LEFT) && player.screen_x > 0 {
        player.movement(Direction::LEFT);
    }
    if arduboy.pressed(RIGHT) {
        player.movement(Direction::RIGHT);
    }
    if arduboy.just_pressed(A) && arduboy.just_pressed(B) {
        rpgSolo.debug = if rpgSolo.debug { false } else { true };
    }
    if arduboy.pressed(A) {}
    if arduboy.pressed(B) {}
}

unsafe fn display() {
    arduboy.clear();

    // put your main code here, to run repeatedly:
    draw_world();
    sprites::draw_override(player.screen_x, player.screen_y, get_sprite_addr!(player_sprites), 0);

    if rpgSolo.debug {
        arduboy.set_cursor(0, 0);
        arduboy.print(player.world_x);
        arduboy.print(f!(b",\0"));
        arduboy.print(player.world_y);
    }
    arduboy.display();
}

unsafe fn draw_world() {
    let tileswide: i16 = WIDTH / TILE_SIZE + 1;
    let tilestall: i16 = HEIGHT / TILE_SIZE + 1;
    for y in 0..tilestall {
        for x in 0..tileswide {
            let tilesx: i16 = x - player.world_x / TILE_SIZE;
            let tilesy: i16 = y - player.world_y / TILE_SIZE;
            if tilesx >= 0
                && tilesy >= 0
                && tilesx < WORLD_WIDTH as i16
                && tilesy < WORLD_HEIGHT as i16 {
                sprites::draw_override(
                    x * TILE_SIZE + player.world_x % TILE_SIZE,
                    y * TILE_SIZE + player.world_y % TILE_SIZE,
                    get_sprite_addr!(tiles),
                    world[tilesy as usize][tilesx as usize],
                );
            }
        }
    }
}