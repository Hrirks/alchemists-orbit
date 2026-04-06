# Domino Chain Reaction

A physics-based domino chain reaction game with slow-motion replay, built with Flutter UI and Bevy physics engine.

## Game Concept

Place dominoes strategically in a scene, tap to trigger the first one, and watch the satisfying chain reaction unfold. Replay in slow-motion to see every detail of the cascade.

**Core Loop:**
1. Place dominoes with drag gesture
2. Tap to trigger first domino
3. Watch realistic physics chain reaction
4. Replay in 0.25x slow-mo
5. Progress through 30 levels

## Architecture

**Stack**: Flutter (UI/Overlay) + Bevy (Physics Engine) + flutter_rust_bridge (Communication)

### Components

- **Physics World (Bevy)**: Headless 2D physics simulation using Rapier2D. Handles domino rigid bodies, collision detection, and toppling mechanics.
- **UI Layer (Flutter)**: Portrait canvas for placement, HUD (level, timer), replay controls, level select screen.
- **The Bridge**: `flutter_rust_bridge` sends commands (`PlaceDominoCmd`, `TriggerChain`) from Flutter to Rust and streams events (`DominoFell`, `ChainCompleted`) from Rust to Flutter.

### Key Features

1. **Realistic Physics**: Rapier2D rigid body simulation for accurate domino toppling
2. **Replay System**: Records 60fps physics snapshots for slow-motion playback
3. **30 Levels**: Progressive difficulty with corners, mixed types, obstacles
4. **3-Star Rating**: Based on time, dominoes used, and perfect chains
5. **Haptic Feedback**: Mobile vibrations on placement, trigger, and completion

## Project Structure

```
alchemists-orbit/  # (repo name kept for continuity)
├── mobile/              # Flutter application (UI layer)
│   ├── lib/
│   │   ├── src/rust/   # Generated flutter_rust_bridge bindings
│   │   └── main.dart   # Game UI (placement, replay)
│   ├── rust/           # Bridge crate linking to game/
│   └── assets/levels/  # 30 level JSON files
├── rust/               # Rust workspace (Physics engine)
│   ├── game/           # Bevy physics + domino simulation
│   └── api/            # Bridge API definitions
├── scripts/            # Build and dev scripts
├── maestro/            # E2E test flows
└── docs/               # Architecture diagrams
```

## Prerequisites

- Flutter 3.41+
- Rust 1.75+ with cargo
- flutter_rust_bridge_codegen 2.5+
- Xcode (for iOS builds)
- Java 21+ (for Maestro E2E tests)

## Quick Start

### 1. Install Dependencies

```bash
# Install flutter_rust_bridge CLI
cargo install flutter_rust_bridge_codegen

# Install Rust iOS targets
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios

# Install Flutter dependencies
cd mobile && flutter pub get
```

### 2. Build Rust Bridge

```bash
# Generate bridge code
make bridge

# Or manually:
cd mobile && flutter_rust_bridge_codegen generate
```

### 3. Run the App

```bash
# iOS Simulator
cd mobile && flutter run -d ios

# Android Emulator
cd mobile && flutter run -d android
```

## Development Workflow

### Phase 1 — Foundation (Issues #6-#12)
Start here: **#10 Bridge API Types** — critical contract definition

```bash
# Create bridge types first
vim rust/game/src/api_types.rs
# Define PlaceDominoCmd, ChainEvent, ReplayFrame

# Generate bindings
make bridge

# Verify in Flutter
cd mobile && flutter test
```

### Phase 2 — Core Gameplay (#13-#20)
**#20 Replay Frame Buffer** is most novel — spike this early

```bash
# Test physics simulation
cargo test --manifest-path=rust/game/Cargo.toml

# Run app and test domino placement
cd mobile && flutter run -d ios
```

### Phase 3 — Visual Polish (#21-#25)
Focus on 60fps rendering and smooth animations

### Phase 4 — Level System (#26-#31)
**#26 30 Levels** can start in parallel with Phase 2

```bash
# Design levels in JSON
vim mobile/assets/levels/level_001.json

# Test level loader
cargo test level_loader
```

### Phase 5 — Polish & Release (#32-#45)
Bug bash, TestFlight, App Store submission

## Testing

### Unit Tests

```bash
# Rust physics tests
cargo test --manifest-path=rust/game/Cargo.toml

# Flutter widget tests
cd mobile && flutter test
```

### E2E Tests (Maestro)

```bash
# Run full flow: place → trigger → replay
maestro test maestro/domino_test.yaml
```

## CI/CD

GitHub Actions workflow (`.github/workflows/ci.yml`):
- Rust clippy + test
- Flutter analyze + test
- iOS build artifact
- Maestro E2E tests on simulator

## Key Technical Decisions

### Issue #10: Bridge API Types
Defines contract between Flutter and Rust. Get this right before building either side.

### Issue #20: Replay Frame Buffer
Most novel feature — records 60fps physics snapshots for slow-mo playback. Deserves spike before Phase 4.

### Issue #26: 30 Levels (JSON)
Level design can run in parallel with physics development. Define JSON schema first (#27).

## Development Commands

- `make bridge` - Generate flutter_rust_bridge code
- `make test-rust` - Run Rust tests
- `make test-flutter` - Run Flutter tests  
- `make ci` - Run all CI checks
- `make clean` - Clean build artifacts

## AI Handoff

See `MEMORY.md` for persistent context and workflows for future AI sessions.

**Current Status:**
- ✅ 40 GitHub issues created (#6-#45)
- ✅ Waterfall phase structure defined
- ⏳ Next: Implement #10 (bridge API types)

**Issue Tracker:**
https://github.com/Hrirks/alchemists-orbit/issues

## License

MIT License - See LICENSE file for details
