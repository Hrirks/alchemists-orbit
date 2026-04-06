use alchemists_orbit_native::{GameEvent, OrbTier, PhysicsWorld};
use flutter_rust_bridge::frb;
use std::sync::{Arc, Mutex};

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

/// The main API exposed to Flutter
pub struct GameApi {
    physics_world: Arc<Mutex<PhysicsWorld>>,
}

// SAFETY: PhysicsWorld operations are protected by Mutex
// and we ensure all Rust API calls happen on the same thread
unsafe impl Send for GameApi {}
unsafe impl Sync for GameApi {}

impl GameApi {
    #[frb(sync)]
    pub fn new() -> Self {
        Self {
            physics_world: Arc::new(Mutex::new(PhysicsWorld::new())),
        }
    }

    /// Drop an orb at the specified position
    #[frb(sync)]
    pub fn drop_orb(&self, x: f32, y: f32, tier: u8) -> Result<(), String> {
        let mut world = self
            .physics_world
            .lock()
            .map_err(|e: std::sync::PoisonError<_>| e.to_string())?;
        world.drop_orb(x, y, tier);
        Ok(())
    }

    /// Step the physics simulation forward by delta_time seconds
    #[frb(sync)]
    pub fn step_physics(&self, delta_time: f32) -> Result<(), String> {
        let mut world = self
            .physics_world
            .lock()
            .map_err(|e: std::sync::PoisonError<_>| e.to_string())?;
        world.step(delta_time);
        Ok(())
    }
}
