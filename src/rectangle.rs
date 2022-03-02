use bevy::{math::Vec2, prelude::Transform};


/// Hitbox with an absolute world position
pub struct Hitbox {
    start: Vec2,
    end: Vec2,
}
impl Hitbox {
    pub fn new(x: f32, y: f32, x2: f32, y2: f32) -> Self {
        Self { start: Vec2::new(x, y), end: Vec2::new(x2, y2) }
    }
    pub fn from_wh(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self::new(x, y, x + w, y + h)
    }
    pub fn with_extents(wh: Vec2) -> Self {
        Self::from_end_points(-wh / 2.0, wh / 2.0)
    }
    pub fn from_end_points(start: Vec2, end: Vec2) -> Self {
        Self { start, end }
    }
    pub fn with_offset(&self, offset: Vec2) -> Self {
        Self::from_end_points(
            self.start + offset,
            self.end + offset,
        )
    }
    pub fn with_translation(&self, translation: &Transform) -> Self {
        let offset = translation.translation.truncate();
        self.with_offset(offset)
    }
    pub fn sx(&self) -> f32 { self.start.x }
    pub fn sy(&self) -> f32 { self.start.y }
    pub fn ex(&self) -> f32 { self.end.x }
    pub fn ey(&self) -> f32 { self.end.y }
    pub fn width(&self) -> f32 { self.ex() - self.sx() }
    pub fn height(&self) -> f32 { self.ey() - self.sy() }
    pub fn touches(&self, other: &Hitbox) -> bool {
        self.ex() >= other.sx() &&
        self.ey() >= other.sy() &&
        self.sx() <= other.ex() &&
        self.sy() <= other.ey()
    }
    pub fn point_touches(&self, other: &Vec2) -> bool {
        self.ex() >= other.x &&
        self.ey() >= other.y &&
        self.sx() <= other.x &&
        self.sy() <= other.y
    }
}