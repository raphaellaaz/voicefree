use slint::include_modules;
extern crate pipewire;

mod audio;
mod virtual_device;
mod config;
mod utils;

include_modules!();

fn main() {

    // Inicializar el sistema de audio
    audio::init_audio_system();

    // Crear un dispositivo virtual de audio
    virtual_device::virtual_mngr::create_virtual_device();

    let ui = MainWindow::new().unwrap();
    ui.run().unwrap();

    println!("Aplicación de audio iniciada.");
}
