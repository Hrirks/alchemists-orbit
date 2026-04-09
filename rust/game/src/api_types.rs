use serde::{Deserialize, Serialize};

// ============================================================================
// COMMANDS (Flutter → Rust)
// ============================================================================

/// Command to place a domino in the scene
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceDominoCmd {
    /// X position in screen coordinates (pixels from left)
    pub x: f32,
    /// Y position in screen coordinates (pixels from top)
    pub y: f32,
    /// Rotation angle in radians (0 = upright, π/2 = horizontal)
    pub angle: f32,
    /// Type of domino to place
    pub domino_type: DominoType,
}

/// Types of dominoes with different physics properties
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DominoType {
    /// Standard domino (10x50 pixels, light)
    Standard,
    /// Heavy domino (10x50 pixels, 2x mass)
    Heavy,
    /// Tall domino (10x75 pixels, unstable)
    Tall,
}

impl DominoType {
    /// Get domino dimensions (width, height) in pixels
    pub fn dimensions(&self) -> (f32, f32) {
        match self {
            DominoType::Standard => (10.0, 50.0),
            DominoType::Heavy => (10.0, 50.0),
            DominoType::Tall => (10.0, 75.0),
        }
    }

    /// Get domino mass for physics simulation
    pub fn mass(&self) -> f32 {
        match self {
            DominoType::Standard => 1.0,
            DominoType::Heavy => 2.0,
            DominoType::Tall => 1.2,
        }
    }

    /// Get friction coefficient
    pub fn friction(&self) -> f32 {
        match self {
            DominoType::Standard => 0.5,
            DominoType::Heavy => 0.7,
            DominoType::Tall => 0.4,
        }
    }
}

// ============================================================================
// EVENTS (Rust → Flutter)
// ============================================================================

/// Events sent from Rust physics engine to Flutter UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainEvent {
    /// Domino successfully placed in scene
    DominoPlaced { id: u32, x: f32, y: f32, angle: f32 },

    /// First domino has been triggered to start chain reaction
    ChainTriggered { domino_id: Option<u32> },

    /// Domino has fallen (rotation > threshold)
    DominoFell {
        id: u32,
        /// Timestamp in seconds since chain started
        timestamp: f32,
    },

    /// All dominoes have fallen - level complete!
    ChainCompleted {
        total_dominoes: u32,
        /// Total time from trigger to last fall
        time: f32,
        /// Number of dominoes used (for star rating)
        dominoes_used: u32,
        /// True if no dominoes were left standing
        perfect_chain: bool,
    },

    /// Level failed (timeout, stuck chain, or exceeded limit)
    LevelFailed { reason: FailureReason },

    /// Level reset to initial state
    LevelReset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FailureReason {
    /// Timer ran out
    Timeout,
    /// No dominoes fell in 5 seconds
    StuckChain,
    /// Exceeded max domino placement limit
    TooManyDominoes,
}

// ============================================================================
// REPLAY SYSTEM
// ============================================================================

/// A single frame of recorded physics state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayFrame {
    /// Timestamp in seconds from start of recording
    pub timestamp: f32,
    /// State of all dominoes at this frame
    pub domino_states: Vec<DominoState>,
}

/// State of a single domino at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DominoState {
    /// Unique domino ID
    pub id: u32,
    /// X position in world coordinates
    pub x: f32,
    /// Y position in world coordinates
    pub y: f32,
    /// Rotation angle in radians
    pub angle: f32,
    /// Domino type
    pub domino_type: DominoType,
    /// True if domino has fallen past threshold
    pub is_fallen: bool,
    /// Angular velocity (for smooth interpolation)
    pub angular_velocity: f32,
}

/// Full replay buffer containing all recorded frames
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayBuffer {
    /// All recorded frames (max 600 frames = 10s at 60fps)
    pub frames: Vec<ReplayFrame>,
    /// Total duration of recording
    pub duration: f32,
    /// Level ID that was recorded
    pub level_id: u32,
}

impl ReplayBuffer {
    /// Create empty replay buffer
    pub fn new(level_id: u32) -> Self {
        Self {
            frames: Vec::with_capacity(600), // Pre-allocate for 10s
            duration: 0.0,
            level_id,
        }
    }

    /// Add a frame to the buffer (ring buffer behavior)
    pub fn push_frame(&mut self, frame: ReplayFrame) {
        if self.frames.len() >= 600 {
            // Ring buffer: remove oldest frame
            self.frames.remove(0);
        }
        self.duration = frame.timestamp;
        self.frames.push(frame);
    }

    /// Get frame at specific timestamp (interpolated if needed)
    pub fn get_frame_at(&self, timestamp: f32) -> Option<&ReplayFrame> {
        self.frames
            .iter()
            .find(|f| (f.timestamp - timestamp).abs() < 0.016) // Within 1 frame tolerance
    }
}

// ============================================================================
// LEVEL DEFINITION
// ============================================================================

/// Level definition loaded from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelDefinition {
    /// Unique level ID (1-30)
    pub level_id: u32,
    /// Display name
    pub name: String,
    /// Maximum dominoes player can place
    pub max_dominoes: u32,
    /// Time limit in seconds (0 = no limit)
    pub time_limit: f32,
    /// Static obstacles in the scene
    pub obstacles: Vec<Obstacle>,
    /// Pre-placed dominoes that cannot be moved
    pub starting_dominoes: Vec<StartingDomino>,
    /// Target thresholds for star ratings
    pub star_thresholds: StarThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obstacle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    /// Obstacle type (Wall, Gap, etc.)
    pub obstacle_type: ObstacleType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObstacleType {
    Wall,
    Gap,
    Platform,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartingDomino {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub domino_type: DominoType,
    /// If true, this is the trigger domino (player taps it)
    pub is_trigger: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarThresholds {
    /// Time threshold for 3 stars (seconds)
    pub time_3_star: f32,
    /// Max dominoes for 3 stars
    pub dominoes_3_star: u32,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domino_type_properties() {
        let standard = DominoType::Standard;
        assert_eq!(standard.dimensions(), (10.0, 50.0));
        assert_eq!(standard.mass(), 1.0);

        let heavy = DominoType::Heavy;
        assert_eq!(heavy.mass(), 2.0);
    }

    #[test]
    fn test_replay_buffer_ring() {
        let mut buffer = ReplayBuffer::new(1);

        // Add 650 frames (exceeds 600 limit)
        for i in 0..650 {
            buffer.push_frame(ReplayFrame {
                timestamp: i as f32 * 0.016,
                domino_states: vec![],
            });
        }

        // Should only keep last 600
        assert_eq!(buffer.frames.len(), 600);

        // First frame should be frame 50 (oldest kept)
        assert!((buffer.frames[0].timestamp - (50.0 * 0.016)).abs() < 0.001);
    }

    #[test]
    fn test_serde_roundtrip() {
        let cmd = PlaceDominoCmd {
            x: 100.0,
            y: 200.0,
            angle: 1.57,
            domino_type: DominoType::Heavy,
        };

        let json = serde_json::to_string(&cmd).unwrap();
        let parsed: PlaceDominoCmd = serde_json::from_str(&json).unwrap();

        assert_eq!(cmd.x, parsed.x);
        assert_eq!(cmd.domino_type, parsed.domino_type);
    }
}
