use std::time::Duration;

use bevy::prelude::*;

use crate::{components::Gold, pathfinding::NavPath};

use self::unitdata::UnitBundle;

pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Stages>()
            .init_resource::<CurrentStage>()
            .add_system(play_stages);
    }
}

pub mod unitdata;

#[derive(Debug)]
pub struct CurrentStage {
    pub index: usize,
    pub in_stage: bool,
    pub spawn_data: StageSpawnData,
}
impl CurrentStage {
    pub fn start_stage(&mut self) {
        if !self.in_stage {
            self.in_stage = true;
            self.spawn_data.unit_group_index = 0;
            self.spawn_data.spawn_timer.reset();
            self.spawn_data.counter = 0;
        }
    }
    pub fn finish_stage(&mut self) {
        if self.in_stage {
            self.in_stage = false;
            self.index += 1;
        }
    }
}
impl Default for CurrentStage {
    fn default() -> Self {
        Self {
            index: 0,
            in_stage: false,
            spawn_data: StageSpawnData {
                unit_group_index: 0,
                spawn_timer: Timer::from_seconds(1.0, true),
                counter: 0,
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct StageSpawnData {
    unit_group_index: usize,
    spawn_timer: Timer,
    counter: usize,
}

fn play_stages(
    mut commands: Commands,
    time: Res<Time>,
    mut stages: ResMut<Stages>,
    path: Res<NavPath>,
    mut current_stage: ResMut<CurrentStage>,
    mut gold: ResMut<Gold>,
) {
    if current_stage.in_stage {
        if let Some(stage) = stages.0.get(current_stage.index) {
            if let Some(units) = stage.units.get(current_stage.spawn_data.unit_group_index) {
                current_stage.spawn_data.spawn_timer.set_duration(Duration::from_secs_f32(units.secs_between_spawns));
                current_stage.spawn_data.spawn_timer.tick(time.delta());
                if current_stage.spawn_data.spawn_timer.just_finished() {
                    if current_stage.spawn_data.counter != units.count {
                        println!("spawning unit {}", current_stage.spawn_data.counter);
                        let translation = path.get(0).unwrap();
                        commands.spawn_bundle(units.unit_data.clone())
                            .insert(Transform::from_translation(translation.extend(1.0)));
                        current_stage.spawn_data.counter += 1;
                    } else {
                        current_stage.spawn_data.counter = 0;
                        current_stage.spawn_data.unit_group_index += 1;
                        println!("going to unit group {}", current_stage.spawn_data.unit_group_index);
                    }
                }
            } else {
                current_stage.finish_stage();
                gold.0 += stage.reward.0;
                println!("going to stage {}", current_stage.index);
            }
        } else {
            stages.0.push(StageData::scale_with_stage(current_stage.index as u32));
        }
    }
}

/// Resource to keep track of coming stages
pub struct Stages(Vec<StageData>);
impl Default for Stages {
    fn default() -> Self {
        Self(
            vec![
                StageData::new(80,
                    vec![
                        UnitGroup::new(
                            UnitBundle::standard(),
                            20,
                            0.5,
                        ),
                    ]
                ),
                StageData::new(100,
                    vec![
                        UnitGroup::new(
                            UnitBundle::standard(),
                            7,
                            0.4,
                        ),
                        UnitGroup::new(
                            UnitBundle::standard_tank(),
                            3,
                            0.5,
                        ),
                        UnitGroup::new(
                            UnitBundle::standard(),
                            7,
                            0.4,
                        ),
                        UnitGroup::new(
                            UnitBundle::standard_tank(),
                            3,
                            0.5,
                        ),
                    ]
                ),
                StageData::new(100,
                    vec![
                        UnitGroup::new(
                            UnitBundle::standard_op(),
                            99,
                            0.01,
                        ),
                    ]
                ),
                StageData::new(100,
                    vec![
                        UnitGroup::new(
                            UnitBundle::standard_tank(),
                            10,
                            0.4,
                        ),
                        UnitGroup::new(
                            UnitBundle::standard(),
                            5,
                            0.4,
                        ),
                        UnitGroup::new(
                            UnitBundle::standard_tank(),
                            10,
                            0.4,
                        ),
                        UnitGroup::new(
                            UnitBundle::standard_fast(),
                            5,
                            0.5,
                        ),
                    ]
                ),
            ]
        )
    }
}

pub struct StageData {
    reward: Gold,
    units: Vec<UnitGroup>
}
impl StageData {
    pub fn new(reward: u32, units: Vec<UnitGroup>) -> Self {
        Self { reward: Gold(reward), units }
    }
    pub fn scale_with_stage(stage: u32) -> Self {
        let reward = (100.0 * 1.01_f32.powi(stage as i32)) as u32;
        let units = vec![
            UnitGroup::new(
                UnitBundle::standard(),
                10 + stage as usize,
                1.0 / stage as f32,
            ),
            UnitGroup::new(
                UnitBundle::standard_tank(),
                5 + (stage / 2) as usize,
                2.0 / stage as f32,
            ),
            UnitGroup::new(
                UnitBundle::standard_fast(),
                (stage / 3) as usize,
                3.0 / stage as f32,
            ),
            UnitGroup::new(
                UnitBundle::standard_large(),
                (stage / 20) as usize,
                10.0 / stage as f32,
            ),
            UnitGroup::new(
                UnitBundle::standard_op(),
                (stage / 10) as usize,
                10.0 / stage as f32,
            ),
        ];
        StageData::new(reward, units)
    }
}

pub struct UnitGroup {
    unit_data: UnitBundle,
    count: usize,
    secs_between_spawns: f32,
}
impl UnitGroup {
    pub fn new(unit_data: UnitBundle, count: usize, secs_between_spawns: f32) -> Self {
        Self { unit_data, count, secs_between_spawns }
    }
}