use domino_game::DominoType;
use flutter_rust_bridge::frb;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct BridgePlaceDominoCmd {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub domino_type: u8,
}

#[derive(Debug, Clone)]
pub struct DominoTransform {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub domino_type: u8,
    pub is_fallen: bool,
}

#[derive(Debug, Clone)]
pub struct BridgeEvent {
    pub kind: String,
    pub domino_id: Option<u32>,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub angle: Option<f32>,
    pub timestamp: Option<f32>,
    pub total_dominoes: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct ChainStatus {
    pub domino_count: u32,
    pub fallen_count: u32,
    pub triggered: bool,
    pub completed: bool,
    pub time_elapsed: f32,
}

#[derive(Debug, Clone)]
pub struct SimDomino {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub domino_type: u8,
    pub is_fallen: bool,
    pub angular_velocity: f32,
}

#[derive(Debug, Clone)]
pub struct PhysicsWorld {
    pub dominoes: Vec<SimDomino>,
    pub events: Vec<BridgeEvent>,
    pub elapsed: f32,
    pub next_id: u32,
    pub triggered: bool,
    pub completed: bool,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self {
            dominoes: Vec::new(),
            events: Vec::new(),
            elapsed: 0.0,
            next_id: 0,
            triggered: false,
            completed: false,
        }
    }
}

thread_local! {
    static WORLD: RefCell<PhysicsWorld> = RefCell::new(PhysicsWorld::default());
}

fn normalize_domino_type(domino_type: u8) -> DominoType {
    match domino_type {
        1 => DominoType::Heavy,
        2 => DominoType::Tall,
        _ => DominoType::Standard,
    }
}

fn domino_type_to_u8(domino_type: DominoType) -> u8 {
    match domino_type {
        DominoType::Standard => 0,
        DominoType::Heavy => 1,
        DominoType::Tall => 2,
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

#[frb(sync)]
pub fn reset_world() {
    WORLD.with(|world| {
        let mut world = world.borrow_mut();
        world.dominoes.clear();
        world.events.clear();
        world.elapsed = 0.0;
        world.next_id = 0;
        world.triggered = false;
        world.completed = false;
        world.events.push(BridgeEvent {
            kind: "LevelReset".to_string(),
            domino_id: None,
            x: None,
            y: None,
            angle: None,
            timestamp: Some(0.0),
            total_dominoes: None,
        });
    });
}

#[frb(sync)]
pub fn place_domino(x: f32, y: f32, angle: f32, domino_type: u8) -> u32 {
    WORLD.with(|world| {
        let mut world = world.borrow_mut();
        let normalized_type = normalize_domino_type(domino_type);
        let normalized_domino_type = domino_type_to_u8(normalized_type);
        world.next_id += 1;
        let id = world.next_id;
        let elapsed = world.elapsed;
        let total_dominoes = world.dominoes.len() as u32 + 1;
        world.dominoes.push(SimDomino {
            id,
            x,
            y,
            angle,
            domino_type: normalized_domino_type,
            is_fallen: false,
            angular_velocity: 0.0,
        });
        world.events.push(BridgeEvent {
            kind: "DominoPlaced".to_string(),
            domino_id: Some(id),
            x: Some(x),
            y: Some(y),
            angle: Some(angle),
            timestamp: Some(elapsed),
            total_dominoes: Some(total_dominoes),
        });
        id
    })
}

#[frb(sync)]
pub fn process_commands(commands: Vec<BridgePlaceDominoCmd>) -> u32 {
    let mut placed = 0;
    for cmd in commands {
        place_domino(cmd.x, cmd.y, cmd.angle, cmd.domino_type);
        placed += 1;
    }
    placed
}

#[frb(sync)]
pub fn trigger_domino_push() -> bool {
    WORLD.with(|world| {
        let mut world = world.borrow_mut();
        if world.dominoes.is_empty() {
            return false;
        }
        if world.triggered {
            return false;
        }
        world.triggered = true;

        let first_id = world.dominoes.first().map(|d| d.id);
        if let Some(first) = world.dominoes.first_mut() {
            first.angular_velocity = base_angular_speed(normalize_domino_type(first.domino_type));
        }

        let elapsed = world.elapsed;
        let total_dominoes = world.dominoes.len() as u32;
        world.events.push(BridgeEvent {
            kind: "ChainTriggered".to_string(),
            domino_id: first_id,
            x: None,
            y: None,
            angle: None,
            timestamp: Some(elapsed),
            total_dominoes: Some(total_dominoes),
        });
        true
    })
}

#[frb(sync)]
pub fn step(delta_time: f32) {
    WORLD.with(|world| {
        let mut world = world.borrow_mut();
        if world.completed {
            return;
        }

        let dt = delta_time.max(0.0);
        world.elapsed += dt;

        if !world.triggered {
            return;
        }

        let elapsed = world.elapsed;
        let total_dominoes = world.dominoes.len() as u32;

        let mut pending_events: Vec<BridgeEvent> = Vec::new();

        for idx in 0..world.dominoes.len() {
            if world.dominoes[idx].is_fallen {
                continue;
            }

            if world.dominoes[idx].angular_velocity <= 0.0 {
                continue;
            }

            let domino_type = normalize_domino_type(world.dominoes[idx].domino_type);
            world.dominoes[idx].angle += world.dominoes[idx].angular_velocity * dt;
            world.dominoes[idx].angular_velocity =
                (world.dominoes[idx].angular_velocity - 0.6 * dt).max(0.0);

            if world.dominoes[idx].angle >= fall_threshold(domino_type) {
                world.dominoes[idx].angle = std::f32::consts::FRAC_PI_2;
                world.dominoes[idx].is_fallen = true;

                pending_events.push(BridgeEvent {
                    kind: "DominoFell".to_string(),
                    domino_id: Some(world.dominoes[idx].id),
                    x: Some(world.dominoes[idx].x),
                    y: Some(world.dominoes[idx].y),
                    angle: Some(world.dominoes[idx].angle),
                    timestamp: Some(elapsed),
                    total_dominoes: Some(total_dominoes),
                });

                if idx + 1 < world.dominoes.len() {
                    let dx = (world.dominoes[idx + 1].x - world.dominoes[idx].x).abs();
                    let range = 80.0;
                    if dx <= range {
                        let transfer = ((range - dx) / range).clamp(0.2, 1.0);
                        let target_type =
                            normalize_domino_type(world.dominoes[idx + 1].domino_type);
                        let existing = world.dominoes[idx + 1].angular_velocity;
                        let impulse = base_angular_speed(target_type) * transfer;
                        world.dominoes[idx + 1].angular_velocity = existing.max(impulse);
                    }
                }
            }
        }

        if !pending_events.is_empty() {
            world.events.extend(pending_events);
        }

        let all_fallen = world.dominoes.iter().all(|domino| domino.is_fallen);
        if all_fallen && !world.dominoes.is_empty() {
            world.completed = true;
            world.events.push(BridgeEvent {
                kind: "ChainCompleted".to_string(),
                domino_id: None,
                x: None,
                y: None,
                angle: None,
                timestamp: Some(elapsed),
                total_dominoes: Some(total_dominoes),
            });
        }
    });
}

#[frb(sync)]
pub fn step_multiple(steps: u32, delta_time: f32) {
    for _ in 0..steps {
        step(delta_time);
    }
}

#[frb(sync)]
pub fn get_domino_transforms() -> Vec<DominoTransform> {
    WORLD.with(|world| {
        world
            .borrow()
            .dominoes
            .iter()
            .map(|domino| DominoTransform {
                id: domino.id,
                x: domino.x,
                y: domino.y,
                angle: domino.angle,
                domino_type: domino.domino_type,
                is_fallen: domino.is_fallen,
            })
            .collect()
    })
}

#[frb(sync)]
pub fn get_events() -> Vec<BridgeEvent> {
    WORLD.with(|world| {
        let mut world = world.borrow_mut();
        std::mem::take(&mut world.events)
    })
}

#[frb(sync)]
pub fn get_chain_status() -> ChainStatus {
    WORLD.with(|world| {
        let world = world.borrow();
        let fallen_count = world
            .dominoes
            .iter()
            .filter(|domino| domino.is_fallen)
            .count() as u32;
        ChainStatus {
            domino_count: world.dominoes.len() as u32,
            fallen_count,
            triggered: world.triggered,
            completed: world.completed,
            time_elapsed: world.elapsed,
        }
    })
}

#[frb(sync)]
pub fn time_elapsed() -> f32 {
    WORLD.with(|world| world.borrow().elapsed)
}

#[frb(sync)]
pub fn domino_count() -> u32 {
    WORLD.with(|world| world.borrow().dominoes.len() as u32)
}
