use bevy::prelude::Vec2;

pub struct Coordinate<const SIZE: usize> {
    pub x: i32,
    pub y: i32,
}

impl<const SIZE: usize> Coordinate<SIZE> {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn into_vec2(&self) -> Vec2 {
        Vec2::new(self.x as f32 * SIZE as f32, self.y as f32 * SIZE as f32)
    }
}
