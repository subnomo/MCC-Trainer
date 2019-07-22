mod ke_interface;
mod game;

use crate::game::Game;

fn main() {
    let game = Game::new();
    invincibility(&game);
    infinite_ammo(&game);
    infinite_grenades(&game);
    infinite_ability(&game);
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
    let mut code_cave: Vec<u8> = vec![
        0x50,                                   // push rax
        0x48, 0xB8,                             // mov rax, 
    ];

    let shield_base: u64 = game.base + 0x01E4489C;

    // Move shield base into rax
    for byte in shield_base.to_le_bytes().iter() {
        code_cave.push(*byte);
    }

    code_cave.append(&mut vec![
        0x48, 0x8B, 0x0,                        // mov rax, [rax]
        0x48, 0x5, 0xD8, 0x0, 0x0, 0x0,         // add rax, D8
        0x48, 0x8B, 0x0,                        // mov rax, [rax]
        0x48, 0x5, 0xB8, 0x5, 0x0, 0x0,         // add rax, 5B8
        0x48, 0x8B, 0x0,                        // mov rax, [rax]
        0x48, 0x5, 0x74, 0xE, 0x0, 0x0,         // add rax, E74
        0x4C, 0x3B, 0xE8,                       // cmp r13, rax
        0x58,                                   // pop rax
        0x74, 0x6,                              // je haloreach.dll+D181C4
        0xF3, 0x41, 0x0F, 0x11, 0x45, 0x10,     // mov [r13+10], xmm0
        0xE9, 0x38, 0x60, 0x4A, 0xFF,           // jmp haloreach.dll+1BE21C ; jump back to hook
    ]);

    // println!("Code cave: {:x?}", code_cave);
    let cave_location = game.base + 0xD181AD;
    game.write_bytes(cave_location, code_cave);

    let hook: Vec<u8> = vec![
        0xE9, 0x91, 0x9F, 0xB5, 0x0,            // jmp haloreach.dll+D181AD ; jump to code cave
        0x90,                                   // nop
    ];

    // println!("Hook: {:x?}", hook);
    let hook_location = game.base + 0x1BE217;
    game.write_bytes(hook_location, hook);

    println!("Invincible!");
}