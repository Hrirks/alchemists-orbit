import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'bridge/frb_generated.dart';

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
  int _score = 0;
  int _level = 1;
  bool _gameStarted = false;
  late AnimationController _animationController;

  @override
  void initState() {
    super.initState();

    // Initialize animation controller for physics updates
    _animationController = AnimationController(
      vsync: this,
      duration: const Duration(hours: 1), // Infinite loop
    );

    _animationController.addListener(_updatePhysics);
  }

  void _updatePhysics() {
    // This will be called every frame
    // TODO: Call Rust physics step
  }

  void _startGame() {
    setState(() {
      _gameStarted = true;
      _score = 0;
      _level = 1;
    });

    // Start physics update loop
    _animationController.repeat();
  }

  void _dropOrb(Offset position) {
    // TODO: Call Rust API to drop orb
    // api.dropOrb(position.dx, position.dy, 1);

    if (!_gameStarted) {
      _startGame();
    }

    // Trigger haptic feedback
    HapticFeedback.mediumImpact();
  }

  @override
  void dispose() {
    _animationController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SafeArea(
        child: Column(
          children: [
            // HUD
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
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        'Score',
                        style: Theme.of(context).textTheme.bodySmall,
                      ),
                      Text(
                        '$_score',
                        style: Theme.of(context).textTheme.headlineMedium
                            ?.copyWith(fontWeight: FontWeight.bold),
                      ),
                    ],
                  ),
                  Column(
                    crossAxisAlignment: CrossAxisAlignment.end,
                    children: [
                      Text(
                        'Level',
                        style: Theme.of(context).textTheme.bodySmall,
                      ),
                      Text(
                        '$_level',
                        style: Theme.of(context).textTheme.headlineMedium
                            ?.copyWith(fontWeight: FontWeight.bold),
                      ),
                    ],
                  ),
                ],
              ),
            ),

            // Game Area
            Expanded(
              child: GestureDetector(
                onTapDown: (details) {
                  _dropOrb(details.localPosition);
                },
                child: Container(
                  color: Colors.black,
                  child: CustomPaint(
                    painter: GamePainter(gameStarted: _gameStarted),
                    child: Center(
                      child: !_gameStarted
                          ? Column(
                              mainAxisAlignment: MainAxisAlignment.center,
                              children: [
                                Icon(
                                  Icons.bubble_chart,
                                  size: 64,
                                  color: Colors.purple.withValues(alpha: 0.5),
                                ),
                                const SizedBox(height: 16),
                                Text(
                                  'Tap to drop orbs',
                                  style: Theme.of(context)
                                      .textTheme
                                      .headlineSmall
                                      ?.copyWith(color: Colors.white70),
                                ),
                                const SizedBox(height: 8),
                                Text(
                                  'Match orbs to merge and score!',
                                  style: Theme.of(context).textTheme.bodyMedium
                                      ?.copyWith(color: Colors.white54),
                                ),
                              ],
                            )
                          : null,
                    ),
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class GamePainter extends CustomPainter {
  final bool gameStarted;

  GamePainter({required this.gameStarted});

  @override
  void paint(Canvas canvas, Size size) {
    if (!gameStarted) {
      // Draw gravity well indicator
      final center = Offset(size.width / 2, size.height / 2);
      final paint = Paint()
        ..color = Colors.purple.withValues(alpha: 0.2)
        ..style = PaintingStyle.stroke
        ..strokeWidth = 2;

      // Draw concentric circles
      for (int i = 1; i <= 3; i++) {
        canvas.drawCircle(center, i * 50.0, paint);
      }

      // Draw center dot
      canvas.drawCircle(
        center,
        8,
        Paint()
          ..color = Colors.purple.withValues(alpha: 0.5)
          ..style = PaintingStyle.fill,
      );
    }

    // TODO: Draw orbs from Rust physics engine
  }

  @override
  bool shouldRepaint(GamePainter oldDelegate) => true;
}
