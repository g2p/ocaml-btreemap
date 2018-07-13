build:
	cargo build --release
	jbuilder build

test: build
	jbuilder runtest --no-buffer --force

clean:
	rm -f src/*.a src/*.so Cargo.lock
	cargo clean
	jbuilder clean

