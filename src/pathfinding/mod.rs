use bevy::prelude::*;

use crate::components::{Velocity, Lives};

pub mod navdata;

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(follow_path)
            .add_system(end_path);
    }
}


/// A path of positions that will be traversed
pub struct NavPath(Vec<Vec2>);
impl NavPath {
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

pub fn end_path(
    mut commands: Commands,
    query: Query<(&PathFollow, Entity)>,
    mut lives: ResMut<Lives>,
    path: Res<NavPath>,
) {
    for (navigation, entity) in query.iter() {
        if let None = path.get(navigation.index) {
            lives.0 = lives.0.saturating_sub(1);
            commands.entity(entity).despawn_recursive();
        }
    }
}