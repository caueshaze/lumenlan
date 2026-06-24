// Evita abrir um console extra no Windows em build release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    lumenlan_lib::run();
}
