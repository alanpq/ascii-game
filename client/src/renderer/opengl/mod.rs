pub mod context;

use crate::renderer::{Renderer, Event};
use pancurses::chtype;
use glium::glutin::{event_loop::EventLoop, window::WindowBuilder, ContextBuilder};

use pancurses::{ToChtype, Input};
use glium::{Display, Surface, glutin, VertexBuffer, IndexBuffer, Program, Frame};
use std::rc::Rc;
use std::io::Cursor;
use glium::index::PrimitiveType;
use glium::glutin::platform::run_return::EventLoopExtRunReturn;
use num_integer::Integer;
use std::cell::{RefCell, Ref};
use std::borrow::BorrowMut;

pub struct GlRenderer {
    pub display: Option<Rc<Display>>,

    vertex_buffer: Option<VertexBuffer<Vertex>>,
    index_buffer: Option<IndexBuffer<u16>>,
    program: Option<Program>,
    char_tex: Option<glium::texture::Texture2d>,

    frame: RefCell<Option<Frame>>,

    view_matrix: [[f32; 4]; 4],
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

impl GlRenderer {
    fn view_matrix(&self, display: &Display) -> [[f32; 4]; 4] {
        let dims = display.get_framebuffer_dimensions();
        let aspect = dims.0 as f32 / dims.1 as f32;
        let right = dims.0 as f32 / CHAR_SIZE as f32;
        let left = 0.0;
        let bottom = -((dims.1 as f32)  / CHAR_SIZE as f32);
        let top = 0.0;
        let near = 0.0;
        let far = 1.0;
        [
            [2./(right-left), 0.0, 0.0, -(right+left)/(right-left)],
            [0.0, 2./(top-bottom), 0.0, -(top+bottom)/(top-bottom)],
            [0.0, 0.0, -2./(far-near), -(far+near)/(far-near)],
            [0.0, 0.0, 0.0, 1.0f32]
        ]
    }

    fn translation_matrix(x: f32, y: f32) -> [[f32; 4]; 4] {
        [
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, -y],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]
    }

    fn draw_char(&self, display: Rc<Display>, frame: Rc<RefCell<Frame>>, x: i32, y: i32, ch: char) {
        if (ch as u32) < 33 {
            //warn!("char '{}' ({}) not in charmap", ch, ch as u32);
            return;
        }
        // let dims = self.dimensions();
        let uniforms = uniform! {
            matrix: self.view_matrix,
            translate: Self::translation_matrix(x as f32 + 0.5, y as f32 + 0.5),
            tex: self.char_tex.as_ref().unwrap(),
            u_idx: ((ch as u32) - 33) as f32,
            u_rows: 16.0f32,
            u_columns: 16.0f32,
        };
        // drawing a frame
        // let mut target = display.draw();
        (*frame).borrow_mut().draw(self.vertex_buffer.as_ref().unwrap(), self.index_buffer.as_ref().unwrap(), self.program.as_ref().unwrap(), &uniforms, &Default::default()).unwrap();

    }

    pub fn draw(&mut self) {
        // let display = self.display.as_ref().unwrap();
        // let dims = self.dimensions();
        // let frame = display.draw();
        // for x in 0..dims.0 {
        //     for y in 0..dims.1 {
        //         self.draw_char(&frame, x, y, x.to_string().chars().next().unwrap());
        //     }
        // }
        // frame.finish().unwrap();
    }
}

const CHAR_SIZE: i32 = 32; // TODO: make this not hardcoded

impl Renderer for GlRenderer {
    fn new() -> Self {
        GlRenderer {
            display: None,
            vertex_buffer: None,
            index_buffer: None,
            program: None,
            char_tex: None,
            frame: RefCell::new(None),
            view_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
        }
    }

    fn resize(&mut self, display: &Display) {
        self.view_matrix = self.view_matrix(display);
    }

    fn dimensions(&self, display: &Display) -> (i32, i32) {
        let dims = display.get_framebuffer_dimensions();
        ((dims.0 as i32).div_floor(&CHAR_SIZE), (dims.1 as i32).div_floor(&CHAR_SIZE))
    }

    fn init(&mut self, display: &Display) {
        let image = image::load(Cursor::new(&include_bytes!("fontmaps/roboto-mono.bmp")), image::ImageFormat::Bmp).unwrap().to_rgb8();
        let image_dims = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgb_reversed(&image.into_raw(), image_dims);

        self.char_tex = Some(glium::texture::Texture2d::new(display, image).unwrap());

        // building the vertex buffer, which contains all the vertices that we will draw
        self.vertex_buffer = Some({

            implement_vertex!(Vertex, position, tex_coords);

            VertexBuffer::new(display,
                                     &[
                                         Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
                                         Vertex { position: [ 0.5, -0.5], tex_coords: [1.0, 0.0] },
                                         Vertex { position: [-0.5,  0.5], tex_coords: [0.0, 1.0] },
                                         Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0] },
                                     ]
            ).unwrap()
        });

        // building the index buffer
        self.index_buffer = Some(glium::IndexBuffer::new(display, PrimitiveType::TrianglesList,
                                                   &[0u16, 1, 2, 2, 1, 3]).unwrap());

        self.program = Some(program!(display,
            140 => {
                vertex: "
                    #version 140
                    uniform mat4 matrix;
                    uniform mat4 translate;
                    uniform float u_idx;
                    uniform float u_rows;
                    uniform float u_columns;

                    in vec2 position;
                    in vec2 tex_coords;
                    out vec2 v_tex_coords;
                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0) * translate * matrix;

                        float idx = mod(u_idx, u_rows * u_columns);
                        float y = floor(idx / u_columns);
                        float x = idx - y*u_columns;
                        vec2 charSize = vec2(
                            1.0 / u_rows,
                            1.0 / u_columns
                        );
                        vec2 offset = vec2(charSize.x * x, charSize.y * -(y+1));
                        v_tex_coords = (tex_coords * charSize) + offset;
                    }
                ",

                fragment: "
                    #version 140
                    in vec2 v_tex_coords;
                    out vec4 f_color;

                    uniform sampler2D tex;

                    void main() {
                        f_color = texture(tex, v_tex_coords);
                    }
                "
            }
        ).unwrap()); // TODO: better shader handling (literally anything other than this please)

        self.view_matrix = self.view_matrix(display);
        // Draw the triangle to the screen.
        //self.draw(self.display.clone().unwrap(), &vertex_buffer, &index_buffer, &program, &char_texture);

    }

    fn kill(&mut self) {

    }

    fn plot<T: ToChtype>(&self, display: Rc<Display>, frame: Rc<RefCell<Frame>>, x: i32, y: i32, chr: T) {
        self.draw_char(display.clone(), frame.clone(), x, y, char::from_u32(chr.to_chtype() as u32).unwrap());
    }

    fn erase(&self) {

    }

    fn getch(&self) -> Option<Event> {
        None
    }

    fn printw<T: AsRef<str>>(&self, string: T) -> i32 {
        0
    }

    fn mvaddch<T: ToChtype>(&self, y: i32, x: i32, ch: T) -> i32 {
        0
    }

    fn mvaddstr<T: AsRef<str>>(&self, display: Rc<Display>, frame: Rc<RefCell<Frame>>, y: i32, x: i32, string: T) -> i32 {
        for (i, c) in string.as_ref().chars().enumerate() {
            self.plot(display.clone(),frame.clone(), x+ i as i32, y, c);
        }
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