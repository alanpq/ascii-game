extern crate pancurses;

use pancurses::{ALL_MOUSE_EVENTS, Window, endwin, newwin, getmouse, initscr, mousemask, Input, resize_term, REPORT_MOUSE_POSITION, ToChtype, Attribute, curs_set, cbreak, noecho, chtype};

fn plot<T: ToChtype>(window: &Window, x: i32, y: i32, ch: T) {
    window.mvaddch(y, x, ch);
}

const line_chars: [char;12] = ['_','.','-','"','"','\'','-','_','/','|','\\','|'];

fn draw_line(window: &Window, x0: i32, y0: i32, x1: i32, y1: i32) {
    let mut x0 = x0;
    let mut y0 = y0;
    let x_diff = x1-x0;
    let y_diff = y1-y0;

    let slope = (y_diff as f32 / x_diff as f32).abs();

    let dx = (x_diff).abs();
    let sx = if x0<x1 { 1 } else { -1 };
    let dy = -(y_diff).abs();
    let sy = if y0<y1 { 1 } else { -1 };
    let mut err = dx + dy;

    window.mvaddstr(y0 - sy, x0, format!("slope: {}", slope));

    loop {
        let e2 = 2 * err;
        if slope == 0.0 {
            plot(window, x0, y0, if dx == 0 { '|' } else { '-' });
        } else if slope < 1.0 {
            let idx = ((e2) / dx) + 1;
            if idx >= 0 && idx < 4 as i32 {
                println!("{}", idx);
                if sy == 1 {
                    plot(window, x0, y0, line_chars[idx as usize]);
                    //plot(window, x0, y0, idx.to_string().chars().next().unwrap());
                } else {
                    plot(window, x0, y0, line_chars[idx as usize+4]);
                    //plot(window, x0, y0, (3-idx).to_string().chars().next().unwrap());
                }
            } else {
                println!("{}   !!!!!!", idx);
                plot(window, x0, y0, idx.to_string().chars().next().unwrap());
            }
        } else {
            let idx = (err / dy)+8;
            if idx >= 0 && idx < line_chars.len() as i32 {
                println!("{}", idx);
                if sx == sy {
                    plot(window, x0, y0, line_chars[idx as usize + 2]);
                } else {
                    plot(window, x0, y0, line_chars[idx as usize]);
                }
            } else {
                println!("{}   !!!!!!", idx);
                plot(window, x0, y0, idx.to_string().chars().next().unwrap());
            }
        }



        if x0 == x1 && y0 == y1 { break; }
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn main() {
    let window = initscr();

    cbreak();
    noecho();
    curs_set(0);

    window.nodelay(true);
    window.keypad(true); // Set keypad mode
    mousemask(ALL_MOUSE_EVENTS | REPORT_MOUSE_POSITION, std::ptr::null_mut()); // Listen to all mouse events

    window.printw("Click in the terminal, press q to exit\n");
    window.refresh();

    let mut x0 = 0;
    let mut y0 = 0;
    let mut x1 = 0;
    let mut y1 = 0;
    let mut toggle = false;

    let mut mx = 0;
    let mut my = 0;
    let mut mc:chtype = 'd' as chtype;

    loop {
        match window.getch() {
            Some(Input::KeyResize) => {
                resize_term(0,0);
            }
            Some(Input::KeyMouse) => {
                if let Ok(mouse_event) = getmouse() {
                    window.mvaddch(my, mx, mc);
                    mx = mouse_event.x;
                    my = mouse_event.y;
                    mc = window.mvinch(my, mx);
                    window.mvaddch(mouse_event.y, mouse_event.x, 'O');

                    if !(mouse_event.bstate == 4 || mouse_event.bstate == 1)  {
                        continue;
                    }
                    window.mvprintw(1, 0,
                                    &format!("Mouse at {},{}\n{}", mouse_event.x, mouse_event.y, mouse_event.bstate),
                    );
                    toggle = !toggle;
                    plot(&window, mouse_event.x, mouse_event.y, 'x');
                    if toggle {
                        x0 = mouse_event.x;
                        y0 = mouse_event.y;
                    } else {
                        x1 = mouse_event.x;
                        y1 = mouse_event.y;
                        draw_line(&window, x0, y0, x1, y1);

                        draw_line(&window, x0, y0-3, x1, (y0-3) - (y1-y0));
                    }

                };
            }
            Some(Input::Character(x)) if x == 'q' => break,
            _ => (),
        }
    }
    endwin();
}