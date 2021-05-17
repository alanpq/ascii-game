use pancurses::{ALL_MOUSE_EVENTS, Window, endwin, newwin, getmouse, initscr, mousemask, Input, resize_term, REPORT_MOUSE_POSITION, ToChtype, Attribute, curs_set, cbreak, noecho, chtype, BUTTON1_RELEASED, COLOR_BLACK, COLOR_PAIR, COLOR_GREEN};
use crate::renderer::curses::CursesRenderer;
use crate::renderer::{Renderer, drawer::Drawer};

use num_integer::Integer;
use crate::util::fps::FpsCounter;

use log::*;

use std::{
    net::{IpAddr, SocketAddr},
    time::Duration,
};

use naia_client::{ClientConfig, ClientEvent, NaiaClient};

use ascii_game_shared::{
    get_shared_config, manifest_load, shared_behaviour, AuthEvent, Actors, Events,
    KeyCommand, PointActorColor,
};
use ascii_game_shared::game::map::Map;
use ascii_game_shared::game::map::{CHUNK_SIZE, CHUNK_SIZE_I32, Tile};
use std::collections::HashMap;

const SERVER_PORT: u16 = 14191;

pub struct App {
    client: NaiaClient<Events, Actors>,
    pawn_key: Option<u16>,
    queued_command: Option<KeyCommand>,

    map: Option<Map>,
    player_x: i32,
    player_y: i32,

    ss: (i32, i32),
    fps_counter: FpsCounter,
    paused: bool,
}

impl App {
    pub fn new<T: Renderer>(renderer: &T) -> Self {
        println!("Naia Macroquad Client Example Started");

        // Put your Server's IP Address here!, can't easily find this automatically from the browser
        let server_ip_address: IpAddr = "192.168.1.16"
            .parse()
            .expect("couldn't parse input IP address");
        let server_socket_address = SocketAddr::new(server_ip_address, SERVER_PORT);

        let mut client_config = ClientConfig::default();
        client_config.heartbeat_interval = Duration::from_secs(2);
        client_config.disconnection_timeout_duration = Duration::from_secs(5);

        let auth = Events::AuthEvent(AuthEvent::new("charlie", "12345"));

        let client = NaiaClient::new(
            server_socket_address,
            manifest_load(),
            Some(client_config),
            get_shared_config(),
            Some(auth),
        );

        App {
            client,
            pawn_key: None,
            queued_command: None,

            map: None,
            player_x: 0,
            player_y: 0,

            ss: renderer.dimensions(),
            fps_counter: FpsCounter::new(),
            paused: false,
        }
    }

    pub fn update<T: Renderer>(&mut self, renderer: &mut T) {
        let frame_time = self.fps_counter.tick();
        match renderer.getch() {
            Some(Input::KeyResize) => {
                resize_term(0, 0); // TODO: abstract this away
                self.ss = renderer.dimensions();
            },
            Some(Input::Character(chr)) => {
                println!("chr {} -> {}",chr, chr as u32);
                let mut w = false;
                let mut s = false;
                let mut a = false;
                let mut d = false;
                match chr as u32 {
                    27 => { // escape
                        self.paused = !self.paused;
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
                    _ => {},
                }
                if w || s || a || d {
                    self.queued_command = Some(KeyCommand::new(w,s,a,d));
                }
            },
            _ => {}
        }

        while let Some(result) = self.client.receive() {
            match result {
                Ok(event) => match event {
                    ClientEvent::Connection => {
                        println!("Connected to {}", self.client.server_address());
                    },
                    ClientEvent::Disconnection => {
                        println!("Disconnected from {}", self.client.server_address());
                    }
                    ClientEvent::AssignPawn(local_key) => {
                        self.pawn_key = Some(local_key);
                        println!("assign pawn");
                    }
                    ClientEvent::UnassignPawn(_) => {
                        self.pawn_key = None;
                        println!("unassign pawn");
                    }
                    ClientEvent::CreateActor(actor_key) => {
                        if let Some(actor) = self.client.get_actor(&actor_key) {
                            match actor {
                                Actors::WorldActor(world_actor) => {
                                    println!("got world update at key {}", actor_key);
                                    self.map = Some(Map::new(*(world_actor.as_ref().borrow().seed.get())));
                                    println!("new seed: {}", self.map.as_ref().unwrap().seed);
                                },
                                _ => {}
                            }
                        }
                    }
                    ClientEvent::Tick => {
                        if let Some(pawn_key) = self.pawn_key {
                            if let Some(command) = self.queued_command.take() {
                                self.client.send_command(pawn_key, &command);
                            }
                        }
                    }
                    ClientEvent::Command(pawn_key, command_type) => {
                        if let Events::KeyCommand(key_command) = command_type {
                            if let Some(typed_actor) = self.client.get_pawn_mut(&pawn_key) {
                                println!("{}", pawn_key);
                                match typed_actor {
                                    Actors::PointActor(actor) => {
                                        shared_behaviour::process_command(&key_command, actor);
                                        self.player_x = *(actor.as_ref().borrow().x.get());
                                        self.player_y = *(actor.as_ref().borrow().y.get());
                                    },
                                    _ => {}
                                }
                            }
                        }
                    },
                    _ => {},
                },
                Err(err) => {
                    println!("Client Error: {}", err);
                }
            }
        }

        if self.client.has_connection() {
            let center_x = self.ss.0.div_floor(&2);
            let center_y = self.ss.1.div_floor(&2);

            let cam_off_x = self.player_x - center_x;
            let cam_off_y = self.player_y - center_y;

            self.draw_map(renderer, cam_off_x, cam_off_y);

            renderer.attrset(COLOR_PAIR(7));
            renderer.plot(center_x, center_y, '@' as chtype);

            self.draw_others(renderer, cam_off_x, cam_off_y);
        } else {
            renderer.mvaddstr(self.ss.0/2,self.ss.1/2,"NOT CONNECTED");
        }

        renderer.mvaddstr(0, 0, format!("frametime: {} ({} fps)", frame_time, 1.0/frame_time));
        renderer.mvaddstr(1, 0, format!("{},{}", self.ss.0, self.ss.1));
    }

    fn draw_map<T: Renderer>(&mut self, renderer: &mut T, cam_off_x: i32, cam_off_y: i32) {
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
                            renderer.plot(i * CHUNK_SIZE_I32 + x as i32 - cam_off_x, j * CHUNK_SIZE_I32 + y as i32 - cam_off_y, chr);
                        }
                    }
                }
            }
        }
    }

    fn draw_others<T: Renderer>(&mut self, renderer: &mut T, cam_off_x: i32, cam_off_y: i32) {
        for actor_key in self.client.actor_keys().unwrap() {
            if actor_key == self.pawn_key.unwrap_or(0) { continue; }
            if let Some(actor) = self.client.get_actor(&actor_key) {
                match actor {
                    Actors::PointActor(point_actor) => {
                        let x = *(point_actor.as_ref().borrow().x.get());
                        let y = *(point_actor.as_ref().borrow().y.get());
                        renderer.plot(x - cam_off_x, y - cam_off_y, 'O');
                    },
                    _ => {}
                }
            }
        }
    }
}