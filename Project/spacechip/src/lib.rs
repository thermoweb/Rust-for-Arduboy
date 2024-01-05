#![no_std]
#![allow(non_upper_case_globals)]

//Include the Arduboy Library
#[allow(unused_imports)]
use arduboy_rust::prelude::*;

#[allow(dead_code)]
const arduboy: Arduboy2 = Arduboy2::new();

static mut spaceship_x: i16 = 0;
static mut spaceship_y: i16 = 0;

// Progmem data
progmem!(
    static spaceship_sprite1: [u8; _] = [
        16, 16,
        0x00, 0x82, 0x86, 0x6a, 0xf2, 0x12, 0xcc, 0xa8, 0xa8, 0xc8, 0x18, 0xe8, 0x10, 0x20, 0x40, 0x80,
        0x01, 0x42, 0x62, 0x54, 0x4f, 0x48, 0x33, 0x15, 0x15, 0x13, 0x18, 0x17, 0x08, 0x04, 0x02, 0x01,
    ];
);

// dynamic ram variables

enum GameSate {
    Title,
    Gameplay,
}

struct SpaceChip {
    state: GameSate,
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
    if !arduboy.next_frame() {
        return;
    }

    update();
    display();
}

unsafe fn update() {
    arduboy.poll_buttons();
    if arduboy.pressed(UP) && spaceship_y > -8 {
        spaceship_y -= 1;
    }
    if arduboy.pressed(DOWN) {
        spaceship_y += 1;
    }
    if arduboy.pressed(LEFT) && spaceship_x > 0 {
        spaceship_x -= 1;
    }
    if arduboy.pressed(RIGHT) {
        spaceship_x += 1;
    }
    if arduboy.pressed(A) {}
    if arduboy.pressed(B) {}
}

unsafe fn display() {
    arduboy.clear();

    // put your main code here, to run repeatedly:
    sprites::draw_override(spaceship_x, spaceship_y, get_sprite_addr!(spaceship_sprite1), 0);

    arduboy.set_cursor(0, 0);
    arduboy.print(spaceship_x);
    arduboy.print(f!(b",\0"));
    arduboy.print(spaceship_y);
    arduboy.display();
}