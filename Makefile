all: build copy

build:
	cargo build --release

copy:
	cp target/release/brainfuck .

clean:
	cargo clean
	rm brainfuck