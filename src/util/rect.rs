pub struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Rect {
    pub fn area(&self) -> i32 {
        self.w * self.h
    }

    pub fn max_x(&self) -> i32 {
        self.x + self.w
    }

    pub fn max_y(&self) -> i32 {
        self.y + self.h
    }
}