#[macro_use]
extern crate log;

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

fn main() {
    let mut renderer = CursesRenderer::new();
    renderer.init();
    let mut app = App::new(&renderer);

    loop {
        app.update(&mut renderer);
    }
}