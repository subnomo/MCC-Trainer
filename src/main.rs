use std::{thread, time};

mod ke_interface;
mod game;

use crate::game::Game;

fn main() {
    let game = Game::new();
    invincibility(&game);
    infinite_ammo(&game);
    infinite_grenades(&game);
    infinite_ability(&game);

    // Shield addy changes sometimes, so update
    loop {
        thread::sleep(time::Duration::from_millis(1000));
        update_invincibility(&game);
    }
}

fn infinite_grenades(game: &Game) {
    let dec_location = game.base + 0x29841D;
    game.write_bytes(dec_location, vec![0x90; 6]);
    println!("Infinite grenades!");
}

fn infinite_ability(game: &Game) {
    let dec_location = game.base + 0x21BEAD;
    game.write_bytes(dec_location, vec![0x90; 8]);
    println!("Infinite ability!");
}

fn infinite_ammo(game: &Game) {
    let dec_location = game.base + 0x20F8A6;
    game.write_bytes(dec_location, vec![0x90; 8]);
    println!("Infinite ammo!");
}

fn invincibility(game: &Game) {
    let cave_location = game.base + 0xD181AD;
    let mut shield_addy: u64 = game.read(game.base + 0x01E4489C);
    shield_addy = game.read(shield_addy + 0xD8);
    shield_addy = game.read(shield_addy + 0x5B8);
    shield_addy += 0xE84;

    // Subtract 10 because the instruction needs this
    shield_addy -= 0x10;

    let mut code_cave: Vec<u8> = vec![
        0x50,                                   // push rax
        0x48, 0xB8,                             // mov rax, 
    ];

    // Move shield addy into rax
    for byte in shield_addy.to_le_bytes().iter() {
        code_cave.push(*byte);
    }

    code_cave.append(&mut vec![
        0x4C, 0x3B, 0xE8,                       // cmp r13, rax
        0x58,                                   // pop rax
        0x74, 0x6,                              // je 0x17
        0xF3, 0x41, 0x0F, 0x11, 0x45, 0x10,     // mov [r13+0x10], xmm0
        0xE9, 0x53, 0x60, 0x4A, 0xFF,           // jmp 0xffffffffff4a606f ; jump back to hook
    ]);

    // println!("Code cave: {:x?}", code_cave);
    game.write_bytes(cave_location, code_cave);

    let hook_location = game.base + 0x1BE217;
    let hook: Vec<u8> = vec![
        0xE9, 0x91, 0x9F, 0xB5, 0x0,            // jmp 0xb59f96 ; jump to code cave
        0x90,                                   // nop
    ];

    // println!("Hook: {:x?}", hook);
    game.write_bytes(hook_location, hook);

    println!("Invincible!");
}

fn update_invincibility(game: &Game) {
    let instruction_location = game.base + 0xD181AE;

    let mut shield_addy: u64 = game.read(game.base + 0x01E4489C);
    shield_addy = game.read(shield_addy + 0xD8);
    shield_addy = game.read(shield_addy + 0x5B8);
    shield_addy += 0xE84;

    // Subtract 10 because the instruction needs this
    shield_addy -= 0x10;

    let mut instruction: Vec<u8> = vec![0x48, 0xB8];

    for byte in shield_addy.to_le_bytes().iter() {
        instruction.push(*byte);
    }

    game.write_bytes(instruction_location, instruction);
}
