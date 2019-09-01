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
    let dec_location = game.base + 0x693B2D;
    game.write_bytes(dec_location, vec![0x90; 6]);
    println!("Infinite grenades!");
}

fn infinite_ability(game: &Game) {
    let dec_location = game.base + 0x628EFD;
    game.write_bytes(dec_location, vec![0x90; 8]);
    println!("Infinite ability!");
}

fn infinite_ammo(game: &Game) {
    let dec_location = game.base + 0x61E146;
    game.write_bytes(dec_location, vec![0x90; 8]);
    println!("Infinite ammo!");
}

fn invincibility(game: &Game) {
    let code_cave: Vec<u8> = vec![
        0x50,                                               // push rax
        0x51,                                               // push rcx
        0x52,                                               // push rdx
        0x41, 0x50,                                         // push r8
        0x41, 0x51,                                         // push r9
        0x41, 0x52,                                         // push r10
        0x44, 0x8B, 0x5, 0xDB, 0xE7, 0xFD, 0xFF,            // mov r8d, [haloreach.dll+CF6998]
        0x65, 0x48, 0x8B, 0x4, 0x25, 0x58, 0x0, 0x0, 0x0,   // mov rax, gs:[58]
        0x41, 0xB9, 0x8, 0x0, 0x0, 0x0,                     // mov r9d, 8
        0xF, 0xB7, 0xC9,                                    // movzx ecx, cx
        0xBA, 0x2, 0x0, 0x0, 0x0,                           // mov edx, 2
        0x4C, 0x63, 0xD2,                                   // movsxd r10, edx
        0x4A, 0x8B, 0x4, 0xC0,                              // mov rax, [rax+r8*8]
        0x4C, 0x8D, 0x4, 0x49,                              // lea r8, [rcx+rcx*2]
        0x4E, 0x8B, 0x0C, 0x8,                              // mov r9, [rax+r9]
        0x49, 0x8B, 0x41, 0x50,                             // mov rax, [r9+50]
        0x4E, 0x8B, 0x4C, 0xC0, 0x10,                       // mov r9, [rax+r8*8+10]
        0x49, 0xF, 0xBF, 0x89, 0x88, 0x1, 0x0, 0x0,         // movsx rcx, word ptr [r9+188]
        0x49, 0xF, 0xBF, 0x81, 0x8A, 0x1, 0x0, 0x0,         // movsx rax, word ptr [r9+18A]
        0x4B, 0x8D, 0x0C, 0x52,                             // lea rcx, [r10+r10*2]
        0x49, 0x03, 0xC1,                                   // add rax, r9
        0x48, 0x8D, 0x4, 0xC8,                              // lea rax, [rax+rcx*8]
        0x49, 0x3B, 0xC5,                                   // cmp rax, r13
        0x74, 0x6,                                          // je $+6 ; skip next line
        0xF3, 0x41, 0xF, 0x11, 0x45, 0x10,                  // movss [r13+10], xmm0
        0x41, 0x5A,                                         // pop r10
        0x41, 0x59,                                         // pop r9
        0x41, 0x58,                                         // pop r8
        0x5A,                                               // pop rdx
        0x59,                                               // pop rcx
        0x58,                                               // pop rax
        0xE9, 0x3C, 0xD9, 0x8B, 0xFF,                       // jmp haloreach.dll+5D5B5C ; jmp back
    ];

    // println!("Code cave: {:x?}", code_cave);
    let cave_location = game.base + 0xD181AD;
    game.write_bytes(cave_location, code_cave);

    let hook: Vec<u8> = vec![
        0xE9, 0x51, 0x26, 0x74, 0x0,            // jmp haloreach.dll+D181AD ; jump to code cave
        0x90,                                   // nop
    ];

    // println!("Hook: {:x?}", hook);
    let hook_location = game.base + 0x5D5B57;
    game.write_bytes(hook_location, hook);

    println!("Invincible!");
}