default:
	just -l

build:
	@cargo build --release
run:
	@echo "------ executing ------"
	@cargo run --release -- ./examples
