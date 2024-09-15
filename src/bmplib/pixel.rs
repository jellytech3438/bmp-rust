use std::num::Wrapping;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixels {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixels {
    pub fn new(r: u8, g: u8, b: u8, _a: u8) -> Self {
        Pixels { r, g, b, a: _a }
    }

    pub fn to_rgb(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
    pub fn to_bgr(&self) -> [u8; 3] {
        [self.b, self.g, self.r]
    }
    pub fn to_bgra(&self) -> [u8; 4] {
        [self.b, self.g, self.r, self.a]
    }
    pub fn to_rgba(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl From<[u8; 4]> for Pixels {
    fn from(value: [u8; 4]) -> Self {
        Pixels {
            r: value[0],
            g: value[1],
            b: value[2],
            a: value[3],
        }
    }
}

impl From<[u8; 3]> for Pixels {
    fn from(value: [u8; 3]) -> Self {
        Pixels {
            r: value[2],
            g: value[1],
            b: value[0],
            a: 1,
        }
    }
}
