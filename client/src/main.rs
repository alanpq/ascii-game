#[macro_use]
extern crate log;
#[macro_use]
extern crate bitflags;

extern crate cfg_if;
extern crate pancurses;
extern crate num_integer;

#[macro_use]
extern crate glium;

pub mod renderer;
pub mod util;
pub mod ui;

mod app;
use app::App;
use crate::renderer::curses::CursesRenderer;
use crate::renderer::Renderer;
use simple_logger::SimpleLogger;
use log::LevelFilter;
use crate::renderer::glium::GlRenderer;
use glium::{glutin, Surface};

fn main() {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    let mut renderer = GlRenderer::new();
    renderer.init();
    let mut app = App::new(&renderer);

    // renderer.run();
}