use crate::{ChainEvent, DominoType, PlaceDominoCmd};

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

#[derive(Debug, Default)]
pub struct GameWorld {
    dominoes: Vec<DominoInstance>,
    events: Vec<ChainEvent>,
    elapsed: f32,
    next_id: u32,
    triggered: bool,
    completed: bool,
}

impl GameWorld {
    pub fn reset(&mut self) {
        self.dominoes.clear();
        self.events.clear();
        self.elapsed = 0.0;
        self.next_id = 0;
        self.triggered = false;
        self.completed = false;
        self.events.push(ChainEvent::LevelReset);
    }

    pub fn place_domino(&mut self, cmd: PlaceDominoCmd) -> u32 {
        self.next_id += 1;
        let id = self.next_id;
        self.dominoes.push(DominoInstance {
            id,
            x: cmd.x,
            y: cmd.y,
            angle: cmd.angle,
            domino_type: cmd.domino_type,
            is_fallen: false,
            angular_velocity: 0.0,
        });
        self.events.push(ChainEvent::DominoPlaced {
            id,
            x: cmd.x,
            y: cmd.y,
            angle: cmd.angle,
        });
        id
    }

    pub fn trigger(&mut self) -> bool {
        if self.dominoes.is_empty() || self.triggered {
            return false;
        }
        self.triggered = true;
        let first_id = self.dominoes.first().map(|d| d.id);
        if let Some(first) = self.dominoes.first_mut() {
            first.angular_velocity = base_angular_speed(first.domino_type);
        }
        self.events.push(ChainEvent::ChainTriggered {
            domino_id: first_id,
        });
        true
    }

    pub fn step(&mut self, delta_time: f32) {
        if self.completed {
            return;
        }

        let dt = delta_time.max(0.0);
        self.elapsed += dt;

        if !self.triggered {
            return;
        }

        for idx in 0..self.dominoes.len() {
            if self.dominoes[idx].is_fallen || self.dominoes[idx].angular_velocity <= 0.0 {
                continue;
            }

            let domino_type = self.dominoes[idx].domino_type;
            self.dominoes[idx].angle += self.dominoes[idx].angular_velocity * dt;
            self.dominoes[idx].angular_velocity =
                (self.dominoes[idx].angular_velocity - 0.6 * dt).max(0.0);

            if self.dominoes[idx].angle >= fall_threshold(domino_type) {
                self.dominoes[idx].angle = std::f32::consts::FRAC_PI_2;
                self.dominoes[idx].is_fallen = true;
                self.events.push(ChainEvent::DominoFell {
                    id: self.dominoes[idx].id,
                    timestamp: self.elapsed,
                });

                if idx + 1 < self.dominoes.len() {
                    let dx = (self.dominoes[idx + 1].x - self.dominoes[idx].x).abs();
                    let range = 80.0;
                    if dx <= range {
                        let transfer = ((range - dx) / range).clamp(0.2, 1.0);
                        let target_type = self.dominoes[idx + 1].domino_type;
                        let impulse = base_angular_speed(target_type) * transfer;
                        self.dominoes[idx + 1].angular_velocity =
                            self.dominoes[idx + 1].angular_velocity.max(impulse);
                    }
                }
            }
        }

        let all_fallen = self.dominoes.iter().all(|domino| domino.is_fallen);
        if all_fallen && !self.dominoes.is_empty() {
            self.completed = true;
            self.events.push(ChainEvent::ChainCompleted {
                total_dominoes: self.dominoes.len() as u32,
                time: self.elapsed,
                dominoes_used: self.dominoes.len() as u32,
                perfect_chain: true,
            });
        }
    }

    pub fn dominoes(&self) -> &[DominoInstance] {
        &self.dominoes
    }

    pub fn take_events(&mut self) -> Vec<ChainEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn status(&self) -> GameChainStatus {
        let fallen_count = self.dominoes.iter().filter(|d| d.is_fallen).count() as u32;
        GameChainStatus {
            domino_count: self.dominoes.len() as u32,
            fallen_count,
            triggered: self.triggered,
            completed: self.completed,
            time_elapsed: self.elapsed,
        }
    }
}

fn fall_threshold(domino_type: DominoType) -> f32 {
    match domino_type {
        DominoType::Standard => 1.2,
        DominoType::Heavy => 1.35,
        DominoType::Tall => 0.95,
    }
}

fn base_angular_speed(domino_type: DominoType) -> f32 {
    match domino_type {
        DominoType::Standard => 3.3,
        DominoType::Heavy => 2.2,
        DominoType::Tall => 4.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn place_line(world: &mut GameWorld, count: usize, spacing: f32) {
        for idx in 0..count {
            world.place_domino(PlaceDominoCmd {
                x: 100.0 + idx as f32 * spacing,
                y: 200.0,
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
        place_line(&mut world, 3, 40.0);

        assert!(world.trigger());

        for _ in 0..240 {
            world.step(1.0 / 60.0);
            if world.status().completed {
                break;
            }
        }

        let status = world.status();
        assert!(status.completed);
        assert_eq!(status.fallen_count, 3);

        let events = world.take_events();
        assert!(events
            .iter()
            .any(|e| matches!(e, ChainEvent::ChainTriggered { .. })));
        assert!(events
            .iter()
            .any(|e| matches!(e, ChainEvent::ChainCompleted { .. })));
    }

    #[test]
    fn large_spacing_stops_chain() {
        let mut world = GameWorld::default();
        place_line(&mut world, 3, 200.0);
        assert!(world.trigger());

        for _ in 0..240 {
            world.step(1.0 / 60.0);
        }

        let status = world.status();
        assert!(!status.completed);
        assert!(status.fallen_count >= 1);
        assert!(status.fallen_count < 3);
    }
}
