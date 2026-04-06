# Alchemist's Orbit - AI Development Context

This file is the persistent handoff for AI agents working on this project.

## Project Overview

**Alchemist's Orbit** is a physics-based mobile game combining rotating gravity mechanics with merge gameplay. Built with Flutter for UI and Bevy for physics simulation, connected via flutter_rust_bridge.

## Architecture

### Stack
- **Frontend**: Flutter 3.41+ (Dart 3.11+)
- **Physics Engine**: Bevy 0.14 + Rapier 2D physics
- **Bridge**: flutter_rust_bridge 2.5
- **Platforms**: iOS, Android, Web (via Wasm)

### Project Structure
```
alchemists-orbit/
├── mobile/              # Flutter application
│   ├── lib/            # Dart source code
│   │   ├── src/        # App modules
│   │   ├── bridge/     # Generated FRB code (auto-generated)
│   │   └── main.dart   # Entry point
│   └── pubspec.yaml    # Flutter dependencies
├── rust/               # Rust workspace
│   ├── native/         # Bevy physics engine
│   │   ├── src/
│   │   │   ├── lib.rs      # Main library
│   │   │   ├── physics.rs  # Gravity & simulation
│   │   │   ├── orb.rs      # Orb components
│   │   │   └── events.rs   # Game events
│   │   └── Cargo.toml
│   ├── api/            # flutter_rust_bridge API
│   │   ├── src/lib.rs  # Exposed API to Flutter
│   │   └── Cargo.toml
│   └── Cargo.toml      # Workspace config
├── scripts/            # Build & dev scripts
├── maestro/            # E2E test flows
├── Makefile           # Build automation
└── MEMORY.md          # This file
```

## Core Game Mechanics

### 1. Circular Physics
- **Gravity Well**: Central point that attracts all orbs
- **Force Calculation**: Inverse square law gravity
- **Rotation**: Gravity well rotates over time, increasing difficulty

### 2. Orb System
- **7 Tiers**: Each tier has unique size, mass, and visual
- **Properties**:
  - Tier 1: radius 10px, mass 1.0
  - Tier 2: radius 15px, mass 2.0
  - ...scaling exponentially to Tier 7
- **Physics**: Dynamic rigidbodies with Rapier collision detection

### 3. Merging Logic
- When two orbs of same tier collide → spawn next tier orb
- Merge location: midpoint between colliding orbs
- Particle effects on merge
- Score increases based on tier created

### 4. Progression
- Rotation speed increases gradually
- Score multipliers unlock at level milestones
- Infinite gameplay with increasing difficulty

## Development Workflows

### Initial Setup
```bash
# Install dependencies
cargo install flutter_rust_bridge_codegen
rustup target add aarch64-apple-ios x86_64-apple-ios
rustup target add aarch64-linux-android armv7-linux-androideabi

cd mobile && flutter pub get
```

### Build Commands
```bash
make bridge          # Generate FRB bindings
make build-rust      # Build Rust library
make test-rust       # Run Rust tests
make test-flutter    # Run Flutter tests
make ci              # Run all CI checks
make doctor          # Check tool versions
```

### Running the App
```bash
# iOS
cd mobile && flutter run -d ios

# Android
cd mobile && flutter run -d android

# Web (testing only - no physics)
cd mobile && flutter run -d chrome
```

### Code Generation
When you modify `rust/api/src/lib.rs` with new `#[frb]` annotated functions:
```bash
make bridge
cd mobile && flutter pub get
```

## Standard AI Execution Flow

1. Run `make doctor` to verify tools.
2. Generate bridge code: `make bridge`
3. Build Rust: `make build-rust`
4. Run tests: `make test-rust && make test-flutter`
5. Run app: `cd mobile && flutter run -d <device>`
6. For CI validation: `make ci`

## Key Files

### Rust Physics Engine
- **rust/native/src/physics.rs**: Main physics loop, gravity system
- **rust/native/src/orb.rs**: Orb components, tier definitions
- **rust/native/src/events.rs**: Event types for Flutter communication

### Rust API Layer
- **rust/api/src/lib.rs**: FFI-safe API exposed to Flutter
  - `GameApi::new()` - Initialize physics world
  - `drop_orb(x, y, tier)` - Spawn orb at position
  - `step_physics(dt)` - Advance simulation
  - `game_events()` - Stream of physics events

### Flutter App
- **mobile/lib/main.dart**: App entry, initializes Rust bridge
- **mobile/lib/src/game_screen.dart**: Main game UI (to be created)
- **mobile/lib/bridge/**: Auto-generated FRB bindings (DO NOT EDIT)

## Testing Strategy

### Unit Tests
- **Rust**: Test physics calculations, orb behavior, collision detection
- **Flutter**: Test UI components, state management, event handling

### Integration Tests
- Test Flutter ↔ Rust communication
- Verify event stream delivery
- Test haptic feedback triggers

### E2E Tests (Maestro)
- Tap to drop orb
- Verify score increases
- Test merge animations
- Performance benchmarks

## Common Tasks

### Adding New Orb Tier
1. Update `OrbTier` enum in `rust/native/src/orb.rs`
2. Add tier properties (radius, mass, color)
3. Update merge logic in physics system
4. Regenerate bridge: `make bridge`
5. Update Flutter UI assets

### Adding New Game Event
1. Add variant to `GameEvent` in `rust/native/src/events.rs`
2. Emit event from physics system
3. Regenerate bridge: `make bridge`
4. Handle event in Flutter UI

### Modifying Physics
1. Edit `rust/native/src/physics.rs`
2. Run tests: `make test-rust`
3. Rebuild: `make build-rust`
4. Test in Flutter: `cd mobile && flutter run`

## Performance Considerations

- **Physics Rate**: Target 60 FPS (16.6ms per frame)
- **Orb Limit**: Max 50 active orbs to maintain performance
- **Event Batching**: Send max 10 events per frame to Flutter
- **Memory**: Clean up despawned orbs immediately

## Debugging

### Rust Logs
```bash
RUST_LOG=debug cargo run
```

### Flutter Bridge Logs
```dart
// In main.dart
RustLib.init();
debugPrint("Rust initialized");
```

### Physics Debug Rendering
Enable Rapier debug rendering in development builds:
```rust
.add_plugins(RapierDebugRenderPlugin::default())
```

## iOS E2E Recipe

```bash
xcrun simctl boot "iPhone 17" || true
open -a Simulator
cd mobile
flutter run -d "iPhone 17"
```

Then in another terminal:
```bash
maestro test maestro/game_test.yaml --udid <ios-udid>
```

## Android E2E Recipe

```bash
emulator -avd <your-avd>
cd mobile
flutter run -d emulator-5554
```

Then:
```bash
maestro test maestro/game_test.yaml --udid emulator-5554
```

## Dependencies Updates

### Check for updates
```bash
cd rust && cargo outdated
cd mobile && flutter pub outdated
```

### Update strategy
- Test thoroughly after Bevy updates (breaking changes common)
- flutter_rust_bridge: Update both Rust and Dart versions together
- Rapier: Usually compatible within minor versions

## Known Issues & Limitations

1. **Web Platform**: Physics runs in browser but performance is limited
2. **Hot Reload**: Rust changes require full restart
3. **Asset Loading**: iOS requires separate asset bundle configuration
4. **Bridge Overhead**: Minimize calls across bridge (batch when possible)

## Conventions For Future Development

- Keep widget keys stable for AI-driven testing
- Use meaningful names for Bevy components and systems
- Document physics constants and formulas
- Keep bridge API minimal and efficient
- Test physics changes with Rust unit tests first

## MVP Features Checklist

- [ ] Circular gravity field implementation
- [ ] Orb spawning and physics
- [ ] Collision detection between orbs
- [ ] Merge logic (same tier → next tier)
- [ ] Score calculation and tracking
- [ ] Level progression (rotation speed increase)
- [ ] Haptic feedback on merge
- [ ] Basic UI (score, level display)
- [ ] Game over detection

## Future Enhancements (Post-MVP)

- [ ] Shop system for orb powerups
- [ ] Social sharing to TikTok/Shorts
- [ ] Particle effects system
- [ ] Sound effects and music
- [ ] Leaderboard integration
- [ ] Daily challenges
- [ ] Orb skins and themes

## References

- [Bevy Book](https://bevyengine.org/learn/book/introduction/)
- [Rapier Physics](https://rapier.rs/docs/user_guides/rust/getting_started)
- [flutter_rust_bridge Guide](https://cjycode.com/flutter_rust_bridge/)
- [Flutter Performance](https://docs.flutter.dev/perf/best-practices)

---

**Last Updated**: 2026-04-06  
**Version**: 0.1.0 (MVP - Initial Setup)
