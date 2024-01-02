// disable console on windows for release builds
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    lib_bevy_game::bevy_main();
}

