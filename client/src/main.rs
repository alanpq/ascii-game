#[macro_use]
extern crate log;
#[macro_use]
extern crate bitflags;

extern crate cfg_if;
extern crate pancurses;
extern crate num_integer;

#[macro_use]
extern crate glium;

extern crate image;

pub mod renderer;
pub mod util;
pub mod ui;

mod app;
use app::App;
use crate::renderer::curses::CursesRenderer;
use crate::renderer::Renderer;
use simple_logger::SimpleLogger;
use log::LevelFilter;
use crate::renderer::opengl::GlRenderer;
use glium::{glutin, Surface};
use std::cell::RefCell;
use std::rc::Rc;
use crate::renderer::opengl::context::GlContext;

fn main() {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    let mut context = GlContext::new();
    let mut renderer = GlRenderer::new();

    let mut app = App::new(&renderer);
    let renderer_ref = Rc::new(RefCell::new(renderer));
    context.run(renderer_ref.clone(), |display, frame, event| {
        app.update(display, frame, renderer_ref.clone(), event);
    });

    // renderer.run();
}