extern crate pancurses;
extern crate num_integer;

mod renderer;
mod game;
mod util;

use pancurses::{ALL_MOUSE_EVENTS, Window, endwin, newwin, getmouse, initscr, mousemask, Input, resize_term, REPORT_MOUSE_POSITION, ToChtype, Attribute, curs_set, cbreak, noecho, chtype, BUTTON1_RELEASED, COLOR_BLACK, COLOR_PAIR, COLOR_GREEN};
use crate::renderer::curses::CursesRenderer;
use crate::renderer::{Renderer, drawer::Drawer};

use game::map::Map;
use crate::game::map::{CHUNK_SIZE, CHUNK_SIZE_I32, Tile};
use num_integer::Integer;
use crate::util::fps::FpsCounter;



fn main() {
    let mut renderer = CursesRenderer::new();

    renderer.init();

    renderer.printw("Click in the terminal, press q to exit\n");
    renderer.refresh();

    let mut map = Map::new();

    let mut ss = renderer.dimensions();

    let mut paused = false;

    let mut camera_x = 0;
    let mut camera_y = 0;

    let mut player_x = 0;
    let mut player_y = 0;


    let mut x0 = 0;
    let mut y0 = 0;
    let mut toggle = false;

    let mut mx = 0;
    let mut my = 0;
    let mut mc:chtype = ' ' as chtype;

    let mut old_m = BUTTON1_RELEASED;

    let mut fps_c = FpsCounter::new();
    let mut have_moved = false;

    loop {
        let frame_time = fps_c.tick();
        match renderer.getch() {
            Some(Input::KeyResize) => {
                resize_term(0, 0); // TODO: abstract this away
                ss = renderer.dimensions();
            }
            Some(Input::KeyMouse) => {
                if let Ok(mouse_event) = getmouse() {
                    //renderer.mvaddch(my, mx, mc);
                    mx = mouse_event.x;
                    my = mouse_event.y;
                    //mc = renderer.mvinch(my, mx);
                    //renderer.mvaddch(mouse_event.y, mouse_event.x, 'O');

                    if mouse_event.bstate != old_m {
                        println!("transition from mouse {} to mouse {}", old_m, mouse_event.bstate);
                        old_m = mouse_event.bstate;
                    }

                    if mouse_event.bstate & BUTTON1_RELEASED == BUTTON1_RELEASED {
                        toggle = !toggle;
                        renderer.plot(mouse_event.x, mouse_event.y, 'x');
                        if toggle {
                            x0 = mouse_event.x;
                            y0 = mouse_event.y;
                        } else {
                            renderer.draw_line(x0, y0, mouse_event.x, mouse_event.y);
                        }
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
                        player_y -= 1;
                        have_moved = true;
                    },
                    115 => { // s
                        player_y += 1;
                        have_moved = true;
                    },
                    97 => { // a
                        player_x -= 1;
                        have_moved = true;
                    },
                    100 => { // d
                        player_x += 1;
                        have_moved = true;
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

        //renderer.erase();

        if paused {
            renderer.mvaddstr(ss.0/2,ss.1/2,"PAUSED");
            continue;
        }

        let center_x = ss.0.div_floor(&2);
        let center_y = ss.1.div_floor(&2);

        let cam_off_x = player_x - center_x;
        let cam_off_y = player_y - center_y;

        let start_x = cam_off_x.div_floor(&CHUNK_SIZE_I32);
        let start_y = cam_off_y.div_floor(&CHUNK_SIZE_I32);

        if have_moved {
            for i in start_x..start_x + ss.0.div_ceil(&CHUNK_SIZE_I32) + 1 {
                for j in start_y..start_y + ss.1.div_ceil(&CHUNK_SIZE_I32) + 1 {
                    let chunk = map.get_chunk_or_create(i, j);
                    for x in 0..CHUNK_SIZE {
                        for y in 0..CHUNK_SIZE {
                            let tile = &chunk.tiles[x][y];
                            let chr;
                            match tile {
                                Tile::Air => {
                                    chr = ' ';
                                    renderer.window.attrset(COLOR_PAIR(8));
                                },
                                Tile::Wall => {
                                    chr = ' ';
                                    renderer.window.attrset(COLOR_PAIR(2));
                                },
                            }
                            renderer.plot(i * CHUNK_SIZE_I32 + x as i32 - cam_off_x, j * CHUNK_SIZE_I32 + y as i32 - cam_off_y, chr);
                        }
                    }
                }
            }
            have_moved = false;
            renderer.window.attrset(COLOR_PAIR(7));
            renderer.plot(center_x, center_y, '@' as chtype);
        }

        renderer.mvaddstr(0, 0, format!("frametime: {} ({} fps)", frame_time, 1.0/frame_time));
        renderer.mvaddstr(1, 0, format!("{},{}", ss.0, ss.1));
        renderer.mvaddstr(2, 0, format!("{},{}", (ss.0.div_ceil(&CHUNK_SIZE_I32)), (ss.1.div_ceil(&CHUNK_SIZE_I32))));

    }
    renderer.kill();
}