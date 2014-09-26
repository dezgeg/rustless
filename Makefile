all: rl

rl:
	cargo build
	@cp target/rl rl

clean:
	cargo clean

.PHONY: all rl clean
