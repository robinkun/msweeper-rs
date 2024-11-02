#[derive(Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl Point<isize> {
    pub fn itou(&self) -> Point<usize> {
        return Point::<usize> {
            x: self.x as usize,
            y: self.y as usize,
        };
    }

    pub fn pos8_iter() -> std::ops::Range<i32> {
        return 0..8;
    }

    pub fn get_pos_8(&self, pos8: i32) -> Point<isize> {
        let mut p = self.clone();
        match pos8 {
            0 => {
                p.x = self.x - 1;
                p.y = self.y - 1;
            }
            1 => {
                p.x = self.x;
                p.y = self.y - 1;
            }
            2 => {
                p.x = self.x + 1;
                p.y = self.y - 1;
            }
            3 => {
                p.x = self.x - 1;
                p.y = self.y;
            }
            4 => {
                p.x = self.x + 1;
                p.y = self.y;
            }
            5 => {
                p.x = self.x - 1;
                p.y = self.y + 1;
            }
            6 => {
                p.x = self.x;
                p.y = self.y + 1;
            }
            7 => {
                p.x = self.x + 1;
                p.y = self.y + 1;
            }
            _ => {}
        };
        return p;
    }
}

impl Point<usize> {
    pub fn utoi(&self) -> Point<isize> {
        return Point::<isize> {
            x: self.x as isize,
            y: self.y as isize,
        };
    }
}

macro_rules! impl_Point {
    ($type:ty) => {
        impl Point<$type> {
            pub fn is_equal(&self, p: &Point<$type>) -> bool {
                if p.x == self.x && p.y == self.y {
                    return true;
                }
                return false;
            }
        }
    };
}

//impl_Point!(isize);
impl_Point!(usize);
