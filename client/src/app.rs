use pancurses::{ALL_MOUSE_EVENTS, Window, endwin, newwin, getmouse, initscr, mousemask, Input, resize_term, REPORT_MOUSE_POSITION, ToChtype, Attribute, curs_set, cbreak, noecho, chtype, BUTTON1_RELEASED, COLOR_BLACK, COLOR_PAIR, COLOR_GREEN};
use crate::renderer::curses::CursesRenderer;
use crate::renderer::{Renderer, drawer::Drawer};

use crate::game::map::Map;
use crate::game::map::{CHUNK_SIZE, CHUNK_SIZE_I32, Tile};
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

const SERVER_PORT: u16 = 14191;

pub struct App {
    client: NaiaClient<Events, Actors>,
    pawn_key: Option<u16>,
    queued_command: Option<KeyCommand>,

    ss: (i32, i32),
    fps_counter: FpsCounter,
}

impl App {
    pub fn new<T: Renderer>(renderer: &T) -> Self {
        info!("Naia Macroquad Client Example Started");

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

            ss: renderer.dimensions(),
            fps_counter: FpsCounter::new(),
        }
    }

    pub fn update<T: Renderer>(&mut self, renderer: &mut T) {
        let frame_time = self.fps_counter.tick();
        match renderer.getch() {
            Some(Input::KeyResize) => {
                resize_term(0, 0); // TODO: abstract this away
                self.ss = renderer.dimensions();
            },
            _ => {}
        }

        while let Some(result) = self.client.receive() {
            match result {
                Ok(event) => match event {
                    ClientEvent::Connection => {
                        info!("Connected to {}", self.client.server_address());
                    },
                    ClientEvent::Disconnection => {
                        info!("Disconnected from {}", self.client.server_address());
                    }
                    ClientEvent::AssignPawn(local_key) => {
                        self.pawn_key = Some(local_key);
                        info!("assign pawn");
                    }
                    ClientEvent::UnassignPawn(_) => {
                        self.pawn_key = None;
                        info!("unassign pawn");
                    }
                    ClientEvent::Tick => {
                        if let Some(pawn_key) = self.pawn_key {
                            if let Some(command) = self.queued_command.take() {
                                self.client.send_command(pawn_key, &command);
                            }
                        }
                    }
                    ClientEvent::Command(pawn_key, command_type) => match command_type {
                        Events::KeyCommand(key_command) => {
                            if let Some(typed_actor) = self.client.get_pawn_mut(&pawn_key) {
                                match typed_actor {
                                    Actors::PointActor(actor) => {
                                        shared_behaviour::process_command(&key_command, actor);
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                    _ => {},
                },
                Err(err) => {
                    info!("Client Error: {}", err);
                }
            }
        }

        if self.client.has_connection() {
            for actor_key in self.client.actor_keys().unwrap() {
                if let Some(actor) = self.client.get_actor(&actor_key) {
                    match actor {
                        Actors::PointActor(point_actor) => {
                            renderer.plot(*(point_actor.as_ref().borrow().x.get()), *(point_actor.as_ref().borrow().y.get()), 'O');
                            info!("{}", *(point_actor.as_ref().borrow().x.get()));
                        }
                    }
                }
            }

            for pawn_key in self.client.pawn_keys().unwrap() {
                if let Some(actor) = self.client.get_pawn(&pawn_key) {
                    match actor {
                        Actors::PointActor(point_actor) => {
                            renderer.plot(*(point_actor.as_ref().borrow().x.get()), *(point_actor.as_ref().borrow().y.get()), '@');
                        }
                    }
                }
            }
        } else {
            renderer.mvaddstr(self.ss.0/2,self.ss.1/2,"NOT CONNECTED");
        }

        renderer.mvaddstr(0, 0, format!("frametime: {} ({} fps)", frame_time, 1.0/frame_time));
        renderer.mvaddstr(1, 0, format!("{},{}", self.ss.0, self.ss.1));
    }
}