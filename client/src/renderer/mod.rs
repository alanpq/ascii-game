pub(crate) mod curses;
pub mod glium;
pub mod drawer;

use pancurses::{ToChtype, Window, chtype, Input};

pub trait Renderer {
    fn new() -> Self;

    fn dimensions(&self) -> (i32, i32);

    fn init(&mut self);
    fn kill(&mut self);
    fn plot<T: ToChtype>(&self, x: i32, y: i32, chr: T);

    fn erase(&self);

    fn getch(&self) -> Option<Input>;

    fn printw<T: AsRef<str>>(&self, string: T) -> i32;

    fn mvaddch<T: ToChtype>(&self, y: i32, x: i32, ch: T) -> i32;
    fn mvaddstr<T: AsRef<str>>(&self, y: i32, x: i32, string: T) -> i32;

    fn mvinch(&self, y: i32, x: i32) -> chtype;
    fn mvprintw<T: AsRef<str>>(&self, y: i32, x: i32, string: T) -> i32;

    // BOTCH JOB INCOMING
    fn attrset<T: Into<chtype>>(&self, attributes: T);

    fn refresh(&self) -> i32;
}