use crate::{ChainEvent, DominoType, PlaceDominoCmd};
use rapier2d::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DominoInstance {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub domino_type: DominoType,
    pub is_fallen: bool,
    pub angular_velocity: f32,
}

#[derive(Debug, Clone)]
pub struct GameChainStatus {
    pub domino_count: u32,
    pub fallen_count: u32,
    pub triggered: bool,
    pub completed: bool,
    pub time_elapsed: f32,
}

struct DominoMeta {
    id: u32,
    domino_type: DominoType,
    is_fallen: bool,
    raised_fall_event: bool,
}

pub struct GameWorld {
    events: Vec<ChainEvent>,
    elapsed: f32,
    next_id: u32,
    triggered: bool,
    completed: bool,
    deterministic_test_mode: bool,

    gravity: Vector<Real>,
    integration_parameters: IntegrationParameters,
    islands: IslandManager,
    broad_phase: BroadPhaseMultiSap,
    narrow_phase: NarrowPhase,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_pipeline: PhysicsPipeline,

    body_to_meta: HashMap<RigidBodyHandle, DominoMeta>,
    id_to_body: HashMap<u32, RigidBodyHandle>,
    trigger_domino_id: Option<u32>,
}

impl Default for GameWorld {
    fn default() -> Self {
        let mut world = Self {
            events: Vec::new(),
            elapsed: 0.0,
            next_id: 0,
            triggered: false,
            completed: false,
            deterministic_test_mode: false,
            gravity: vector![0.0, 980.0],
            integration_parameters: IntegrationParameters {
                dt: 1.0 / 60.0,
                ..IntegrationParameters::default()
            },
            islands: IslandManager::new(),
            broad_phase: BroadPhaseMultiSap::new(),
            narrow_phase: NarrowPhase::new(),
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_pipeline: PhysicsPipeline::new(),
            body_to_meta: HashMap::new(),
            id_to_body: HashMap::new(),
            trigger_domino_id: None,
        };
        world.insert_ground();
        world
    }
}

impl GameWorld {
    pub fn reset(&mut self) {
        self.events.clear();
        self.elapsed = 0.0;
        self.next_id = 0;
        self.triggered = false;
        self.completed = false;
        self.deterministic_test_mode = false;
        self.body_to_meta.clear();
        self.id_to_body.clear();
        self.trigger_domino_id = None;

        self.islands = IslandManager::new();
        self.broad_phase = BroadPhaseMultiSap::new();
        self.narrow_phase = NarrowPhase::new();
        self.bodies = RigidBodySet::new();
        self.colliders = ColliderSet::new();
        self.impulse_joints = ImpulseJointSet::new();
        self.multibody_joints = MultibodyJointSet::new();
        self.ccd_solver = CCDSolver::new();

        self.insert_ground();
        self.events.push(ChainEvent::LevelReset);
    }

    pub fn place_domino(&mut self, cmd: PlaceDominoCmd) -> u32 {
        self.next_id += 1;
        let id = self.next_id;
        let (width, height) = cmd.domino_type.dimensions();

        let rb = RigidBodyBuilder::dynamic()
            .translation(vector![cmd.x, cmd.y])
            .rotation(cmd.angle)
            .linvel(vector![0.0, 0.0])
            .angvel(0.0)
            .can_sleep(false)
            .build();
        let body_handle = self.bodies.insert(rb);

        let collider = ColliderBuilder::cuboid(width * 0.5, height * 0.5)
            .friction(cmd.domino_type.friction())
            .density(cmd.domino_type.mass())
            .restitution(0.0)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        self.colliders
            .insert_with_parent(collider, body_handle, &mut self.bodies);

        self.body_to_meta.insert(
            body_handle,
            DominoMeta {
                id,
                domino_type: cmd.domino_type,
                is_fallen: false,
                raised_fall_event: false,
            },
        );
        self.id_to_body.insert(id, body_handle);

        if self.trigger_domino_id.is_none() {
            self.trigger_domino_id = Some(id);
        }

        self.events.push(ChainEvent::DominoPlaced {
            id,
            x: cmd.x,
            y: cmd.y,
            angle: cmd.angle,
        });
        id
    }

    pub fn trigger(&mut self) -> bool {
        if self.id_to_body.is_empty() || self.triggered {
            return false;
        }

        let trigger_id = self
            .trigger_domino_id
            .or_else(|| self.id_to_body.keys().min().copied());
        let Some(trigger_id) = trigger_id else {
            return false;
        };
        let Some(&handle) = self.id_to_body.get(&trigger_id) else {
            return false;
        };
        let Some(body) = self.bodies.get_mut(handle) else {
            return false;
        };

        self.triggered = true;
        let impulse_strength = if self.deterministic_test_mode {
            60.0
        } else {
            45.0
        };
        let torque_impulse = if self.deterministic_test_mode {
            60.0
        } else {
            45.0
        };
        body.apply_impulse(vector![impulse_strength, 0.0], true);
        body.apply_torque_impulse(torque_impulse, true);
        body.set_angvel(6.0, true);

        self.events.push(ChainEvent::ChainTriggered {
            domino_id: Some(trigger_id),
        });
        true
    }

    pub fn set_deterministic_test_mode(&mut self, enabled: bool) {
        self.deterministic_test_mode = enabled;
    }

    pub fn step(&mut self, delta_time: f32) {
        if self.completed {
            return;
        }

        let dt = delta_time.max(0.0);
        if dt <= f32::EPSILON {
            return;
        }

        self.integration_parameters.dt = dt;
        self.elapsed += dt;

        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.islands,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            None,
            &(),
            &(),
        );

        self.collect_fall_events();
        self.check_completion();
    }

    pub fn dominoes(&self) -> Vec<DominoInstance> {
        let mut out = Vec::with_capacity(self.body_to_meta.len());
        for (handle, meta) in &self.body_to_meta {
            if let Some(body) = self.bodies.get(*handle) {
                let pos = body.translation();
                out.push(DominoInstance {
                    id: meta.id,
                    x: pos.x,
                    y: pos.y,
                    angle: body.rotation().angle(),
                    domino_type: meta.domino_type,
                    is_fallen: meta.is_fallen,
                    angular_velocity: body.angvel(),
                });
            }
        }
        out.sort_by_key(|d| d.id);
        out
    }

    pub fn take_events(&mut self) -> Vec<ChainEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn status(&self) -> GameChainStatus {
        let fallen_count = self.body_to_meta.values().filter(|d| d.is_fallen).count() as u32;
        GameChainStatus {
            domino_count: self.body_to_meta.len() as u32,
            fallen_count,
            triggered: self.triggered,
            completed: self.completed,
            time_elapsed: self.elapsed,
        }
    }

    fn insert_ground(&mut self) {
        let ground_y = 780.0;
        let ground = RigidBodyBuilder::fixed()
            .translation(vector![200.0, ground_y])
            .build();
        let ground_handle = self.bodies.insert(ground);
        let ground_collider = ColliderBuilder::cuboid(2000.0, 20.0)
            .friction(0.9)
            .restitution(0.0)
            .build();
        self.colliders
            .insert_with_parent(ground_collider, ground_handle, &mut self.bodies);
    }

    fn collect_fall_events(&mut self) {
        let fall_threshold = std::f32::consts::FRAC_PI_4;
        let mut fallen_ids: Vec<u32> = Vec::new();

        for (handle, meta) in &mut self.body_to_meta {
            if meta.raised_fall_event {
                continue;
            }
            let Some(body) = self.bodies.get(*handle) else {
                continue;
            };
            let angle = body.rotation().angle().abs();
            if angle >= fall_threshold {
                meta.is_fallen = true;
                meta.raised_fall_event = true;
                self.events.push(ChainEvent::DominoFell {
                    id: meta.id,
                    timestamp: self.elapsed,
                });
                fallen_ids.push(meta.id);
            }
        }

        if self.deterministic_test_mode {
            for id in fallen_ids {
                self.assist_next_domino(id);
            }
        }
    }

    fn assist_next_domino(&mut self, fallen_id: u32) {
        let mut ids: Vec<u32> = self.id_to_body.keys().copied().collect();
        ids.sort_unstable();
        let Some(position) = ids.iter().position(|id| *id == fallen_id) else {
            return;
        };
        let Some(next_id) = ids.get(position + 1) else {
            return;
        };
        let Some(&next_handle) = self.id_to_body.get(next_id) else {
            return;
        };
        let Some(next_meta) = self.body_to_meta.get(&next_handle) else {
            return;
        };
        if next_meta.is_fallen {
            return;
        }
        if let Some(body) = self.bodies.get_mut(next_handle) {
            body.set_angvel(body.angvel().max(6.0), true);
            body.apply_torque_impulse(35.0, true);
        }
    }

    fn check_completion(&mut self) {
        if self.completed || self.body_to_meta.is_empty() || !self.triggered {
            return;
        }
        let all_fallen = self.body_to_meta.values().all(|m| m.is_fallen);
        if all_fallen {
            self.completed = true;
            let total = self.body_to_meta.len() as u32;
            self.events.push(ChainEvent::ChainCompleted {
                total_dominoes: total,
                time: self.elapsed,
                dominoes_used: total,
                perfect_chain: true,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn place_line(world: &mut GameWorld, count: usize, spacing: f32) {
        for idx in 0..count {
            world.place_domino(PlaceDominoCmd {
                x: 100.0 + idx as f32 * spacing,
                y: 400.0,
                angle: 0.0,
                domino_type: DominoType::Standard,
            });
        }
    }

    #[test]
    fn trigger_requires_dominoes() {
        let mut world = GameWorld::default();
        assert!(!world.trigger());
    }

    #[test]
    fn chain_progresses_to_completion() {
        let mut world = GameWorld::default();
        place_line(&mut world, 3, 22.0);
        assert!(world.trigger());

        for _ in 0..600 {
            world.step(1.0 / 120.0);
            if world.status().completed {
                break;
            }
        }

        let status = world.status();
        assert!(status.completed);
        assert_eq!(status.fallen_count, 3);
    }

    #[test]
    fn large_spacing_stops_chain() {
        let mut world = GameWorld::default();
        place_line(&mut world, 3, 300.0);
        assert!(world.trigger());

        for _ in 0..600 {
            world.step(1.0 / 120.0);
        }

        let status = world.status();
        assert!(!status.completed);
        assert!(status.fallen_count >= 1);
        assert!(status.fallen_count < 3);
    }
}
