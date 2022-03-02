use bevy::prelude::*;

use crate::{components::*, pathfinding::PathFollow};

#[derive(Bundle, Clone)]
pub struct UnitBundle {
    #[bundle]
    sprite: SpriteBundle,
    velocity: Velocity,
    ai_unit: AiUnit,
    path_follow: PathFollow,
    health: Health,
    damage_absorber: DamageAbsorber,
    gold: Gold,
    // sprite: SpriteBundle {
    //     sprite: Sprite {
    //         color: Color::rgb(1.0, 0.0, 0.0),
    //         custom_size: Some(Vec2::new(32.0, 32.0)),
    //         ..Default::default()
    //     },
    //     transform: Transform::from_translation(translation.extend(1.0)),
    //     ..Default::default()
    // })
    // .insert(Velocity::new(1.0, 0.0, 0.0))
    // .insert(AiUnit)
    // .insert(PathFollow::new(0, 1.5))
    // .insert(Health::new(1.0))
    // .insert(DamageAbsorber::new(32.0, 32.0))
    // .insert(Gold(1));
}
impl Default for UnitBundle {
    fn default() -> Self {
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 0.5),
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            velocity: Velocity::new(0.0, 0.0, 0.0),
            ai_unit: AiUnit,
            path_follow: PathFollow::new(0, 1.5),
            health: Health::new(1.0),
            damage_absorber: DamageAbsorber::new(32.0, 32.0),
            gold: Gold(1),
        }
    }
}
impl UnitBundle {
    pub fn standard() -> Self {
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            velocity: Velocity::new(1.0, 0.0, 0.0),
            path_follow: PathFollow::new(0, 1.5),
            health: Health::new(1.0),
            damage_absorber: DamageAbsorber::new(32.0, 32.0),
            gold: Gold(1),
            ..Default::default()
        }
    }
    pub fn standard_tank() -> Self {
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 1.0),
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            velocity: Velocity::new(0.0, 0.0, 0.0),
            path_follow: PathFollow::new(0, 2.0),
            health: Health::new(2.0),
            damage_absorber: DamageAbsorber::new(32.0, 32.0),
            gold: Gold(1),
            ..Default::default()
        }
    }
    pub fn standard_op() -> Self {
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 1.0, 1.0),
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            path_follow: PathFollow::new(0, 8.0),
            health: Health::new(20.0),
            damage_absorber: DamageAbsorber::new(32.0, 32.0),
            gold: Gold(0),
            ..Default::default()
        }
    }
}