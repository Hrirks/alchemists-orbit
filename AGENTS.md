# Agent Workflow Notes

## Phase Completion Testing Policy

After each phase is completed, run the following automated checks before marking the phase done:

1. Rust core game tests:
   - `cargo test -p domino-game` (from `rust/`)
2. Rust mobile bridge crate build:
   - `cargo build` (from `mobile/rust/`)
3. Flutter static checks:
   - `flutter analyze` (from `mobile/`)
4. Flutter unit and widget tests:
   - `flutter test` (from `mobile/`)
5. Flutter integration tests on emulator/simulator:
   - `flutter test integration_test/simple_test.dart -d <device>` (from `mobile/`)
6. Maestro mobile smoke tests:
   - `PATH="/opt/homebrew/opt/openjdk/bin:$PATH" maestro test --udid <device-udid> --no-reinstall-driver maestro/game_test.yaml`

## Test Matrix Notes

- `flutter analyze`: catches Dart/Flutter static issues before runtime.
- `flutter test`: runs widget and bridge smoke tests quickly.
- `integration_test`: validates FRB bridge and UI interactions on real simulator/emulator runtime.
- `maestro`: black-box mobile smoke coverage for critical UI flow.
- Deterministic physics mode is test-only and should be enabled from test setup, never from gameplay code.

## Environment Notes

- Maestro requires Java (OpenJDK) available in PATH.
- iOS Maestro runs may require launching the app once via `flutter run` if app launch attach is flaky.

If any test fails, fix the issue and re-run the full phase completion test set.
