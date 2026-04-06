// Bridge API for Domino Chain Reaction
// Exposes domino-game crate to Flutter via flutter_rust_bridge

use domino_game::DominoType;
pub use domino_game::PlaceDominoCmd;
use flutter_rust_bridge::frb;

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

// Re-export API types for Flutter bindings
// These will be automatically converted to Dart classes

/// Place a domino command
#[frb(sync)]
pub fn create_place_domino_cmd(x: f32, y: f32, angle: f32, domino_type: u8) -> PlaceDominoCmd {
    let dt = match domino_type {
        0 => DominoType::Standard,
        1 => DominoType::Heavy,
        2 => DominoType::Tall,
        _ => DominoType::Standard,
    };

    PlaceDominoCmd {
        x,
        y,
        angle,
        domino_type: dt,
    }
}

/// Get domino type properties for UI display
#[frb(sync)]
pub fn get_domino_dimensions(domino_type: u8) -> (f32, f32) {
    let dt = match domino_type {
        0 => DominoType::Standard,
        1 => DominoType::Heavy,
        2 => DominoType::Tall,
        _ => DominoType::Standard,
    };
    dt.dimensions()
}

// Note: The actual game API (PhysicsWorld, etc.) will be implemented in next issues
// For now, we just expose the type system to Flutter
