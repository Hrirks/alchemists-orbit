import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:alchemists_orbit/main.dart';
import 'package:alchemists_orbit/src/rust/api/physics.dart';
import 'package:alchemists_orbit/src/rust/frb_generated.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  setUpAll(() async {
    await RustLib.init();
  });

  setUp(() {
    setDeterministicTestMode(enabled: true);
    resetWorld();
  });

  tearDown(() {
    setDeterministicTestMode(enabled: false);
    resetWorld();
  });

  testWidgets('Tap canvas updates bridge status', (WidgetTester tester) async {
    await tester.pumpWidget(const AlchemistsOrbitApp());
    await tester.pumpAndSettle();

    expect(find.text('FRB Bridge Smoke Test'), findsOneWidget);
    expect(find.textContaining('Domino type 0 selected'), findsOneWidget);

    await tester.tap(find.byKey(const Key('game_canvas')));
    await tester.pumpAndSettle();

    expect(find.textContaining('Placed domino #'), findsOneWidget);

    await tester.tap(find.byKey(const Key('game_canvas')));
    await tester.pumpAndSettle();

    await tester.tap(find.text('Trigger'));
    await tester.pumpAndSettle();

    await tester.pump(const Duration(seconds: 2));
    await tester.pumpAndSettle();

    expect(find.textContaining('Events:'), findsOneWidget);
    expect(find.textContaining('ChainTriggered'), findsOneWidget);
    expect(find.textContaining('DominoFell'), findsOneWidget);
    expect(find.textContaining('ChainCompleted'), findsOneWidget);

    await tester.tap(find.text('Reset World'));
    await tester.pumpAndSettle();

    expect(find.textContaining('Dominoes: 0 | Fallen: 0'), findsOneWidget);
  });
}
