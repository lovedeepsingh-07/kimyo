default:
	just -l

build:
	@mkdir -p build
	@cargo build --release
	@cp target/release/libkimyo.so build/kimyo.so
run: build
	@echo "------ executing ------"
	@lua main.lua
