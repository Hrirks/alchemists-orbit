# Alchemist's Orbit (MVP)

A physics-based merging game with rotating gravity, built with Flutter UI and Bevy physics engine.

## Architecture

**Core Stack**: Flutter (UI/Overlay) + Bevy (Physics Engine) + flutter_rust_bridge (Data Bridge)

### Components

- **The World (Bevy)**: 2D physics simulation using Rapier physics. Handles rotating gravity field, orb collisions, and particle explosions.
- **The Interface (Flutter)**: HUD (Score, Level), shop for orb upgrades, and social share API for TikTok/Shorts.
- **The Bridge**: `flutter_rust_bridge` sends "Drop Orb" commands from Dart to Rust and sends "Collision Events/Score" from Rust to Dart.

### MVP Features

1. **Circular Physics**: A central "Gravity Well" that attracts all orbs
2. **Merging Logic**: When two orbs of the same type touch, they vanish and spawn a higher-tier orb
3. **Haptic Feedback**: Flutter triggers mobile vibrations whenever a merge occurs in Bevy
4. **Infinite Progression**: Rotation speed increases over time

## Project Structure

```
alchemists-orbit/
├── mobile/              # Flutter application (UI layer)
│   ├── lib/            # Dart source code
│   ├── android/        # Android platform
│   ├── ios/            # iOS platform
│   └── pubspec.yaml    # Flutter dependencies
├── rust/               # Rust workspace (Physics engine)
│   ├── api/            # flutter_rust_bridge API definitions
│   └── native/         # Bevy physics engine implementation
├── scripts/            # Development scripts
├── maestro/            # E2E test flows
└── docker-compose.yml  # Container orchestration (optional)
```

## Prerequisites

- Flutter 3.41+
- Rust 1.75+ with cargo
- flutter_rust_bridge_codegen
- Android NDK (for Android builds)
- Xcode (for iOS builds)
- Java 21+ (required by Maestro CLI)
- Maestro CLI (optional for E2E testing)

## Getting Started

### 1. Install Dependencies

```bash
# Install flutter_rust_bridge CLI
cargo install flutter_rust_bridge_codegen

# Install Rust targets for mobile
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

# Install Flutter dependencies
cd mobile && flutter pub get
```

### 2. Build Rust Bridge

```bash
# Generate bridge code
make bridge

# Or manually:
cd rust/api && flutter_rust_bridge_codegen generate
```

### 3. Run the App

```bash
# iOS
cd mobile && flutter run -d ios

# Android
cd mobile && flutter run -d android

# Desktop (for testing)
cd mobile && flutter run -d macos
```

## Development Commands

- `make bridge` - Generate flutter_rust_bridge code
- `make test-rust` - Run Rust tests
- `make test-flutter` - Run Flutter tests
- `make ci` - Run all CI checks
- `make clean` - Clean build artifacts

## Testing

### Unit Tests

```bash
# Rust tests
cargo test --manifest-path=rust/native/Cargo.toml

# Flutter tests
cd mobile && flutter test
```

### E2E Tests

```bash
# Run Maestro flow
maestro test maestro/game_test.yaml
```

## Architecture Details

### Communication Flow

```
User Input (Tap) → Flutter Widget → Rust API (drop_orb) 
                                        ↓
                              Bevy Physics System
                                        ↓
                         Collision Detection & Merging
                                        ↓
                    Rust Event Stream → Flutter (score_updates)
                                        ↓
                              UI Update + Haptics
```

### Physics Engine (Bevy + Rapier)

- **Gravity Well**: Circular force field that attracts orbs
- **Orb Types**: 7 tiers with increasing size and mass
- **Collision System**: Rapier's contact detection for merge events
- **Particle Effects**: Spawned on merge events

### State Management

- **Flutter**: Uses streams to receive physics events from Rust
- **Rust**: Bevy's ECS manages game state and physics
- **Bridge**: Asynchronous message passing between layers

## CI/CD

GitHub Actions workflow runs:
- Rust tests and formatting checks
- Flutter analyze + test
- Multi-platform build validation (iOS, Android, Web)

## AI Handoff

See `MEMORY.md` for persistent context and standard execution recipes for future AI sessions.

## License

MIT License - See LICENSE file for details
