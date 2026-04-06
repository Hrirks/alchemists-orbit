#!/bin/bash
# Bulk create all 40 GitHub issues for Domino Chain Reaction game
# Run from repo root: bash scripts/create-issues.sh

set -e

echo "Creating Phase 1 — Foundation issues..."

gh issue create --title "#001 Init Bevy workspace and Cargo.toml" \
  --body "## Objective
Set up Bevy workspace structure for physics simulation.

## Tasks
- [ ] Create \`rust/game/\` crate for Bevy physics
- [ ] Add Bevy 0.14 and Rapier2D dependencies
- [ ] Configure \`crate-type = [\"lib\", \"staticlib\", \"cdylib\"]\`
- [ ] Add basic \`App::new()\` with MinimalPlugins
- [ ] Verify builds on iOS simulator target

## Acceptance
- \`cargo build\` succeeds
- Can link to mobile/rust bridge layer

## Phase
Phase 1 — Foundation" \

gh issue create --title "#002 Flutter app scaffold (portrait, tap gestures)" \
  --body "## Objective
Create Flutter UI scaffold for portrait gameplay with tap interactions.

## Tasks
- [ ] Lock orientation to portrait only
- [ ] Create \`GameScreen\` with CustomPainter canvas
- [ ] Add tap/drag gesture detection
- [ ] Design HUD layout (level number, replay button)
- [ ] Add placeholder \"Tap to place domino\" UI

## Acceptance
- App runs in portrait
- Tap positions logged to console
- HUD renders correctly

## Phase
Phase 1 — Foundation" \

gh issue create --title "#003 flutter_rust_bridge config and codegen" \
  --body "## Objective
Set up flutter_rust_bridge for Rust ↔ Flutter communication.

## Tasks
- [ ] Create \`flutter_rust_bridge.yaml\` config
- [ ] Add \`mobile/rust/\` bridge crate
- [ ] Link to \`rust/game/\` crate as dependency
- [ ] Run \`flutter_rust_bridge_codegen generate\`
- [ ] Verify Dart bindings generated in \`lib/src/rust/\`

## Acceptance
- Bridge generates without errors
- Can call simple Rust function from Flutter
- Hot reload works

## Phase
Phase 1 — Foundation" \

gh issue create --title "#004 Minimal physics loop (Bevy + Rapier headless)" \
  --body "## Objective
Create headless Bevy physics loop callable from Flutter.

## Tasks
- [ ] Add RapierPhysicsPlugin to Bevy App
- [ ] Create \`PhysicsWorld\` struct with \`step(dt)\` method
- [ ] Add simple ground plane for testing
- [ ] Spawn test rigid body and verify gravity works
- [ ] Expose \`step_physics(delta_time)\` via bridge

## Acceptance
- Physics updates on every Flutter frame
- Test body falls due to gravity
- No panics or crashes

## Phase
Phase 1 — Foundation" \

gh issue create --title "#005 Bridge API types (PlaceDominoCmd, ChainEvent, ReplayFrame)" \
  --body "## Objective
**CRITICAL EARLY DECISION** — Define bridge API contract between Rust and Flutter.

## API Types Needed

### Commands (Flutter → Rust)
\`\`\`rust
pub struct PlaceDominoCmd {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub domino_type: DominoType,
}

pub enum DominoType {
    Standard,
    Heavy,
    Tall,
}
\`\`\`

### Events (Rust → Flutter)
\`\`\`rust
pub enum ChainEvent {
    DominoPlaced { id: u32, x: f32, y: f32 },
    DominoFell { id: u32, timestamp: f32 },
    ChainCompleted { total_dominoes: u32, time: f32 },
    LevelFailed,
}
\`\`\`

### Replay Data
\`\`\`rust
pub struct ReplayFrame {
    pub timestamp: f32,
    pub domino_states: Vec<DominoState>,
}

pub struct DominoState {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub is_fallen: bool,
}
\`\`\`

## Tasks
- [ ] Create \`rust/game/src/api_types.rs\`
- [ ] Define all structs with \`#[frb(...)]\` annotations
- [ ] Add Serde derives for JSON serialization
- [ ] Document field meanings and units
- [ ] Generate Dart types and verify in Flutter

## Acceptance
- All types compile in Rust
- Dart bindings generated correctly
- Types are immutable and Copy where possible

## Phase
Phase 1 — Foundation

## ⚠️ Important
This issue blocks #008–#015. Get this right before building either side." \

gh issue create --title "#006 Initial CI/CD (lint, test, build iOS)" \
  --body "## Objective
Set up GitHub Actions for automated testing.

## Tasks
- [ ] Add \`.github/workflows/ci.yml\`
- [ ] Run \`cargo clippy\` on Rust code
- [ ] Run \`cargo test\` for Rust unit tests
- [ ] Run \`flutter analyze\`
- [ ] Build iOS .ipa artifact
- [ ] Cache Cargo and Flutter dependencies

## Acceptance
- CI passes on every PR
- Build artifacts uploaded
- Runs in < 10 minutes

## Phase
Phase 1 — Foundation" \

gh issue create --title "#007 Project docs (README, MEMORY.md, architecture diagram)" \
  --body "## Objective
Document project architecture and handoff instructions.

## Tasks
- [ ] Update README.md with game description
- [ ] Create architecture diagram (Flutter ↔ Bridge ↔ Bevy)
- [ ] Update MEMORY.md with workflows
- [ ] Document build commands in Makefile
- [ ] Add troubleshooting section

## Acceptance
- New developers can clone and run the app
- Architecture is clear from docs
- AI agents can understand context from MEMORY.md

## Phase
Phase 1 — Foundation" \

echo ""
echo "Creating Phase 2 — Core Gameplay issues..."

gh issue create --title "#008 Domino entity (RigidBody + Collider + component)" \
  --body "## Objective
Create Bevy entity bundle for domino physics.

## Tasks
- [ ] Define \`DominoBundle\` with RigidBody::Dynamic
- [ ] Add BoxCollider (10x50 pixels, 5:1 aspect ratio)
- [ ] Set mass, friction, restitution for realistic toppling
- [ ] Add \`Domino\` component with \`id\`, \`domino_type\`
- [ ] Test spawning domino and verify it stands upright

## Acceptance
- Domino stands stable until pushed
- Falls realistically when tipped
- Collider matches visual bounds

## Dependencies
Requires #004, #005

## Phase
Phase 2 — Core Gameplay" \

gh issue create --title "#009 Place domino command handler (spawn from Flutter)" \
  --body "## Objective
Handle domino placement commands from Flutter.

## Tasks
- [ ] Implement \`place_domino(cmd: PlaceDominoCmd)\` in bridge API
- [ ] Queue commands via mpsc channel (thread-safe)
- [ ] Process commands in Bevy's Update system
- [ ] Spawn domino at (x, y) with angle rotation
- [ ] Send \`ChainEvent::DominoPlaced\` back to Flutter

## Acceptance
- Tapping in Flutter spawns domino in Rust
- Domino appears at correct position/angle
- Event received in Flutter logs

## Dependencies
Requires #005, #008

## Phase
Phase 2 — Core Gameplay" \

gh issue create --title "#010 Tap-to-trigger system (apply impulse to first domino)" \
  --body "## Objective
Allow player to trigger chain reaction by tapping first domino.

## Tasks
- [ ] Detect tap on specific domino (raycasting or closest)
- [ ] Apply impulse to domino rigid body
- [ ] Send \`trigger_chain(domino_id)\` command to Rust
- [ ] Mark domino as \"triggered\" to prevent re-trigger
- [ ] Visual feedback (highlight on tap)

## Acceptance
- Tap triggers the correct domino
- Domino tips over from impulse
- Can only trigger once per level

## Dependencies
Requires #009

## Phase
Phase 2 — Core Gameplay" \

gh issue create --title "#011 Chain reaction detection (collision → topple → next)" \
  --body "## Objective
Detect when dominoes knock each other over in sequence.

## Tasks
- [ ] Listen to Rapier CollisionEvent::Started
- [ ] Check if collision is between two dominoes
- [ ] Determine if impact force exceeds topple threshold
- [ ] Send \`ChainEvent::DominoFell\` with timestamp
- [ ] Track chain propagation (domino A → B → C)

## Acceptance
- One domino knocking another triggers event
- Chain propagates through 3+ dominoes
- Events sent in correct order

## Dependencies
Requires #008, #010

## Phase
Phase 2 — Core Gameplay" \

gh issue create --title "#012 Win condition (all dominoes fell)" \
  --body "## Objective
Detect level completion when all dominoes have fallen.

## Tasks
- [ ] Track \`total_dominoes\` and \`fallen_count\`
- [ ] Check if all dominoes reached \"fallen\" state
- [ ] Send \`ChainEvent::ChainCompleted\` event
- [ ] Record completion time
- [ ] Prevent further interactions after win

## Acceptance
- Level completes when all dominoes fall
- Displays win screen in Flutter
- Can replay or advance to next level

## Dependencies
Requires #011

## Phase
Phase 2 — Core Gameplay" \

gh issue create --title "#013 Fail condition (timer, domino limit, or stuck chain)" \
  --body "## Objective
Detect level failure states.

## Tasks
- [ ] Implement level timer (30s default)
- [ ] Track placed domino count vs. max limit
- [ ] Detect \"stuck\" chain (no dominoes fell in 5s)
- [ ] Send \`ChainEvent::LevelFailed\` event
- [ ] Show retry UI in Flutter

## Acceptance
- Timeout triggers failure
- Exceeding domino limit fails level
- Stuck chain detected correctly

## Dependencies
Requires #012

## Phase
Phase 2 — Core Gameplay" \

gh issue create --title "#014 Reset level (clear all entities, reload scene)" \
  --body "## Objective
Allow player to reset level to initial state.

## Tasks
- [ ] Implement \`reset_level()\` command
- [ ] Despawn all domino entities
- [ ] Reset physics simulation state
- [ ] Reload level from JSON definition
- [ ] Send reset confirmation to Flutter

## Acceptance
- Tap \"Reset\" clears all dominoes
- Scene reloads to starting state
- No leftover entities or physics bugs

## Dependencies
Requires #022 (level loader)

## Phase
Phase 2 — Core Gameplay" \

gh issue create --title "#015 Replay frame buffer (60fps physics snapshots)" \
  --body "## Objective
**MOST TECHNICALLY NOVEL** — Record all rigid body states at 60fps for slow-mo replay.

## Architecture
\`\`\`
Bevy Update System (60fps)
  ↓
Capture all DominoState structs
  ↓
Store in Vec<ReplayFrame> (ring buffer, max 10s = 600 frames)
  ↓
Send buffer to Flutter on ChainCompleted
  ↓
Flutter plays back at 0.25x speed
\`\`\`

## Tasks
- [ ] Create \`ReplayRecorder\` Bevy resource
- [ ] Every frame: snapshot all \`(id, Transform, fallen)\` states
- [ ] Store in ring buffer (max 600 frames)
- [ ] Expose \`get_replay_buffer() -> Vec<ReplayFrame>\` via bridge
- [ ] Handle memory limits (compress or downsample if needed)

## Acceptance
- Full 10s game recorded at 60fps
- Flutter can fetch replay buffer
- No significant FPS drop during recording

## Dependencies
Requires #005 (ReplayFrame type)

## Phase
Phase 2 — Core Gameplay

## ⚠️ Spike Recommended
This is the most novel feature. Prototype replay recording in Bevy first before building Flutter playback." \

echo ""
echo "Creating Phase 3 — Visual Polish issues..."

gh issue create --title "#016 Render dominoes in Flutter (position sync from Rust)" \
  --body "## Objective
Render dominoes in Flutter based on Rust physics state.

## Tasks
- [ ] Add \`get_domino_states() -> Vec<DominoState>\` API
- [ ] Call every frame in Flutter (60fps)
- [ ] Draw dominoes in CustomPainter at (x, y, angle)
- [ ] Use different colors for domino types
- [ ] Show visual distinction between standing/fallen

## Acceptance
- Dominoes render at correct positions
- Rotation matches Rust physics
- No visual lag or jitter

## Dependencies
Requires #008, #009

## Phase
Phase 3 — Visual Polish" \

gh issue create --title "#017 Domino placement preview (ghost domino on drag)" \
  --body "## Objective
Show preview of domino before placement.

## Tasks
- [ ] On drag gesture, show semi-transparent domino
- [ ] Rotate preview based on drag direction
- [ ] Snap to grid (optional, 10px increments)
- [ ] Validate placement (no overlap with existing)
- [ ] Confirm placement on release

## Acceptance
- Ghost domino follows finger
- Rotation feels natural
- Invalid positions highlighted

## Dependencies
Requires #016

## Phase
Phase 3 — Visual Polish" \

gh issue create --title "#018 Falling animation polish (rotation smoothing)" \
  --body "## Objective
Make domino falling animations feel smooth and satisfying.

## Tasks
- [ ] Ensure 60fps rendering from Rust updates
- [ ] Add rotation interpolation if needed
- [ ] Test on low-end devices (iPhone 8)
- [ ] Add subtle particle effect on impact (optional)

## Acceptance
- Dominoes fall smoothly without stutter
- Maintains 60fps on target hardware

## Dependencies
Requires #016

## Phase
Phase 3 — Visual Polish" \

gh issue create --title "#019 Sound effects (place, tap, fall, chain complete)" \
  --body "## Objective
Add audio feedback for key interactions.

## Tasks
- [ ] Add sound assets (place.mp3, tap.mp3, fall.mp3, win.mp3)
- [ ] Play sound on \`DominoPlaced\` event
- [ ] Play sound on \`DominoFell\` event
- [ ] Cascade fall sounds (avoid audio spam)
- [ ] Play victory fanfare on \`ChainCompleted\`

## Acceptance
- Sounds play at correct times
- No audio glitches or overlaps
- Volume feels balanced

## Phase
Phase 3 — Visual Polish" \

gh issue create --title "#020 Haptic feedback (tap, trigger, win)" \
  --body "## Objective
Add haptic feedback for tactile experience.

## Tasks
- [ ] Add \`vibration\` package
- [ ] Light haptic on domino placement
- [ ] Medium haptic on trigger tap
- [ ] Success pattern on level complete
- [ ] Allow toggle in settings

## Acceptance
- Haptics feel responsive
- Can be disabled in settings
- Works on iOS and Android

## Phase
Phase 3 — Visual Polish" \

echo ""
echo "Creating Phase 4 — Level System issues..."

gh issue create --title "#021 Design 30 levels (JSON scene format)" \
  --body "## Objective
**PARALLEL TRACK** — Design 30 levels while engineers build loader.

## Level Progression
- Levels 1-5: Basic chains (3-5 dominoes)
- Levels 6-10: Corners and curves
- Levels 11-15: Mixed domino types
- Levels 16-20: Obstacles (walls, gaps)
- Levels 21-25: Timing challenges
- Levels 26-30: Complex multi-path chains

## JSON Format (define in #022)
\`\`\`json
{
  \"level_id\": 1,
  \"max_dominoes\": 10,
  \"time_limit\": 30,
  \"obstacles\": [],
  \"starting_dominoes\": [
    {\"x\": 100, \"y\": 200, \"type\": \"Standard\", \"locked\": true}
  ]
}
\`\`\`

## Tasks
- [ ] Create \`assets/levels/level_001.json\` through \`level_030.json\`
- [ ] Playtest each level for difficulty curve
- [ ] Balance domino limits and time limits
- [ ] Add optional 3-star rating thresholds

## Acceptance
- 30 levels designed and saved as JSON
- Difficulty increases gradually
- No impossible or trivial levels

## Dependencies
Requires #022 (scene format definition)

## Phase
Phase 4 — Level System

## ⚠️ Can Start Early
Level design can begin in week 2, parallel to physics work." \

gh issue create --title "#022 Level loader (JSON → Bevy scene)" \
  --body "## Objective
Load level definitions from JSON into Bevy physics world.

## Tasks
- [ ] Define JSON schema for levels
- [ ] Create \`LevelDefinition\` struct
- [ ] Implement \`load_level(level_id) -> Result<LevelDefinition>\`
- [ ] Spawn obstacles as static RigidBodies
- [ ] Spawn locked starting dominoes
- [ ] Set level constraints (max dominoes, timer)

## Acceptance
- Can load any of 30 levels from JSON
- Scene spawns correctly in Bevy
- Errors handled gracefully (missing files)

## Dependencies
Requires #008, #021 (for schema)

## Phase
Phase 4 — Level System" \

gh issue create --title "#023 Level progression (unlock next, save progress)" \
  --body "## Objective
Track player progress through 30 levels.

## Tasks
- [ ] Store progress in local storage (SharedPreferences)
- [ ] Unlock next level on completion
- [ ] Save 3-star ratings per level
- [ ] Allow replaying completed levels
- [ ] Show level select screen with lock states

## Acceptance
- Progress persists across app restarts
- Can replay any unlocked level
- Lock icons shown for locked levels

## Dependencies
Requires #012 (win condition), #022 (level system)

## Phase
Phase 4 — Level System" \

gh issue create --title "#024 3-star rating (time, dominoes used, perfect chain)" \
  --body "## Objective
Rate player performance with 1-3 stars.

## Criteria
- ⭐ Complete the level
- ⭐⭐ Complete under target time OR use ≤ optimal dominoes
- ⭐⭐⭐ Perfect chain (no failed dominoes) + time/domino bonus

## Tasks
- [ ] Calculate stars based on level metrics
- [ ] Store best star rating per level
- [ ] Display stars on level complete screen
- [ ] Show star count on level select

## Acceptance
- Star calculation feels fair
- Players understand criteria
- Stars persist in save data

## Dependencies
Requires #012, #023

## Phase
Phase 4 — Level System" \

gh issue create --title "#025 Replay playback UI (slow-mo, scrubber)" \
  --body "## Objective
Allow player to watch replay in slow motion with scrubber.

## Tasks
- [ ] Add \"Watch Replay\" button on level complete
- [ ] Fetch replay buffer from Rust via bridge
- [ ] Render dominoes from buffered states (no Rust physics)
- [ ] Add playback speed controls (0.25x, 0.5x, 1x, 2x)
- [ ] Add timeline scrubber to jump to specific frames
- [ ] Add pause/resume controls

## Acceptance
- Replay plays smoothly at 0.25x speed
- Scrubber works without lag
- Can pause and resume

## Dependencies
Requires #015 (replay buffer), #016 (rendering)

## Phase
Phase 4 — Level System" \

gh issue create --title "#026 Tutorial level (onboarding, tap hints)" \
  --body "## Objective
Teach new players the core mechanics.

## Tasks
- [ ] Create \`level_000.json\` as tutorial
- [ ] Add overlay hints (\"Place domino here\")
- [ ] Show tap animation on trigger domino
- [ ] Force completion before unlocking level 1
- [ ] Skip button for returning players

## Acceptance
- New players understand mechanics
- Tutorial feels quick (< 30s)
- Can be skipped

## Dependencies
Requires #022, #023

## Phase
Phase 4 — Level System" \

echo ""
echo "Creating Phase 5 — Polish & Release issues..."

gh issue create --title "#027 Settings screen (sound, haptics, reset progress)" \
  --body "## Objective
Add app settings for user preferences.

## Tasks
- [ ] Create Settings screen
- [ ] Add sound volume slider
- [ ] Add haptics toggle
- [ ] Add \"Reset All Progress\" button with confirmation
- [ ] Persist settings in local storage

## Acceptance
- Settings save correctly
- Changes apply immediately
- Reset clears all progress

## Phase
Phase 5 — Polish & Release" \

gh issue create --title "#028 Level select screen (grid, stars, locks)" \
  --body "## Objective
Create polished level selection UI.

## Tasks
- [ ] Grid layout (5 columns × 6 rows)
- [ ] Show level number, star count, lock state
- [ ] Tap to load level
- [ ] Scroll smoothly through all 30 levels
- [ ] Highlight current level

## Acceptance
- Grid feels responsive
- Visual hierarchy is clear
- Easy to navigate

## Phase
Phase 5 — Polish & Release" \

gh issue create --title "#029 App icon and splash screen" \
  --body "## Objective
Add branded app icon and splash screen.

## Tasks
- [ ] Design app icon (1024×1024)
- [ ] Generate all iOS sizes via \`flutter_launcher_icons\`
- [ ] Create splash screen animation
- [ ] Add launch screen for iOS

## Acceptance
- Icon looks good on home screen
- Splash screen shows on launch
- No white flash on startup

## Phase
Phase 5 — Polish & Release" \

gh issue create --title "#030 Performance profiling (60fps, memory, battery)" \
  --body "## Objective
Ensure app runs smoothly on target devices.

## Tasks
- [ ] Profile with Flutter DevTools
- [ ] Check physics overhead (Bevy step time)
- [ ] Monitor memory during 30-level playthrough
- [ ] Test on iPhone 8 (oldest target)
- [ ] Optimize hot paths if needed

## Acceptance
- Maintains 60fps during gameplay
- Memory stays under 150MB
- No excessive battery drain

## Phase
Phase 5 — Polish & Release" \

gh issue create --title "#031 End-to-end Maestro tests (3 levels, replay)" \
  --body "## Objective
Add automated UI tests with Maestro.

## Tasks
- [ ] Write test: Complete level 1
- [ ] Write test: Fail level and retry
- [ ] Write test: Watch replay
- [ ] Run tests in CI
- [ ] Add screenshot assertions

## Acceptance
- All Maestro tests pass
- Tests run in CI pipeline
- Catch regressions early

## Phase
Phase 5 — Polish & Release" \

gh issue create --title "#032 Bug bash and polish pass" \
  --body "## Objective
Fix all known bugs and polish rough edges.

## Areas to Check
- [ ] Edge cases (0 dominoes, timeout at 0s)
- [ ] UI glitches (overflow, alignment)
- [ ] Physics quirks (dominoes stuck, tunneling)
- [ ] Audio issues (stuttering, volume)
- [ ] Localization prep (hardcoded strings)

## Acceptance
- No critical bugs remain
- App feels polished
- Ready for TestFlight

## Phase
Phase 5 — Polish & Release" \

gh issue create --title "#033 Privacy policy and App Store assets" \
  --body "## Objective
Prepare for App Store submission.

## Tasks
- [ ] Write privacy policy (data collection: none)
- [ ] Host privacy policy page
- [ ] Create App Store screenshots (6.5\" and 5.5\")
- [ ] Write app description
- [ ] Add keywords for ASO

## Acceptance
- Privacy policy live and linked
- Screenshots look professional
- Description is compelling

## Phase
Phase 5 — Polish & Release" \

gh issue create --title "#034 TestFlight beta (internal testing)" \
  --body "## Objective
Deploy to TestFlight for internal testing.

## Tasks
- [ ] Create App Store Connect record
- [ ] Upload build via Xcode or Fastlane
- [ ] Add internal testers (5 people)
- [ ] Collect feedback
- [ ] Fix critical issues

## Acceptance
- Build installs on TestFlight
- No crashes in testing
- Feedback incorporated

## Phase
Phase 5 — Polish & Release" \

gh issue create --title "#035 App Store submission (1.0 release)" \
  --body "## Objective
Submit v1.0 to App Store for review.

## Tasks
- [ ] Final QA pass
- [ ] Bump version to 1.0.0
- [ ] Submit for App Review
- [ ] Respond to any rejections
- [ ] Release when approved

## Acceptance
- App approved by Apple
- Live on App Store
- Downloadable by public

## Phase
Phase 5 — Polish & Release" \

gh issue create --title "#036 Post-launch analytics (optional)" \
  --body "## Objective
Add basic analytics to track player behavior.

## Tasks
- [ ] Add Firebase Analytics or similar
- [ ] Track level completions
- [ ] Track average stars per level
- [ ] Track drop-off points
- [ ] Respect privacy (no PII)

## Acceptance
- Analytics working
- Privacy policy updated
- No performance impact

## Phase
Phase 5 — Polish & Release" \

gh issue create --title "#037 Leaderboard system (optional)" \
  --body "## Objective
Add global leaderboard for speed runs.

## Tasks
- [ ] Integrate backend (Firebase or custom)
- [ ] Submit best times per level
- [ ] Display top 10 times
- [ ] Show player rank
- [ ] Prevent cheating (server validation)

## Acceptance
- Leaderboard updates in real-time
- Times are accurate
- No obvious exploits

## Phase
Phase 5 — Polish & Release (Optional)" \

gh issue create --title "#038 Daily challenge (optional)" \
  --body "## Objective
Add daily procedural level for retention.

## Tasks
- [ ] Generate procedural levels from seed
- [ ] Sync seed daily (server or deterministic date)
- [ ] Track daily completion
- [ ] Award bonus stars for daily
- [ ] Show streak counter

## Acceptance
- New challenge every 24h
- All players see same level
- Streaks tracked correctly

## Phase
Phase 5 — Polish & Release (Optional)" \

gh issue create --title "#039 Localization (5 languages)" \
  --body "## Objective
Support English, Spanish, French, German, Japanese.

## Tasks
- [ ] Extract all strings to \`intl\` files
- [ ] Translate to 5 languages
- [ ] Test RTL layout (if adding Arabic)
- [ ] Verify in-app language picker
- [ ] Update App Store listings

## Acceptance
- All languages render correctly
- No hardcoded strings
- Descriptions translated

## Phase
Phase 5 — Polish & Release (Optional)" \

gh issue create --title "#040 Android port (Flutter + Rust)" \
  --body "## Objective
Port to Android (stretch goal).

## Tasks
- [ ] Build Rust for Android targets (arm64-v8a, armeabi-v7a)
- [ ] Configure Gradle for native libs
- [ ] Test on Android emulator
- [ ] Fix platform-specific bugs
- [ ] Submit to Play Store

## Acceptance
- App runs on Android
- No critical platform bugs
- Published on Play Store

## Phase
Phase 5 — Polish & Release (Optional)" \

echo ""
echo "✅ All 40 issues created successfully!"
echo ""
echo "Next steps:"
echo "1. Create milestones: gh milestone create 'Phase 1' --description '...'"
echo "2. Assign issues to milestones"
echo "3. Start with #005 (bridge API types) — critical foundation"
