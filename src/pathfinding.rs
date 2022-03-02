use bevy::prelude::*;

use crate::components::Velocity;

/// A path of positions that will be traversed
pub struct NavPath(Vec<Vec2>);
impl NavPath {
    pub fn map1() -> NavPath {
        let vec = vec![
            Vec2::new(-500.0, 500.0),
            Vec2::new(0.0, 500.0),
            Vec2::new(-10.0, 100.0),
            Vec2::new(-500.0, 100.0),
        ];
        NavPath(vec)
    }
    pub fn get(&self, index: usize) -> Option<Vec2> {
        self.0.get(index).cloned()
    }
}

#[derive(Clone, Debug, Component, Reflect, Default)]
/// Component to keep track of which part of the path you need to visit
pub struct PathFollow {
    index: usize,
    speed: f32,
}
impl PathFollow {
    pub fn new(index: usize, speed: f32) -> PathFollow {
        PathFollow {
            index,
            speed,
        }
    }
    pub fn advance(&mut self) {
        self.index += 1;
    }
}

pub fn follow_path(
    mut query: Query<(&mut Velocity, &Transform, &mut PathFollow)>,
    path: Res<NavPath>,
) {
    for (mut velocity, transform, mut navigation) in query.iter_mut() {
        if let Some(goal) = path.get(navigation.index) {
            let position = transform.translation.truncate();
            if goal.distance_squared(position) > 100.0 {
                let direction = (goal - position).normalize();
                velocity.velocity = (direction * navigation.speed).extend(0.0);
            } else {
                navigation.advance();
            }
        }
    }
}