use crate::renderer::Renderer;
use pancurses::{ALL_MOUSE_EVENTS, Window, endwin, newwin, getmouse, initscr, mousemask, Input, resize_term, REPORT_MOUSE_POSITION, ToChtype, Attribute, curs_set, cbreak, noecho, chtype, mouseinterval};

pub struct CursesRenderer {
    pub window: Window,
}

impl Renderer for CursesRenderer {
    fn new() -> CursesRenderer {
        CursesRenderer {
            window: initscr(),
        }
    }

    fn init(&mut self) {
        cbreak();
        noecho();
        curs_set(0);

        mouseinterval(0); // disable click resolution (that shit smells)

        self.window.nodelay(true);
        self.window.keypad(true); // Set keypad mode
        mousemask(ALL_MOUSE_EVENTS | REPORT_MOUSE_POSITION, std::ptr::null_mut()); // Listen to all mouse events
    }

    fn kill(&mut self) {
        endwin();
    }

    fn plot(&self, x: i32, y: i32, chr: char) {
        self.window.mvaddch(y, x, chr);
    }

    fn erase(&self) {
        self.window.erase();
    }

    fn getch(&self) -> Option<Input> {
        self.window.getch()
    }

    fn printw<T: AsRef<str>>(&self, string: T) -> i32 {
        self.window.printw(string)
    }

    fn mvaddch<T: ToChtype>(&self, y: i32, x: i32, ch: T) -> i32 {
        self.window.mvaddch(y, x, ch)
    }

    fn mvaddstr<T: AsRef<str>>(&self, y: i32, x: i32, string: T) -> i32 {
        self.window.mvaddstr(y, x, string)
    }

    fn mvinch(&self, y: i32, x: i32) -> u64 {
        self.window.mvinch(y, x)
    }

    fn mvprintw<T: AsRef<str>>(&self, y: i32, x: i32, string: T) -> i32 {
        self.window.mvprintw(y, x, string)
    }

    fn refresh(&self) -> i32 {
        self.window.refresh()
    }
}