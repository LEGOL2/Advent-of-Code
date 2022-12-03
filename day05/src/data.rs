use crate::abs;

pub struct Data {
    pub x1: isize,
    pub y1: isize,
    pub x2: isize,
    pub y2: isize,
}

impl Data {
    pub fn new(x1: isize, y1: isize, x2: isize, y2: isize) -> Self {
        Data { x1, y1, x2, y2 }
    }

    pub fn min_x(&self) -> isize {
        if self.x1 <= self.x2 {
            self.x1
        } else {
            self.x2
        }
    }

    pub fn min_y(&self) -> isize {
        if self.y1 <= self.y2 {
            self.y1
        } else {
            self.y2
        }
    }

    pub fn slope(&self) -> isize {
        if self.x1 == self.x2 {
            0
        } else {
            (self.y2 - self.y1) / (self.x2 - self.x1)
        }
    }

    pub fn distance(&self) -> isize {
        isize::max(abs(self.x1, self.x2), abs(self.y1, self.y2))
    }
}
