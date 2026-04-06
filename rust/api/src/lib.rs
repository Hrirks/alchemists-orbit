use alchemists_orbit_native::{PhysicsWorld, OrbTier, GameEvent};
use flutter_rust_bridge::frb;
use std::sync::{Arc, Mutex};

/// The main API exposed to Flutter
pub struct GameApi {
    physics_world: Arc<Mutex<PhysicsWorld>>,
}

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
        let mut world = self.physics_world.lock().map_err(|e| e.to_string())?;
        world.drop_orb(x, y, tier);
        Ok(())
    }
    
    /// Step the physics simulation forward by delta_time seconds
    #[frb(sync)]
    pub fn step_physics(&self, delta_time: f32) -> Result<(), String> {
        let mut world = self.physics_world.lock().map_err(|e| e.to_string())?;
        world.step(delta_time);
        Ok(())
    }
    
    /// Initialize the physics engine
    #[frb(sync)]
    pub fn init() -> Self {
        Self::new()
    }
}

/// Stream of game events for Flutter to listen to
#[frb(stream)]
pub async fn game_events() -> impl futures::Stream<Item = GameEvent> {
    futures::stream::iter(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_api_creation() {
        let api = GameApi::new();
        assert!(api.drop_orb(0.0, 0.0, 1).is_ok());
    }
}
