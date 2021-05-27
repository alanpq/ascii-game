use crate::renderer::Renderer;
use glium::Frame;
use std::rc::Rc;

pub trait Drawer {
    fn draw_line(&self, frame: Rc<Frame>, x0: i32, y0: i32, x1: i32, y1: i32);
}


const line_chars: [char;12] = ['_','.','-','"','"','\'','-','_','/','|','\\','|'];
impl<T> Drawer for T where T: Renderer {
    fn draw_line(&self, frame: Rc<Frame>, x0: i32, y0: i32, x1: i32, y1: i32) {
        // let mut x0 = x0;
        // let mut y0 = y0;
        // let x_diff = x1-x0;
        // let y_diff = y1-y0;
        //
        // let slope = (y_diff as f32 / x_diff as f32).abs();
        //
        // let dx = (x_diff).abs();
        // let sx = if x0<x1 { 1 } else { -1 };
        // let dy = -(y_diff).abs();
        // let sy = if y0<y1 { 1 } else { -1 };
        // let mut err = dx + dy;
        //
        // self.mvaddstr(frame.clone(), y0 - sy, x0, format!("slope: {}", slope));
        //
        // loop {
        //     let e2 = 2 * err;
        //     if slope == 0.0 {
        //         self.plot(frame.clone(),x0, y0, if dx == 0 { '|' } else { '-' });
        //     } else if slope < 1.0 {
        //         let idx = ((e2) / dx) + 1;
        //         if idx >= 0 && idx < 4 as i32 {
        //             println!("{}", idx);
        //             if sy == 1 {
        //                 self.plot(frame.clone(),x0, y0, line_chars[idx as usize]);
        //                 //plot(window, x0, y0, idx.to_string().chars().next().unwrap());
        //             } else {
        //                 self.plot(frame.clone(),x0, y0, line_chars[idx as usize+4]);
        //                 //plot(window, x0, y0, (3-idx).to_string().chars().next().unwrap());
        //             }
        //         } else {
        //             println!("{}   !!!!!!", idx);
        //             self.plot(frame.clone(),x0, y0, idx.to_string().chars().next().unwrap());
        //         }
        //     } else {
        //         let idx = (err / dy)+8;
        //         if idx >= 0 && idx < line_chars.len() as i32 {
        //             println!("{}", idx);
        //             if sx == sy {
        //                 self.plot(frame.clone(),x0, y0, line_chars[idx as usize + 2]);
        //             } else {
        //                 self.plot(frame.clone(),x0, y0, line_chars[idx as usize]);
        //             }
        //         } else {
        //             println!("{}   !!!!!!", idx);
        //             self.plot(frame.clone(),x0, y0, idx.to_string().chars().next().unwrap());
        //         }
        //     }
        //
        //     if x0 == x1 && y0 == y1 { break; }
        //     if e2 >= dy {
        //         err += dy;
        //         x0 += sx;
        //     }
        //     if e2 <= dx {
        //         err += dx;
        //         y0 += sy;
        //     }
        // }
    }
}