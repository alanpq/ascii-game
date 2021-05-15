pub(crate) mod curses;
pub mod drawer;

use pancurses::{ToChtype, Window, chtype, Input};

pub trait Renderer {
    fn new() -> Self;

    fn init(&mut self);
    fn plot(&self, x: i32, y: i32, chr: char);

    fn getch(&self) -> Option<Input>;

    fn printw<T: AsRef<str>>(&self, string: T) -> i32;

    fn mvaddch<T: ToChtype>(&self, y: i32, x: i32, ch: T) -> i32;
    fn mvaddstr<T: AsRef<str>>(&self, y: i32, x: i32, string: T) -> i32;

    fn mvinch(&self, y: i32, x: i32) -> chtype;
    fn mvprintw<T: AsRef<str>>(&self, y: i32, x: i32, string: T) -> i32;

    fn refresh(&self) -> i32;
}