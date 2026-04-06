use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrbTier {
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3,
    Tier4 = 4,
    Tier5 = 5,
    Tier6 = 6,
    Tier7 = 7,
}

impl OrbTier {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(OrbTier::Tier1),
            2 => Some(OrbTier::Tier2),
            3 => Some(OrbTier::Tier3),
            4 => Some(OrbTier::Tier4),
            5 => Some(OrbTier::Tier5),
            6 => Some(OrbTier::Tier6),
            7 => Some(OrbTier::Tier7),
            _ => None,
        }
    }

    pub fn radius(&self) -> f32 {
        match self {
            OrbTier::Tier1 => 10.0,
            OrbTier::Tier2 => 15.0,
            OrbTier::Tier3 => 20.0,
            OrbTier::Tier4 => 25.0,
            OrbTier::Tier5 => 30.0,
            OrbTier::Tier6 => 35.0,
            OrbTier::Tier7 => 40.0,
        }
    }

    pub fn mass(&self) -> f32 {
        match self {
            OrbTier::Tier1 => 1.0,
            OrbTier::Tier2 => 2.0,
            OrbTier::Tier3 => 4.0,
            OrbTier::Tier4 => 8.0,
            OrbTier::Tier5 => 16.0,
            OrbTier::Tier6 => 32.0,
            OrbTier::Tier7 => 64.0,
        }
    }

    pub fn next_tier(&self) -> Option<Self> {
        match self {
            OrbTier::Tier1 => Some(OrbTier::Tier2),
            OrbTier::Tier2 => Some(OrbTier::Tier3),
            OrbTier::Tier3 => Some(OrbTier::Tier4),
            OrbTier::Tier4 => Some(OrbTier::Tier5),
            OrbTier::Tier5 => Some(OrbTier::Tier6),
            OrbTier::Tier6 => Some(OrbTier::Tier7),
            OrbTier::Tier7 => None, // Max tier
        }
    }
}

#[derive(Component, Debug)]
pub struct Orb {
    pub tier: OrbTier,
    pub id: u32,
}

#[derive(Bundle)]
pub struct OrbBundle {
    pub orb: Orb,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub restitution: Restitution,
    pub mass: ColliderMassProperties,
    pub external_force: ExternalForce,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl OrbBundle {
    pub fn new(id: u32, tier: OrbTier, x: f32, y: f32) -> Self {
        Self {
            orb: Orb { tier, id },
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(tier.radius()),
            restitution: Restitution::coefficient(0.7),
            mass: ColliderMassProperties::Mass(tier.mass()),
            external_force: ExternalForce::default(),
            transform: Transform::from_xyz(x, y, 0.0),
            global_transform: GlobalTransform::default(),
        }
    }
}
