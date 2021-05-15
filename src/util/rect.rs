pub struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Rect {
    pub fn area(&self) -> i32 {
        w * h
    }

    pub fn max_x(&self) -> i32 {
        x + w
    }

    pub fn max_y(&self) -> i32 {
        y + h
    }
}