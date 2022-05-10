use std::slice::Iter;

pub struct Vec2D {
    vector: Vec<usize>,
    pub width: usize,
}

impl Vec2D {
    pub fn from_vec(vector: Vec<usize>, width: usize) -> Self {
        if width * width != vector.len() {
            panic!();
        }
        Vec2D { vector, width }
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    pub fn get(&self, x: usize, y: usize) -> usize {
        let idx = self.idx(x, y);
        self.vector[idx]
    }

    pub fn set(&mut self, x: usize, y: usize, value: usize) {
        let idx = self.idx(x, y);
        self.vector[idx] = value;
    }

    pub fn add(&mut self, x: usize, y: usize, term: usize) {
        let idx = self.idx(x, y);
        self.vector[idx] += term;
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for y in 0..self.width {
            for x in 0..self.width {
                print!("{} ", self.get(x, y));
            }
            println!();
        }
        println!();
    }

    pub fn iter(&self) -> Iter<'_, usize> {
        self.vector.iter()
    }
}

impl Clone for Vec2D {
    fn clone(&self) -> Self {
        Self {
            vector: self.vector.clone(),
            width: self.width.clone(),
        }
    }
}
