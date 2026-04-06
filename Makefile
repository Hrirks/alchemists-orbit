SHELL := /bin/zsh

JAVA_HOME_FALLBACK := /opt/homebrew/opt/openjdk@21/libexec/openjdk.jdk/Contents/Home
MAESTRO_BIN := $(HOME)/.maestro/bin
JAVA_ENV := $(if $(wildcard $(JAVA_HOME_FALLBACK)),export JAVA_HOME="$(JAVA_HOME_FALLBACK)" && export PATH="/opt/homebrew/opt/openjdk@21/bin:$(MAESTRO_BIN):$$PATH" &&,export PATH="$(MAESTRO_BIN):$$PATH" &&)

.PHONY: bridge build-rust build-wasm test-rust test-flutter ci doctor clean

# Generate flutter_rust_bridge code
bridge:
	cd rust/api && flutter_rust_bridge_codegen generate

# Build Rust library
build-rust:
	cd rust && cargo build --release

# Run Rust tests
test-rust:
	cd rust && cargo test

# Run Flutter tests
test-flutter:
	cd mobile && flutter test

# Build Flutter web with Wasm
build-wasm:
	cd mobile && flutter build web --wasm --release -O4

# Build iOS
build-ios:
	cd mobile && flutter build ios --release

# Build Android
build-android:
	cd mobile && flutter build apk --release

# Run all CI checks
ci:
	$(MAKE) test-rust
	cd mobile && flutter analyze
	$(MAKE) test-flutter
	cd mobile && flutter build web --wasm --release -O4

# Check tool versions
doctor:
	@echo "Rust:      $$(command -v rustc >/dev/null 2>&1 && rustc --version || echo missing)"
	@echo "Cargo:     $$(command -v cargo >/dev/null 2>&1 && cargo --version || echo missing)"
	@echo "Flutter:   $$(command -v flutter >/dev/null 2>&1 && flutter --version | sed -n '1p' || echo missing)"
	@echo "FRB:       $$(command -v flutter_rust_bridge_codegen >/dev/null 2>&1 && flutter_rust_bridge_codegen --version || echo missing)"
	@echo "Maestro:   $$($(JAVA_ENV) command -v maestro >/dev/null 2>&1 && maestro --version || echo missing)"
	@echo "Java:      $$($(JAVA_ENV) command -v java >/dev/null 2>&1 && java -version 2>&1 | sed -n '1p' || echo missing)"

# Clean build artifacts
clean:
	cd rust && cargo clean
	cd mobile && flutter clean
	rm -rf mobile/build
