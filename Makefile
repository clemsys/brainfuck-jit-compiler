all: build copy

build:
	cargo build --release

copy:
	cp target/release/brainfuck-jit-compiler .

clean:
	cargo clean
	rm brainfuck-jit-compiler