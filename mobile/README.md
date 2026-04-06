# Mobile Client

Flutter client for the Game-Stack test setup.

## Features

- Fetches initial position from `/game-state`.
- Renders a draggable `Character` box with stable widget keys for automation.
- Supports Android, iOS, and Web (Wasm build supported).

## Run

- Local mobile default:

  `flutter run --dart-define=API_BASE_URL=http://127.0.0.1:3000`

- iOS simulator example (if needed):

  `flutter run -d ios --dart-define=API_BASE_URL=http://127.0.0.1:3000`

- Web dev:

  `flutter run -d chrome --dart-define=API_BASE_URL=http://localhost:3000`

## Build (Wasm)

`flutter build web --wasm --release -O4 --dart-define=API_BASE_URL=/api`
