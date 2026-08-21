# Makefile to convert loading screen textures
# Assumes texconv is available in PATH (DirectX Texture Conversion Tool)

.DEFAULT_GOAL := help

.PHONY: help convert-loadingscreens run-dynamic-release build-dynamic-release build-dynamic run-dynamic build-release run-release build run

help:  ## Show this help message
	@echo "Usage: make [target]"
	@echo ""
	@echo "install texconv before running"
	@echo ""
	@echo "Available targets:"
	@echo "  help                   Show this help message"
	@echo "  convert-loadingscreens  Convert all .dds files in ./assets/gfx/loadingscreens"
	@echo "                         to 2048x1536 dds (no mipmaps) and save to"
	@echo "                         ./assets/gfx/loadingscreens/fixed/"
	@echo "  run-dynamic-release     Run release build with 'dynamic,debug' features"
	@echo "  build-dynamic-release   Build release with 'dynamic,debug' features"
	@echo "  build-dynamic           Build with 'dynamic,debug' features"
	@echo "  run-dynamic             Run with 'dynamic,debug' features"
	@echo "  build-static-release    Build release binary (cargo build --release)"
	@echo "  run--staticrelease      Run release binary (cargo run --release)"
	@echo "  build-static            Build debug binary (cargo build)"
	@echo "  run-static              Run debug binary (cargo run)"
	@echo ""
	@echo "Example:"
	@echo "  make convert-loadingscreens"

convert-loadingscreens:  ## Convert .dds → .dds (2048x1536, no mipmaps, fixed headers). Add -f BC7_UNORM if low VRAM
	cd ./assets/gfx/loadingscreens && mkdir fixed && texconv -ft dds  -m 1 -w 2048 -h 1536 -y *.dds -o fixed

run-dynamic-release:
	cargo run --release --features "dynamic,debug"

build-dynamic-release:
	cargo build --release --features "dynamic,debug"

build-dynamic:
	cargo build --features "dynamic,debug"

run-dynamic:
	cargo run --features "dynamic,debug"

# New targets added as requested
build-static-release:
	cargo build --release

run-static-release:
	cargo run --release

build-static:
	cargo build

run-static:
	cargo run