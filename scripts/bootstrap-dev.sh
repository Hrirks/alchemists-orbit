#!/usr/bin/env bash
set -euo pipefail

require_cmd() {
  local cmd="$1"
  local hint="$2"
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "[missing] $cmd"
    echo "hint: $hint"
    return 1
  fi
  echo "[ok] $cmd"
}

detect_compose() {
  if command -v docker >/dev/null 2>&1; then
    echo "docker compose"
    return 0
  fi
  if command -v podman >/dev/null 2>&1; then
    echo "podman compose"
    return 0
  fi
  return 1
}

install_maestro_if_possible() {
  if command -v maestro >/dev/null 2>&1; then
    return 0
  fi

  if command -v curl >/dev/null 2>&1; then
    echo "Installing Maestro CLI..."
    curl -fsSL "https://get.maestro.mobile.dev" | bash
    if [ -x "$HOME/.maestro/bin/maestro" ]; then
      export PATH="$HOME/.maestro/bin:$PATH"
      echo "[ok] maestro installed to $HOME/.maestro/bin"
      return 0
    fi
  fi

  return 1
}

configure_java_env() {
  if [ -d "/opt/homebrew/opt/openjdk@21/libexec/openjdk.jdk/Contents/Home" ]; then
    export JAVA_HOME="/opt/homebrew/opt/openjdk@21/libexec/openjdk.jdk/Contents/Home"
    export PATH="/opt/homebrew/opt/openjdk@21/bin:$PATH"
  fi
}

echo "== Game-Stack bootstrap =="
configure_java_env

require_cmd go "Install Go 1.26+ from https://go.dev/dl" || true
require_cmd flutter "Install Flutter 3.41+ from https://docs.flutter.dev/get-started/install" || true

if ! COMPOSE_CMD="$(detect_compose)"; then
  echo "[missing] docker/podman"
  echo "hint: install Docker Desktop or Podman"
  exit 1
fi
echo "[ok] compose via: $COMPOSE_CMD"

if ! command -v java >/dev/null 2>&1; then
  echo "[missing] java"
  echo "hint: install OpenJDK 21+ (e.g. brew install openjdk@21)"
  exit 1
fi
echo "[ok] java"
export PATH="$HOME/.maestro/bin:$PATH"

if ! install_maestro_if_possible; then
  echo "[missing] maestro"
  echo "hint: install manually from https://maestro.mobile.dev/getting-started/installing-maestro"
  exit 1
fi
echo "[ok] maestro"

echo "== Running stack =="
make up

echo "== Running Maestro test =="
make test

echo "Done. App: http://localhost:8080  API docs: http://localhost:3000/docs"
