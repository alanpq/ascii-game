#[macro_use]
extern crate log;
#[macro_use]
extern crate bitflags;

extern crate cfg_if;
extern crate pancurses;
extern crate num_integer;

pub mod renderer;
pub mod util;
pub mod ui;

mod app;
use app::App;
use crate::renderer::curses::CursesRenderer;
use crate::renderer::Renderer;
use simple_logger::SimpleLogger;
use log::LevelFilter;

fn main() {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    let mut renderer = CursesRenderer::new();
    renderer.init();
    let mut app = App::new(&renderer);

    loop {
        app.update(&mut renderer);
    }
}