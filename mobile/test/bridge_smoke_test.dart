import 'dart:io';

import 'package:alchemists_orbit/src/rust/api/simple.dart';
import 'package:alchemists_orbit/src/rust/frb_generated.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

const _localRustLibPath =
    'rust/target/debug/librust_lib_alchemists_orbit.dylib';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  final hasLocalRustLibrary = File(_localRustLibPath).existsSync();

  setUpAll(() async {
    if (hasLocalRustLibrary) {
      await RustLib.init(
        externalLibrary: ExternalLibrary.open(_localRustLibPath),
      );
    }
  });

  test('bridge returns valid domino dimensions', () {
    for (final dominoType in <int>[0, 1, 2]) {
      final dimensions = getDominoDimensions(dominoType: dominoType);
      expect(dimensions.$1, greaterThan(0));
      expect(dimensions.$2, greaterThan(0));
    }
  }, skip: !hasLocalRustLibrary);

  test(
    'bridge creates place command for each domino type',
    () {
      for (final dominoType in <int>[0, 1, 2]) {
        final command = createPlaceDominoCmd(
          x: 120,
          y: 260,
          angle: 0,
          dominoType: dominoType,
        );

        expect(command, isNotNull);
      }
    },
    skip: !hasLocalRustLibrary,
  );

  test('bridge supports repeated command creation', () {
    for (var i = 0; i < 50; i++) {
      final command = createPlaceDominoCmd(
        x: i.toDouble(),
        y: (i * 2).toDouble(),
        angle: 0.1 * i,
        dominoType: i % 3,
      );

      expect(command, isNotNull);
    }
  }, skip: !hasLocalRustLibrary);
}
