pub(crate) mod curses;
pub mod opengl;
pub mod drawer;
mod event;

use pancurses::{ToChtype, Window, chtype, Input};

pub use event::Event;
use glium::{Frame, Display};
use std::rc::Rc;
use std::cell::RefCell;

pub trait Renderer {
    fn new() -> Self;

    //fn frame_buffer(&self) -> Frame;
    fn dimensions(&self, display: &Display) -> (i32, i32);

    fn init(&mut self, display: &Display);

    fn kill(&mut self);
    fn plot<T: ToChtype>(&self, display: Rc<Display>, frame: Rc<RefCell<Frame>>, x: i32, y: i32, chr: T);

    fn erase(&self);

    fn getch(&self) -> Option<Event>;

    fn printw<T: AsRef<str>>(&self, string: T) -> i32;

    fn mvaddch<T: ToChtype>(&self, y: i32, x: i32, ch: T) -> i32;
    fn mvaddstr<T: AsRef<str>>(&self, display: Rc<Display>, frame: Rc<RefCell<Frame>>, y: i32, x: i32, string: T) -> i32;

    fn mvinch(&self, y: i32, x: i32) -> chtype;
    fn mvprintw<T: AsRef<str>>(&self, y: i32, x: i32, string: T) -> i32;

    // BOTCH JOB INCOMING
    fn attrset<T: Into<chtype>>(&self, attributes: T);

    fn refresh(&self) -> i32;
}