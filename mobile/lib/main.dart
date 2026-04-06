import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'dart:async';
import 'package:alchemists_orbit/src/rust/frb_generated.dart';
import 'package:alchemists_orbit/src/rust/api/physics.dart';
import 'package:alchemists_orbit/src/rust/api/simple.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // Initialize Rust bridge
  await RustLib.init();

  runApp(const AlchemistsOrbitApp());
}

class AlchemistsOrbitApp extends StatelessWidget {
  const AlchemistsOrbitApp({super.key});

  @override
  Widget build(BuildContext context) {
    // Lock orientation to portrait
    SystemChrome.setPreferredOrientations([
      DeviceOrientation.portraitUp,
      DeviceOrientation.portraitDown,
    ]);

    return MaterialApp(
      title: 'Alchemist\'s Orbit',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: const Color(0xFF6B2E9B),
          brightness: Brightness.dark,
        ),
        useMaterial3: true,
      ),
      home: const GameScreen(),
    );
  }
}

class GameScreen extends StatefulWidget {
  const GameScreen({super.key});

  @override
  State<GameScreen> createState() => _GameScreenState();
}

class _GameScreenState extends State<GameScreen>
    with SingleTickerProviderStateMixin {
  int _selectedType = 0;
  final Map<int, (double, double)> _dimensionsByType =
      <int, (double, double)>{};
  List<DominoTransform> _dominoes = <DominoTransform>[];
  ChainStatus? _status;
  final List<String> _eventLog = <String>[];
  (double, double) _currentDimensions = (8.0, 24.0);
  String _statusText = 'Tap the field to place a domino in Rust world.';
  Timer? _loopTimer;
  bool _bridgeReady = false;

  @override
  void initState() {
    super.initState();
    _initializeBridge();
  }

  void _initializeBridge() {
    try {
      resetWorld();
      for (final type in <int>[0, 1, 2]) {
        final dimensions = getDominoDimensions(dominoType: type);
        _dimensionsByType[type] = (dimensions.$1, dimensions.$2);
      }
      _bridgeReady = true;
      _refreshDimensions();
      _pollWorld();
      _loopTimer = Timer.periodic(const Duration(milliseconds: 16), (_) {
        if (!_bridgeReady) {
          return;
        }
        step(deltaTime: 1 / 60);
        _pollWorld();
      });
    } catch (_) {
      setState(() {
        _statusText =
            'Bridge not initialized yet. Launch with `main()` to enable Rust calls.';
      });
    }
  }

  void _refreshDimensions() {
    final dimensions =
        _dimensionsByType[_selectedType] ??
        (_currentDimensions.$1, _currentDimensions.$2);
    setState(() {
      _currentDimensions = dimensions;
      _statusText =
          'Domino type $_selectedType selected (${dimensions.$1.toStringAsFixed(1)} x ${dimensions.$2.toStringAsFixed(1)}).';
    });
  }

  void _pollWorld() {
    try {
      final transforms = getDominoTransforms();
      final events = getEvents();
      final chainStatus = getChainStatus();
      setState(() {
        _dominoes = transforms;
        _status = chainStatus;
        if (events.isNotEmpty) {
          for (final event in events) {
            _eventLog.add(event.kind);
          }
          if (_eventLog.length > 10) {
            _eventLog.removeRange(0, _eventLog.length - 10);
          }

          final event = events.last;
          if (event.kind == 'ChainCompleted') {
            _statusText =
                'Chain completed in ${chainStatus.timeElapsed.toStringAsFixed(2)}s';
          } else if (event.kind == 'DominoFell') {
            _statusText =
                'Domino ${event.dominoId ?? '-'} fell at ${event.timestamp?.toStringAsFixed(2) ?? '?'}s';
          } else if (event.kind == 'ChainTriggered') {
            _statusText = 'Chain triggered';
          } else if (event.kind == 'DominoPlaced') {
            _statusText = 'Placed domino #${event.dominoId ?? '-'}';
          }
        }
      });
    } catch (_) {
      _bridgeReady = false;
    }
  }

  void _placeDomino(Offset position) {
    if (!_bridgeReady) {
      return;
    }
    try {
      placeDomino(
        x: position.dx,
        y: position.dy,
        angle: 0,
        dominoType: _selectedType,
      );
      _pollWorld();
    } catch (e) {
      setState(() {
        _statusText = 'Bridge call failed: $e';
      });
    }
    HapticFeedback.mediumImpact();
  }

  void _triggerChain() {
    if (!_bridgeReady) {
      return;
    }
    final triggered = triggerDominoPush();
    setState(() {
      _statusText = triggered
          ? 'Chain trigger requested.'
          : 'No trigger available.';
    });
    _pollWorld();
  }

  void _resetWorldState() {
    if (!_bridgeReady) {
      return;
    }
    resetWorld();
    _pollWorld();
  }

  @override
  void dispose() {
    _loopTimer?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SafeArea(
        child: Column(
          children: [
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                gradient: LinearGradient(
                  colors: [
                    Colors.purple.withValues(alpha: 0.3),
                    Colors.transparent,
                  ],
                  begin: Alignment.topCenter,
                  end: Alignment.bottomCenter,
                ),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    'FRB Bridge Smoke Test',
                    style: Theme.of(context).textTheme.titleLarge?.copyWith(
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  const SizedBox(height: 8),
                  Text(
                    _statusText,
                    style: Theme.of(context).textTheme.bodyMedium,
                  ),
                  const SizedBox(height: 8),
                  Text(
                    'Dominoes: ${_status?.dominoCount ?? 0} | Fallen: ${_status?.fallenCount ?? 0} | Time: ${(_status?.timeElapsed ?? 0).toStringAsFixed(2)}s',
                    style: Theme.of(context).textTheme.bodySmall,
                  ),
                  const SizedBox(height: 4),
                  Text(
                    'Events: ${_eventLog.isEmpty ? '-' : _eventLog.join(' > ')}',
                    style: Theme.of(context).textTheme.bodySmall,
                  ),
                  const SizedBox(height: 8),
                  Wrap(
                    spacing: 8,
                    children: [
                      ChoiceChip(
                        label: const Text('Standard'),
                        selected: _selectedType == 0,
                        onSelected: (_) {
                          _selectedType = 0;
                          _refreshDimensions();
                        },
                      ),
                      ChoiceChip(
                        label: const Text('Heavy'),
                        selected: _selectedType == 1,
                        onSelected: (_) {
                          _selectedType = 1;
                          _refreshDimensions();
                        },
                      ),
                      ChoiceChip(
                        label: const Text('Tall'),
                        selected: _selectedType == 2,
                        onSelected: (_) {
                          _selectedType = 2;
                          _refreshDimensions();
                        },
                      ),
                    ],
                  ),
                ],
              ),
            ),
            Expanded(
              child: GestureDetector(
                key: const Key('game_canvas'),
                onTapDown: (details) {
                  _placeDomino(details.localPosition);
                },
                child: Container(
                  color: Colors.black,
                  child: CustomPaint(
                    painter: GamePainter(
                      dominoes: _dominoes,
                      dimensionsByType: _dimensionsByType,
                    ),
                    child: Center(
                      child: _dominoes.isEmpty
                          ? Text(
                              'Tap anywhere to place dominoes',
                              style: Theme.of(context).textTheme.titleMedium
                                  ?.copyWith(color: Colors.white70),
                            )
                          : null,
                    ),
                  ),
                ),
              ),
            ),
            Padding(
              padding: const EdgeInsets.fromLTRB(16, 8, 16, 16),
              child: Row(
                children: [
                  Expanded(
                    child: FilledButton.tonal(
                      onPressed: _resetWorldState,
                      child: const Text('Reset World'),
                    ),
                  ),
                  const SizedBox(width: 8),
                  Expanded(
                    child: FilledButton(
                      onPressed: _triggerChain,
                      child: const Text('Trigger'),
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class GamePainter extends CustomPainter {
  final List<DominoTransform> dominoes;
  final Map<int, (double, double)> dimensionsByType;

  GamePainter({required this.dominoes, required this.dimensionsByType});

  @override
  void paint(Canvas canvas, Size size) {
    final gridPaint = Paint()
      ..color = Colors.white12
      ..strokeWidth = 1;
    for (double y = 0; y < size.height; y += 40) {
      canvas.drawLine(Offset(0, y), Offset(size.width, y), gridPaint);
    }

    final dominoPaint = Paint()
      ..color = const Color(0xFFFFA726)
      ..style = PaintingStyle.fill;
    final strokePaint = Paint()
      ..color = Colors.white70
      ..style = PaintingStyle.stroke
      ..strokeWidth = 1;

    for (final domino in dominoes) {
      final dimensions = dimensionsByType[domino.dominoType] ?? (10.0, 50.0);
      final rect = Rect.fromCenter(
        center: Offset(domino.x, domino.y),
        width: dimensions.$1,
        height: dimensions.$2,
      );
      canvas.save();
      canvas.translate(domino.x, domino.y);
      canvas.rotate(domino.angle);
      canvas.translate(-domino.x, -domino.y);
      canvas.drawRRect(
        RRect.fromRectAndRadius(rect, const Radius.circular(3)),
        dominoPaint,
      );
      canvas.drawRRect(
        RRect.fromRectAndRadius(rect, const Radius.circular(3)),
        strokePaint,
      );
      canvas.restore();
    }
  }

  @override
  bool shouldRepaint(GamePainter oldDelegate) {
    return oldDelegate.dominoes != dominoes ||
        oldDelegate.dimensionsByType != dimensionsByType;
  }
}
