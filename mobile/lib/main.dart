import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;

import 'src/api_base_resolver.dart';

void main() {
  runApp(const GameStackApp());
}

class GameStackApp extends StatelessWidget {
  const GameStackApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Game Stack',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: const Color(0xFF0F6B5B)),
      ),
      home: const GameScreen(),
    );
  }
}

class GameStateModel {
  const GameStateModel({
    required this.x,
    required this.y,
    required this.status,
  });

  final double x;
  final double y;
  final String status;

  factory GameStateModel.fromJson(Map<String, dynamic> json) {
    return GameStateModel(
      x: (json['x'] as num?)?.toDouble() ?? 0,
      y: (json['y'] as num?)?.toDouble() ?? 0,
      status: (json['status'] as String?) ?? 'active',
    );
  }
}

class GameScreen extends StatefulWidget {
  const GameScreen({super.key});

  @override
  State<GameScreen> createState() => _GameScreenState();
}

class _GameScreenState extends State<GameScreen> {
  static const double _characterSize = 48;

  late final String _apiBase;
  Offset _position = const Offset(0, 0);
  bool _loaded = false;
  String _status = 'loading';

  @override
  void initState() {
    super.initState();
    _apiBase = _resolveApiBaseUrl();
    _loadGameState();
  }

  Future<void> _loadGameState() async {
    try {
      final response = await http.get(Uri.parse('$_apiBase/game-state'));
      if (response.statusCode != 200) {
        throw Exception('status code ${response.statusCode}');
      }

      final body = jsonDecode(response.body) as Map<String, dynamic>;
      final gameState = GameStateModel.fromJson(body);

      if (!mounted) {
        return;
      }

      setState(() {
        _position = Offset(gameState.x, gameState.y);
        _status = gameState.status;
        _loaded = true;
      });
    } catch (_) {
      if (!mounted) {
        return;
      }
      setState(() {
        _position = const Offset(0, 0);
        _status = 'offline';
        _loaded = true;
      });
    }
  }

  void _onPanUpdate(DragUpdateDetails details, Size bounds) {
    final maxX = bounds.width - _characterSize;
    final maxY = bounds.height - _characterSize;
    final next = _position + details.delta;

    setState(() {
      _position = Offset(next.dx.clamp(0, maxX), next.dy.clamp(0, maxY));
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Game Stack')),
      body: LayoutBuilder(
        builder: (context, constraints) {
          final maxX = constraints.maxWidth - _characterSize;
          final maxY = constraints.maxHeight - _characterSize;
          final safePosition = Offset(
            _position.dx.clamp(0, maxX),
            _position.dy.clamp(0, maxY),
          );

          return Stack(
            children: [
              Positioned(
                top: 16,
                left: 16,
                child: Text(
                  _loaded ? 'status: $_status' : 'loading state...',
                  key: const ValueKey('game_status_label'),
                ),
              ),
              Positioned(
                left: safePosition.dx,
                top: safePosition.dy,
                child: GestureDetector(
                  key: const ValueKey('character_drag_target'),
                  onPanUpdate: (details) {
                    _onPanUpdate(
                      details,
                      Size(constraints.maxWidth, constraints.maxHeight),
                    );
                  },
                  child: Container(
                    key: const ValueKey('character_box'),
                    width: _characterSize,
                    height: _characterSize,
                    color: const Color(0xFF0F6B5B),
                    alignment: Alignment.center,
                    child: const Text(
                      'Character',
                      key: ValueKey('character_label'),
                      style: TextStyle(fontSize: 10, color: Colors.white),
                    ),
                  ),
                ),
              ),
            ],
          );
        },
      ),
    );
  }
}

String _resolveApiBaseUrl() {
  const fromEnv = String.fromEnvironment('API_BASE_URL', defaultValue: '');
  if (fromEnv.isNotEmpty) {
    return fromEnv;
  }

  if (kIsWeb) {
    return resolveWebApiBaseUrl();
  }

  return 'http://127.0.0.1:3000';
}
