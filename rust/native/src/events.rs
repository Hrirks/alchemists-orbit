use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    OrbSpawned {
        id: u32,
        tier: u8,
        x: f32,
        y: f32,
    },
    OrbMerged {
        orb1_id: u32,
        orb2_id: u32,
        new_orb_id: u32,
        new_tier: u8,
        x: f32,
        y: f32,
    },
    ScoreUpdated {
        score: u32,
    },
    LevelUp {
        level: u32,
    },
    GameOver,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollisionEvent {
    pub orb1_id: u32,
    pub orb2_id: u32,
}
