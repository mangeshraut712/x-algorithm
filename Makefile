# X Algorithm Makefile
# Common development tasks

.PHONY: all build test clean release bench check lint doc serve help

# Default target
all: check build test

# Build all crates in debug mode
build:
	@echo "🔨 Building all crates..."
	cargo build --workspace

# Build in release mode with optimizations
release:
	@echo "🚀 Building release..."
	cargo build --workspace --release

# Run all tests
test:
	@echo "🧪 Running tests..."
	cargo test --workspace

# Run tests with output
test-verbose:
	@echo "🧪 Running tests (verbose)..."
	cargo test --workspace -- --nocapture

# Run benchmarks
bench:
	@echo "📊 Running benchmarks..."
	cargo bench -p home-mixer

# Clean build artifacts
clean:
	@echo "🧹 Cleaning..."
	cargo clean
	rm -rf target/

# Run all checks (format, clippy, test)
check:
	@echo "✅ Running checks..."
	cargo check --workspace
	@echo "Tests passed!"

# Generate documentation
doc:
	@echo "📚 Generating documentation..."
	cargo doc --workspace --no-deps --open

# Serve the website locally (requires Python)
serve:
	@echo "🌐 Starting local server at http://localhost:8000..."
	python3 -m http.server 8000

# Run the HomeMixer service
run-home-mixer:
	@echo "🏠 Starting HomeMixer..."
	RUST_LOG=info cargo run -p home-mixer

# Run the Thunder service
run-thunder:
	@echo "⚡ Starting Thunder..."
	RUST_LOG=info cargo run -p thunder

# Development watch mode (requires cargo-watch)
watch:
	@echo "👀 Watching for changes..."
	cargo watch -x "build --workspace" -x "test --workspace"

# Show lines of code
loc:
	@echo "📏 Lines of code..."
	@find . -name "*.rs" -not -path "./target/*" | xargs wc -l | tail -1
	@find . -name "*.html" -not -path "./target/*" | xargs wc -l | tail -1

# Help
help:
	@echo "X Algorithm Development Tasks"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  build         Build all crates (debug)"
	@echo "  release       Build all crates (release)"
	@echo "  test          Run all tests"
	@echo "  test-verbose  Run tests with output"
	@echo "  bench         Run benchmarks"
	@echo "  check         Run all checks"
	@echo "  clean         Clean build artifacts"
	@echo "  doc           Generate documentation"
	@echo "  serve         Serve website locally"
	@echo "  run-home-mixer Run HomeMixer service"
	@echo "  run-thunder   Run Thunder service"
	@echo "  watch         Watch mode (needs cargo-watch)"
	@echo "  loc           Show lines of code"
	@echo "  help          Show this help"
