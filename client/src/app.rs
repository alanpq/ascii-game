use pancurses::{ALL_MOUSE_EVENTS, Window, endwin, newwin, getmouse, initscr, mousemask, Input, resize_term, REPORT_MOUSE_POSITION, ToChtype, Attribute, curs_set, cbreak, noecho, chtype, BUTTON1_RELEASED, COLOR_BLACK, COLOR_PAIR, COLOR_GREEN};
use crate::renderer::curses::CursesRenderer;
use crate::renderer::{Renderer, drawer::Drawer, Event};
use crate::ui::{MenuState};

use num_integer::Integer;
use crate::util::fps::FpsCounter;

use log::*;

use std::{
    net::{IpAddr, SocketAddr},
    time::Duration,
};

use naia_client::{ClientConfig, ClientEvent, NaiaClient};

use ascii_game_shared::{
    get_shared_config, manifest_load, shared_behaviour, events::{AuthEvent,KeyCommand,ChatEvent}, Actors, Events,
    actors::{PointActorColor},
};
use ascii_game_shared::game::map::Map;
use ascii_game_shared::game::map::{CHUNK_SIZE, CHUNK_SIZE_I32, Tile};
use std::collections::HashMap;
use std::borrow::BorrowMut;
use std::cell::{RefCell, RefMut};
use std::net::ToSocketAddrs;
use std::io::empty;
use bitflags::_core::ops::Not;
use std::cmp::min;
use std::rc::Rc;
use glium::{Frame, Display};

const SERVER_PORT: u16 = 14191;

bitflags! {
    struct DebugFlags: u32 {
        const KEY = 0b00000001;
    }
}

pub struct App {
    client: Option<NaiaClient<Events, Actors>>,
    pawn_key: Option<u16>,
    queued_command: Option<KeyCommand>,

    map: Option<Map>,
    player_x: i32,
    player_y: i32,

    menu_counter: u8,
    menu_space: u8,
    menu_state: MenuState,
    menu_eaten: bool,
    menu_idx: u8,
    menu_press: bool,
    menu_unpress: bool,
    menu_text: String,

    chat_open: bool,
    chat_msg: Option<String>,
    chat_log: Vec<String>,

    debug_keys: DebugFlags,

    ip: String,

    ss: (i32, i32),
    fps_counter: FpsCounter,
    paused: bool,
}

impl App {
    pub fn new<T: Renderer>(renderer: &T) -> Self {
        info!("Naia Macroquad Client Example Started");

        App {
            client: None,
            pawn_key: None,
            queued_command: None,

            map: None,
            player_x: 0,
            player_y: 0,

            menu_counter: 0,
            menu_space: 0,
            menu_state: MenuState::Main,
            menu_eaten: false,
            menu_idx: 0,
            menu_press: false,
            menu_unpress: false,
            menu_text: String::new(),

            chat_open: false,
            chat_msg: None,
            chat_log: Vec::new(),

            debug_keys: DebugFlags::KEY.not(),

            ip: String::new(),

            ss: (0,0),
            fps_counter: FpsCounter::new(),
            paused: false,
        }
    }

    pub fn connect<T: AsRef<str>>(&mut self, ip: T) {
        let split = ip.as_ref().split(":");
        let final_ip;
        match split.count() {
            1 => {
                final_ip = format!("{}:{}",ip.as_ref(), SERVER_PORT)
            },
            2 => {
                final_ip = ip.as_ref().to_string();
            },
            _ => {
                return;
            }
        }
        info!("{}", final_ip);
        let server_socket_address = final_ip
            .to_socket_addrs()
            .expect("couldn't parse input IP address").next().expect("still couldn't parse hostname");
        let mut client_config = ClientConfig::default();
        client_config.heartbeat_interval = Duration::from_secs(2);
        client_config.disconnection_timeout_duration = Duration::from_secs(10000);

        let auth = Events::AuthEvent(AuthEvent::new("charlie", "12345"));

        let client = NaiaClient::new(
            server_socket_address,
            manifest_load(),
            Some(client_config),
            get_shared_config(),
            Some(auth),
        );
        self.client = Some(client);
    }

    pub fn update<T: Renderer>(&mut self, display: Rc<Display>, frame: Rc<RefCell<Frame>>, renderer_ref: Rc<RefCell<T>>, event: Option<Event>) {
        let frame_time = self.fps_counter.tick();
        match event {
            Some(Event::Character(chr)) => {
                if self.debug_keys & DebugFlags::KEY == DebugFlags::KEY {
                    info!("chr {} -> {}", chr, chr as u32);
                }
                let mut w = false;
                let mut s = false;
                let mut a = false;
                let mut d = false;
                match chr as u32 {
                    27 => { // escape
                        self.paused = !self.paused;
                        if self.menu_eaten {
                            self.menu_eaten = false;
                            self.menu_unpress = true;
                        }
                        if self.chat_open {
                            self.menu_text.clear();
                        }
                        self.chat_open = false;
                    },
                    119 => { // w
                        w = true;
                    },
                    115 => { // s
                        s = true;
                    },
                    97 => { // a
                        a = true;
                    },
                    100 => { // d
                        d = true;
                    },
                    107 => {
                        self.debug_keys.toggle(DebugFlags::KEY);
                    },
                    32 => { // space
                        if self.menu_state != MenuState::Game && !self.menu_eaten {
                            self.menu_press = true;
                        }
                    },
                    10 => { // enter
                        if self.menu_state != MenuState::Game {
                            if !self.menu_eaten {
                                self.menu_press = true;
                            }
                        } else {
                            if self.chat_open {
                                self.chat_msg = Some(self.menu_text.clone());
                                self.menu_text.clear();
                            }
                            self.chat_open = !self.chat_open;
                            self.menu_eaten = self.chat_open;
                        }
                    }
                    _ => {},
                }
                if self.menu_state != MenuState::Game && !self.menu_eaten {
                    self.menu_idx = self.menu_idx.saturating_add(s as u8);
                    self.menu_idx = self.menu_idx.saturating_sub(w as u8);
                }
                if self.menu_eaten {
                    match chr as u32 {
                        8 => { // backspace
                            self.menu_text.pop();
                        }
                        10 => { // enter
                            if !self.chat_open {
                                self.menu_eaten = false;
                                self.menu_unpress = true;
                            }
                        }
                        _ => {
                            self.menu_text.push(chr);
                        }
                    }
                }
                if w || s || a || d {
                    self.queued_command = Some(KeyCommand::new(w, s, a, d));
                }
            },
            Some(Event::WindowResize) => {
                (*renderer_ref).borrow_mut().resize(display.as_ref());
            }
            _ => {},
        }

        if let Some(client) = &mut self.client {
            while let Some(result) = client.receive() {
                match result {
                    Ok(event) => match event {
                        ClientEvent::Connection => {
                            info!("Connected to {}", client.server_address());
                            // self.change_menu(MenuState::Game);
                            self.menu_state = MenuState::Game;
                            self.menu_idx = 0;
                        },
                        ClientEvent::Disconnection => {
                            info!("Disconnected from {}", client.server_address());
                            // self.change_menu(MenuState::Connect);
                            self.menu_state = MenuState::Connect;
                            self.menu_idx = 0;
                        }
                        ClientEvent::AssignPawn(local_key) => {
                            self.pawn_key = Some(local_key);
                            info!("assign pawn");
                        }
                        ClientEvent::UnassignPawn(_) => {
                            self.pawn_key = None;
                            info!("unassign pawn");
                        }
                        ClientEvent::CreateActor(actor_key) => {
                            if let Some(actor) = client.get_actor(&actor_key) {
                                match actor {
                                    Actors::WorldActor(world_actor) => {
                                        info!("got world update at key {}", actor_key);
                                        self.map = Some(Map::new(*(world_actor.as_ref().borrow().seed.get())));
                                        info!("new seed: {}", self.map.as_ref().unwrap().seed);
                                    },
                                    _ => {}
                                }
                            }
                        }
                        ClientEvent::Tick => {
                            if let Some(pawn_key) = self.pawn_key {
                                if let Some(command) = self.queued_command.take() {
                                    client.send_command(pawn_key, &command);
                                }
                                if let Some(msg) = &mut self.chat_msg {                                                                         info!("sending chat: '{}'", msg);
                                    let cmd = ChatEvent::new(msg);
                                    client.send_event(&cmd);
                                    self.chat_msg = None;
                                }
                            }
                        }
                        ClientEvent::Command(pawn_key, command_type) => {
                            if let Events::KeyCommand(key_command) = command_type {
                                if let Some(typed_actor) = client.get_pawn_mut(&pawn_key) {
                                    if let Actors::PointActor(actor) = typed_actor {
                                        shared_behaviour::process_command(&key_command, actor);
                                        self.player_x = *(actor.as_ref().borrow().x.get());
                                        self.player_y = *(actor.as_ref().borrow().y.get());
                                    }
                                }
                            }
                        },
                        ClientEvent::Event(event) => {
                            if let Events::ChatEvent(event) = event {
                                info!("chat received: '{}'", event.body.get());
                                self.chat_log.push(event.body.get().clone());
                            }
                        }
                        _ => {},
                    },
                    Err(err) => {
                        info!("Client Error: {}", err);
                    }
                }
            }
        }

        self.draw_ui(display.clone(), frame.clone(), renderer_ref.clone(), frame_time);
        // renderer.plot(display.clone(), frame.clone(), 0, 0, 'a');

        // self.frame.take().unwrap().finish().unwrap();
    }

    fn draw_game<T: Renderer>(&mut self, display: Rc<Display>, frame: Rc<RefCell<Frame>>, renderer_ref: Rc<RefCell<T>>, frame_time: f32) {
        let renderer = (*renderer_ref).borrow();
        if let Some(client) = &mut self.client {
            if client.has_connection() {
                let center_x = self.ss.0.div_floor(&2);
                let center_y = self.ss.1.div_floor(&2);

                let cam_off_x = self.player_x - center_x;
                let cam_off_y = self.player_y - center_y;

                self.draw_map(display.clone(), frame.clone(), renderer_ref.clone(), cam_off_x, cam_off_y);

                renderer.attrset(COLOR_PAIR(7));
                renderer.plot(display.clone(),frame.clone(), center_x, center_y, '@' as chtype);

                self.draw_others(display.clone(), frame.clone(),renderer_ref.clone(), cam_off_x, cam_off_y);
            } else {}

            renderer.mvaddstr(display.clone(),frame.clone(),0, 0, format!("frametime: {} ({} fps)", frame_time, 1.0 / frame_time));
            renderer.mvaddstr(display.clone(),frame.clone(),1, 0, format!("{},{}", self.ss.0, self.ss.1));
        } else {}
    }

    fn draw_button<T: Renderer, U: AsRef<str>>(&mut self, display: Rc<Display>, frame: Rc<RefCell<Frame>>, renderer_ref: Rc<RefCell<T>>, txt: U) -> bool {
        let renderer = (*renderer_ref).borrow();
        if self.menu_counter == self.menu_idx {
            renderer.attrset(COLOR_PAIR(7));
            if self.menu_press {
                self.menu_press = false;
                return true;
            }
        } else {
            renderer.attrset(COLOR_PAIR(8));
        }
        renderer.mvaddstr(display.clone(),frame.clone(), self.menu_counter as i32 + self.menu_space as i32, 0, txt);
        self.menu_counter += 1;
        false
    }

    fn draw_label<T: Renderer, U: AsRef<str>>(&mut self, display: Rc<Display>, frame: Rc<RefCell<Frame>>,  renderer_ref: Rc<RefCell<T>>, text: U) {
        let renderer = (*renderer_ref).borrow();
        renderer.attrset(COLOR_PAIR(8));
        renderer.mvaddstr(display.clone(),frame.clone(), self.menu_counter as i32 + self.menu_space as i32, 0, text);
    }

    fn draw_input<T: Renderer, U: AsRef<str>>(&mut self, display: Rc<Display>, frame: Rc<RefCell<Frame>>,  renderer_ref: Rc<RefCell<T>>, label: U, value: U) -> String {
        let renderer = (*renderer_ref).borrow();
        let input_txt;
        if self.menu_counter == self.menu_idx {
            renderer.attrset(COLOR_PAIR(7));
            if self.menu_press {
                self.menu_press = false;
                self.menu_eaten = true;
            }
            if self.menu_unpress {
                self.menu_unpress = false;
            }
            if self.menu_eaten {
                input_txt = self.menu_text.as_ref();
            } else {
                input_txt = value.as_ref();
            }
        } else {
            renderer.attrset(COLOR_PAIR(8));
            input_txt = value.as_ref();
        }
        renderer.mvaddstr(display.clone(),frame.clone(), self.menu_counter as i32, 0, label.as_ref());

        if self.menu_eaten && self.menu_counter == self.menu_idx {
            renderer.attrset(COLOR_PAIR(7));
        } else {
            renderer.attrset(COLOR_PAIR(8));
        }
        if input_txt.is_empty() {
            renderer.mvaddstr(display.clone(),frame.clone(), self.menu_counter as i32 + self.menu_space as i32, 1 + label.as_ref().len() as i32, "   ");
        } else {
            renderer.mvaddstr(display.clone(),frame.clone(), self.menu_counter as i32 + self.menu_space as i32, 1 + label.as_ref().len() as i32, input_txt);
        }
        self.menu_counter += 1;
        input_txt.to_string()
    }

    fn change_menu(&mut self, new_state: MenuState) {
        self.menu_state = new_state;
        self.menu_idx = 0;
    }

    fn draw_ui<T: Renderer>(&mut self, display: Rc<Display>, frame: Rc<RefCell<Frame>>,  renderer_ref: Rc<RefCell<T>>, frame_time: f32) {
        let renderer = (*renderer_ref).borrow();
        renderer.erase();
        match self.menu_state {
            MenuState::Main => {
                if self.draw_button(display.clone(),frame.clone(),renderer_ref.clone(), "CONNECT") {
                    self.change_menu(MenuState::Connect);
                }
                self.draw_button(display.clone(),frame.clone(),renderer_ref.clone(), "EXIT");
            }
            MenuState::Connect => {
                self.ip = self.draw_input(display.clone(),frame.clone(),renderer_ref.clone(), "IP:", &self.ip.clone());

                if self.draw_button(display.clone(),frame.clone(),renderer_ref.clone(), "CONNECT") {
                    self.connect(self.ip.clone());
                    self.change_menu(MenuState::Connecting);
                }

                self.menu_space += 1;
                if self.draw_button(display.clone(),frame.clone(),renderer_ref.clone(), "BACK") {
                    self.change_menu(MenuState::Connect);
                }
            }
            MenuState::Connecting => {
                if let Some(client) = &self.client {
                    if client.has_connection() {
                        //self.change_menu(MenuState::Game);
                        self.draw_label(display.clone(),frame.clone(), renderer_ref.clone(), ":)");
                    } else {
                        self.draw_label(display.clone(),frame.clone(), renderer_ref.clone(), "CONNECTING...");
                        self.menu_space += 1;
                        if self.draw_button(display.clone(),frame.clone(), renderer_ref.clone(), "CANCEL") {
                            self.client = None;
                            self.menu_state = MenuState::Connect;
                            self.menu_idx = 0;
                        }
                    }
                }
            }
            MenuState::Game => {
                self.draw_game(display.clone(),frame.clone(), renderer_ref.clone(), frame_time);
                renderer.mvaddstr(display.clone(),frame.clone(), self.ss.1-1, 0, &self.menu_text);

                let len = self.chat_log.len();
                if len > 0 {
                    let mut j = 0;
                    for i in len - min(len, 10)..len {
                        if let Some(str) = self.chat_log.get(i) {
                            renderer.mvaddstr(display.clone(),frame.clone(), self.ss.1 - 11 + j, 0, str);
                            j+=(str.len() as i32).div_ceil(&self.ss.0);
                        }
                    }
                }
            }
        }
        self.menu_counter = 0;
        self.menu_space = 0;
        self.menu_unpress = false;
        self.menu_press = false;
    }

    fn draw_map<T: Renderer>(&mut self, display: Rc<Display>, frame: Rc<RefCell<Frame>>, renderer_ref: Rc<RefCell<T>>, cam_off_x: i32, cam_off_y: i32) {
        let renderer = (*renderer_ref).borrow();
        if let Some(map) = &mut self.map {
            let start_x = cam_off_x.div_floor(&CHUNK_SIZE_I32);
            let start_y = cam_off_y.div_floor(&CHUNK_SIZE_I32);

            for i in start_x..start_x + self.ss.0.div_ceil(&CHUNK_SIZE_I32) + 1 {
                for j in start_y..start_y + self.ss.1.div_ceil(&CHUNK_SIZE_I32) + 1 {
                    let chunk = map.get_chunk_or_create(i, j);
                    for x in 0..CHUNK_SIZE {
                        for y in 0..CHUNK_SIZE {
                            let tile = &chunk.tiles[x][y];
                            let chr;
                            match tile {
                                Tile::Air => {
                                    chr = ' ';
                                    renderer.attrset(COLOR_PAIR(8));
                                },
                                Tile::Wall => {
                                    chr = ' ';
                                    renderer.attrset(COLOR_PAIR(2));
                                },
                            }
                            renderer.plot(display.clone(),frame.clone(), i * CHUNK_SIZE_I32 + x as i32 - cam_off_x, j * CHUNK_SIZE_I32 + y as i32 - cam_off_y, chr);
                        }
                    }
                }
            }
        }
    }

    fn draw_others<T: Renderer>(&mut self, display: Rc<Display>, frame: Rc<RefCell<Frame>>,  renderer_ref: Rc<RefCell<T>>, cam_off_x: i32, cam_off_y: i32) {
        let renderer = (*renderer_ref).borrow();
        if let Some(client) = &mut self.client {
            for actor_key in client.actor_keys().unwrap() {
                if actor_key == self.pawn_key.unwrap_or(0) { continue; }
                if let Some(actor) = client.get_actor(&actor_key) {
                    match actor {
                        Actors::PointActor(point_actor) => {
                            let x = *(point_actor.as_ref().borrow().x.get());
                            let y = *(point_actor.as_ref().borrow().y.get());
                            renderer.plot(display.clone(),frame.clone(), x - cam_off_x, y - cam_off_y, 'O');
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}