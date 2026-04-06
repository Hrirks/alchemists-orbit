use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod physics;
pub mod orb;
pub mod events;

pub use physics::PhysicsWorld;
pub use orb::{Orb, OrbTier};
pub use events::{GameEvent, CollisionEvent};

/// Initialize the Bevy physics engine
pub fn init_physics_engine() -> PhysicsWorld {
    PhysicsWorld::new()
}
