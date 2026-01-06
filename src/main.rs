mod app;
mod commands;
mod domain;
mod errors;
mod repository;
mod ui;

use app::App;

fn main() {
    if let Err(e) = App::new().run() {
        eprintln!("Application Error: {:?}", e);
    }
}
