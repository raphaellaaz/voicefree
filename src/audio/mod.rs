// src/audio/mod.rs

pub mod cpal_mngr;
pub mod pipewire_mngr;

pub fn init_audio_system() {

    cpal_mngr::initialize_cpal();
    pipewire_mngr::initialize_pipewire();
    println!("Inicializando el sistema de audio...");
}
