use std::rc::Rc;
use glium::{Display, Frame, Surface};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::glutin::event::{WindowEvent, Event, ElementState};
use glium::glutin::event_loop::{EventLoop, ControlFlow};
use crate::renderer;
use glium::glutin::platform::run_return::EventLoopExtRunReturn;
use crate::renderer::opengl::GlRenderer;
use std::cell::RefCell;
use crate::renderer::Renderer;

pub struct GlContext {
    pub display: Option<Rc<Display>>
}

impl GlContext {
    pub fn new() -> Self {
        GlContext {
            display: None,
        }
    }

    pub fn run<F>(&mut self, renderer: Rc<RefCell<GlRenderer>>, mut tick_fn: F)
        where F: FnMut(Rc<Display>, Rc<RefCell<Frame>>, Option<renderer::Event>) {
        let mut event_loop = EventLoop::new();
        let wb = WindowBuilder::new();
        let cb = ContextBuilder::new();

        let display = Rc::new(glium::Display::new(wb, cb, &event_loop).unwrap());

        renderer.borrow_mut().init(&display);

        // the main loop
        event_loop.run_return(move |event, _, control_flow| {
            let mut event_out: Option<renderer::Event> = None;
            *control_flow = match event {
               Event::WindowEvent { event, .. } => match event {
                    // Break from the main loop when the window is closed.
                    WindowEvent::CloseRequested => ControlFlow::Exit,
                    // Redraw the triangle when the window is resized.
                    WindowEvent::Resized(..) => {
                        //self.draw(display.clone().unwrap(), &vertex_buffer, &index_buffer, &program, &char_texture);
                        // let dims = self.dimensions();
                        // info!("{}, {}", dims.0, dims.1);
                        ControlFlow::Poll
                    },
                    WindowEvent::KeyboardInput {input, ..} => {
                        if input.state == ElementState::Pressed {
                            info!("scancode: {}", input.scancode);
                            event_out = match input.scancode {
                                28 => Some(renderer::Event::Character(10 as char)), // enter
                                14 => Some(renderer::Event::Character(8 as char)), // backspace
                                1 => Some(renderer::Event::Character(27 as char)), // escape
                                _ => None
                            };
                        }
                        ControlFlow::Poll
                    }
                    WindowEvent::ReceivedCharacter(ch) => {
                        event_out = Some(renderer::Event::Character(ch));
                        ControlFlow::Poll
                    },
                    _ => ControlFlow::Poll,
                },
                _ => ControlFlow::Poll,
            };
            let mut frame = display.draw();
            frame.clear_color(0.0,0.0,0.0,1.0);
            let mut frame = Rc::new(RefCell::new(frame));
            tick_fn(display.clone(), frame.clone(), event_out);

            if let Ok(frame) = Rc::try_unwrap(frame) {
                frame.into_inner().finish().unwrap();
            } else {
                panic!("could not finish frame!");
            }

        });
    }
}