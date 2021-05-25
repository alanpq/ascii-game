use crate::renderer::Renderer;
use pancurses::chtype;
use glium::glutin::{event_loop::EventLoop, window::WindowBuilder, ContextBuilder};

use pancurses::{ToChtype, Input};
use glium::{Display, Surface, glutin, VertexBuffer, IndexBuffer, Program};
use std::rc::Rc;
use glium::index::PrimitiveType;
use glium::glutin::platform::run_return::EventLoopExtRunReturn;

pub struct GlRenderer {
    pub display: Option<Rc<Display>>,
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

impl GlRenderer {
    pub fn draw(&mut self, display: Rc<Display>, vertex_buffer: &VertexBuffer<Vertex>, index_buffer: &IndexBuffer<u16>, program: &Program) {
        // let display = display.unwrap();
        // building the uniforms
        let uniforms = uniform! {
                    matrix: [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0f32]
                    ]
                };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(vertex_buffer, index_buffer, program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
    }
}

impl Renderer for GlRenderer {
    fn new() -> Self {
        GlRenderer {
            display: None
        }
    }

    fn dimensions(&self) -> (i32, i32) {
        (10, 10) // TODO: implement this
    }

    fn init(&mut self) {
        let mut event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new();

        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        // building the vertex buffer, which contains all the vertices that we will draw
        let vertex_buffer = {

            implement_vertex!(Vertex, position, color);

            VertexBuffer::new(&display,
                                     &[
                                         Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
                                         Vertex { position: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
                                         Vertex { position: [ 0.5, -0.5], color: [1.0, 0.0, 0.0] },
                                     ]
            ).unwrap()
        };

        // building the index buffer
        let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList,
                                                   &[0u16, 1, 2]).unwrap();

        let program = program!(&display,
            140 => {
                vertex: "
                    #version 140
                    uniform mat4 matrix;
                    in vec2 position;
                    in vec3 color;
                    out vec3 vColor;
                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0) * matrix;
                        vColor = color;
                    }
                ",

                fragment: "
                    #version 140
                    in vec3 vColor;
                    out vec4 f_color;
                    void main() {
                        f_color = vec4(vColor, 1.0);
                    }
                "
            },

            110 => {
                vertex: "
                    #version 110
                    uniform mat4 matrix;
                    attribute vec2 position;
                    attribute vec3 color;
                    varying vec3 vColor;
                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0) * matrix;
                        vColor = color;
                    }
                ",

                fragment: "
                    #version 110
                    varying vec3 vColor;
                    void main() {
                        gl_FragColor = vec4(vColor, 1.0);
                    }
                ",
            },

            100 => {
                vertex: "
                    #version 100
                    uniform lowp mat4 matrix;
                    attribute lowp vec2 position;
                    attribute lowp vec3 color;
                    varying lowp vec3 vColor;
                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0) * matrix;
                        vColor = color;
                    }
                ",

                fragment: "
                    #version 100
                    varying lowp vec3 vColor;
                    void main() {
                        gl_FragColor = vec4(vColor, 1.0);
                    }
                ",
            },
        ).unwrap();

        self.display = Some(Rc::new(display));

        // Draw the triangle to the screen.
        self.draw(self.display.clone().unwrap(), &vertex_buffer, &index_buffer, &program);

        let display = self.display.clone();
        // the main loop
        event_loop.run_return(move |event, _, control_flow| {
            *control_flow = match event {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    // Break from the main loop when the window is closed.
                    glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
                    // Redraw the triangle when the window is resized.
                    glutin::event::WindowEvent::Resized(..) => {
                        self.draw(display.clone().unwrap(), &vertex_buffer, &index_buffer, &program);
                        glutin::event_loop::ControlFlow::Poll
                    },
                    _ => glutin::event_loop::ControlFlow::Poll,
                },
                _ => glutin::event_loop::ControlFlow::Poll,
            };
        });
    }

    fn kill(&mut self) {

    }

    fn plot<T: ToChtype>(&self, x: i32, y: i32, chr: T) {

    }

    fn erase(&self) {

    }

    fn getch(&self) -> Option<Input> {
        None
    }

    fn printw<T: AsRef<str>>(&self, string: T) -> i32 {
        0
    }

    fn mvaddch<T: ToChtype>(&self, y: i32, x: i32, ch: T) -> i32 {
        0
    }

    fn mvaddstr<T: AsRef<str>>(&self, y: i32, x: i32, string: T) -> i32 {
        0
    }

    fn mvinch(&self, y: i32, x: i32) -> u64 {
        0
    }

    fn mvprintw<T: AsRef<str>>(&self, y: i32, x: i32, string: T) -> i32 {
        0
    }

    fn attrset<T: Into<chtype>>(&self, attributes: T) {

    }

    fn refresh(&self) -> i32 {
        0
    }
}