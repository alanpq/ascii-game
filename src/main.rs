extern crate pancurses;

mod renderer;

use pancurses::{ALL_MOUSE_EVENTS, Window, endwin, newwin, getmouse, initscr, mousemask, Input, resize_term, REPORT_MOUSE_POSITION, ToChtype, Attribute, curs_set, cbreak, noecho, chtype, BUTTON1_RELEASED};
use crate::renderer::curses::CursesRenderer;
use crate::renderer::{Renderer, drawer::Drawer};

fn main() {
    let mut renderer = CursesRenderer::new();

    renderer.init();

    renderer.printw("Click in the terminal, press q to exit\n");
    renderer.refresh();

    let mut paused = false;


    let mut x0 = 0;
    let mut y0 = 0;
    let mut toggle = false;

    let mut mx = 0;
    let mut my = 0;
    let mut mc:chtype = ' ' as chtype;

    let mut old_m = BUTTON1_RELEASED;

    loop {
        match renderer.getch() {
            Some(Input::KeyResize) => {
                resize_term(0, 0);
            }
            Some(Input::KeyMouse) => {
                if let Ok(mouse_event) = getmouse() {
                    renderer.mvaddch(my, mx, mc);
                    mx = mouse_event.x;
                    my = mouse_event.y;
                    mc = renderer.mvinch(my, mx);
                    renderer.mvaddch(mouse_event.y, mouse_event.x, 'O');

                    if mouse_event.bstate != old_m {
                        println!("transition from mouse {} to mouse {}", old_m, mouse_event.bstate);
                        old_m = mouse_event.bstate;
                    }

                    if mouse_event.bstate & BUTTON1_RELEASED != BUTTON1_RELEASED {
                        continue;
                    }
                    toggle = !toggle;
                    renderer.plot(mouse_event.x, mouse_event.y, 'x');
                    if toggle {
                        x0 = mouse_event.x;
                        y0 = mouse_event.y;
                    } else {
                        renderer.draw_line(x0, y0, mouse_event.x, mouse_event.y);
                    }
                };
            }
            Some(Input::Character(chr)) => {
                println!("chr {} -> {}",chr, chr as u32);
                match chr as u32 {
                    113 => break, // q
                    27 => { // escape
                        paused = !paused;
                    },
                    119 => { // w

                    },
                    115 => { // s

                    },
                    97 => { // a

                    },
                    100 => { // d

                    },
                    _ => {},
                }
            },
            Some(Input::Unknown(x)) => {
                println!("unknown {}", x);
            },
            asd => {
                if asd.is_some() {
                    println!("{:?}", asd);
                }
            },
        }

        renderer.erase();

        if paused {
            renderer.mvaddstr(0,0,"PAUSED");
        }

    }
    renderer.kill();
}