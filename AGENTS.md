# Agent Workflow Notes

## Phase Completion Testing Policy

After each phase is completed, run the following automated checks before marking the phase done:

1. Flutter unit and widget tests:
   - `flutter test`
2. Flutter integration tests on emulator/simulator:
   - `flutter test integration_test/simple_test.dart -d <device>`
3. Maestro mobile smoke tests:
   - `maestro test .maestro/smoke.yaml`

If any test fails, fix the issue and re-run the full phase completion test set.
