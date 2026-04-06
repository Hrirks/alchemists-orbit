use domino_game::{ChainEvent, DominoType, FailureReason, GameWorld, PlaceDominoCmd};
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
    static WORLD: RefCell<GameWorld> = RefCell::new(GameWorld::default());
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

fn map_event(event: ChainEvent) -> BridgeEvent {
    match event {
        ChainEvent::DominoPlaced { id, x, y, angle } => BridgeEvent {
            kind: "DominoPlaced".to_string(),
            domino_id: Some(id),
            x: Some(x),
            y: Some(y),
            angle: Some(angle),
            timestamp: None,
            total_dominoes: None,
        },
        ChainEvent::ChainTriggered { domino_id } => BridgeEvent {
            kind: "ChainTriggered".to_string(),
            domino_id,
            x: None,
            y: None,
            angle: None,
            timestamp: None,
            total_dominoes: None,
        },
        ChainEvent::DominoFell { id, timestamp } => BridgeEvent {
            kind: "DominoFell".to_string(),
            domino_id: Some(id),
            x: None,
            y: None,
            angle: None,
            timestamp: Some(timestamp),
            total_dominoes: None,
        },
        ChainEvent::ChainCompleted {
            total_dominoes,
            time,
            ..
        } => BridgeEvent {
            kind: "ChainCompleted".to_string(),
            domino_id: None,
            x: None,
            y: None,
            angle: None,
            timestamp: Some(time),
            total_dominoes: Some(total_dominoes),
        },
        ChainEvent::LevelFailed { reason } => BridgeEvent {
            kind: match reason {
                FailureReason::Timeout => "LevelFailedTimeout",
                FailureReason::StuckChain => "LevelFailedStuckChain",
                FailureReason::TooManyDominoes => "LevelFailedTooManyDominoes",
            }
            .to_string(),
            domino_id: None,
            x: None,
            y: None,
            angle: None,
            timestamp: None,
            total_dominoes: None,
        },
        ChainEvent::LevelReset => BridgeEvent {
            kind: "LevelReset".to_string(),
            domino_id: None,
            x: None,
            y: None,
            angle: None,
            timestamp: Some(0.0),
            total_dominoes: Some(0),
        },
    }
}

#[frb(sync)]
pub fn reset_world() {
    WORLD.with(|world| {
        world.borrow_mut().reset();
    });
}

#[frb(sync)]
pub fn set_deterministic_test_mode(enabled: bool) {
    WORLD.with(|world| {
        world.borrow_mut().set_deterministic_test_mode(enabled);
    });
}

#[frb(sync)]
pub fn place_domino(x: f32, y: f32, angle: f32, domino_type: u8) -> u32 {
    WORLD.with(|world| {
        let mut world = world.borrow_mut();
        world.place_domino(PlaceDominoCmd {
            x,
            y,
            angle,
            domino_type: normalize_domino_type(domino_type),
        })
    })
}

#[frb(sync)]
pub fn process_commands(commands: Vec<BridgePlaceDominoCmd>) -> u32 {
    WORLD.with(|world| {
        let mut world = world.borrow_mut();
        let mut placed = 0;
        for cmd in commands {
            world.place_domino(PlaceDominoCmd {
                x: cmd.x,
                y: cmd.y,
                angle: cmd.angle,
                domino_type: normalize_domino_type(cmd.domino_type),
            });
            placed += 1;
        }
        placed
    })
}

#[frb(sync)]
pub fn trigger_domino_push() -> bool {
    WORLD.with(|world| world.borrow_mut().trigger())
}

#[frb(sync)]
pub fn step(delta_time: f32) {
    WORLD.with(|world| {
        world.borrow_mut().step(delta_time);
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
            .dominoes()
            .iter()
            .map(|domino| DominoTransform {
                id: domino.id,
                x: domino.x,
                y: domino.y,
                angle: domino.angle,
                domino_type: domino_type_to_u8(domino.domino_type),
                is_fallen: domino.is_fallen,
            })
            .collect()
    })
}

#[frb(sync)]
pub fn get_events() -> Vec<BridgeEvent> {
    WORLD.with(|world| {
        world
            .borrow_mut()
            .take_events()
            .into_iter()
            .map(map_event)
            .collect()
    })
}

#[frb(sync)]
pub fn get_chain_status() -> ChainStatus {
    WORLD.with(|world| {
        let status = world.borrow().status();
        ChainStatus {
            domino_count: status.domino_count,
            fallen_count: status.fallen_count,
            triggered: status.triggered,
            completed: status.completed,
            time_elapsed: status.time_elapsed,
        }
    })
}

#[frb(sync)]
pub fn time_elapsed() -> f32 {
    WORLD.with(|world| world.borrow().status().time_elapsed)
}

#[frb(sync)]
pub fn domino_count() -> u32 {
    WORLD.with(|world| world.borrow().status().domino_count)
}
