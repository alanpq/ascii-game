extern crate pancurses;

mod renderer;

use pancurses::{ALL_MOUSE_EVENTS, Window, endwin, newwin, getmouse, initscr, mousemask, Input, resize_term, REPORT_MOUSE_POSITION, ToChtype, Attribute, curs_set, cbreak, noecho, chtype};
use crate::renderer::curses::CursesRenderer;
use crate::renderer::{Renderer, drawer::Drawer};


fn main() {
    let mut renderer = CursesRenderer::new();

    renderer.init();

    renderer.printw("Click in the terminal, press q to exit\n");
    renderer.refresh();

    let mut x0 = 0;
    let mut y0 = 0;
    let mut x1 = 0;
    let mut y1 = 0;
    let mut toggle = false;

    let mut mx = 0;
    let mut my = 0;
    let mut mc:chtype = 'd' as chtype;

    loop {
        match renderer.getch() {
            Some(Input::KeyResize) => {
                resize_term(0,0);
            }
            Some(Input::KeyMouse) => {
                if let Ok(mouse_event) = getmouse() {
                    renderer.mvaddch(my, mx, mc);
                    mx = mouse_event.x;
                    my = mouse_event.y;
                    mc = renderer.mvinch(my, mx);
                    renderer.mvaddch(mouse_event.y, mouse_event.x, 'O');

                    renderer.mvprintw(1, 0,
                      &format!("Mouse at {},{}\n{}", mouse_event.x, mouse_event.y, mouse_event.bstate),
                    );

                    if !(mouse_event.bstate == 4 || mouse_event.bstate == 1)  {
                        continue;
                    }
                    toggle = !toggle;
                    renderer.plot(mouse_event.x, mouse_event.y, 'x');
                    if toggle {
                        x0 = mouse_event.x;
                        y0 = mouse_event.y;
                    } else {
                        x1 = mouse_event.x;
                        y1 = mouse_event.y;
                        renderer.draw_line(x0, y0, x1, y1);
                    }

                };
            }
            Some(Input::Character(x)) if x == 'q' => break,
            _ => (),
        }
    }
    endwin();
}