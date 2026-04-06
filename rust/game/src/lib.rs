// Domino Chain Reaction - Physics Engine
// Built with Bevy + Rapier2D

pub mod api_types;
pub mod world;

pub use api_types::*;
pub use world::*;

// Re-export commonly used types
pub use bevy::prelude::*;
pub use bevy_rapier2d::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Basic sanity test
        assert_eq!(2 + 2, 4);
    }
}
