use crate::renderer::Renderer;
use pancurses::{ALL_MOUSE_EVENTS, Window, endwin, newwin, getmouse, initscr, mousemask, Input, resize_term, REPORT_MOUSE_POSITION, ToChtype, Attribute, curs_set, cbreak, noecho, chtype, mouseinterval, has_colors, start_color, init_pair, COLOR_BLACK, COLOR_RED, COLOR_BLUE, COLOR_GREEN, COLOR_CYAN, COLOR_MAGENTA, COLOR_YELLOW, COLOR_WHITE};

pub struct CursesRenderer {
    pub window: Window,
}

const COLOR_TABLE: [i16; 8] = [
    COLOR_RED,
    COLOR_BLUE,
    COLOR_GREEN,
    COLOR_CYAN,
    COLOR_RED,
    COLOR_MAGENTA,
    COLOR_YELLOW,
    COLOR_WHITE,
];

impl Renderer for CursesRenderer {
    fn new() -> CursesRenderer {
        CursesRenderer {
            window: initscr(),
        }
    }

    fn dimensions(&self) -> (i32, i32) {
        (self.window.get_max_x(), self.window.get_max_y())
    }

    fn init(&mut self) {
        cbreak();
        noecho();
        curs_set(0);

        if has_colors() {
            start_color();
        }

        for (i, color) in COLOR_TABLE.iter().enumerate() {
            init_pair(i as i16, COLOR_BLACK, *color);
        }
        init_pair(8, COLOR_WHITE, COLOR_BLACK);

        mouseinterval(0); // disable click resolution (that shit smells)

        resize_term(0, 0);

        self.window.nodelay(true);
        self.window.keypad(true); // Set keypad mode
        mousemask(ALL_MOUSE_EVENTS | REPORT_MOUSE_POSITION, std::ptr::null_mut()); // Listen to all mouse events
    }

    fn kill(&mut self) {
        endwin();
    }

    fn plot<T: ToChtype>(&self, x: i32, y: i32, chr: T) {
        if x < 0 || y < 0 { return; }
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

    fn attrset<T: Into<chtype>>(&self, attributes: T) {
        self.window.attrset(attributes);
    }

    fn refresh(&self) -> i32 {
        self.window.refresh()
    }
}