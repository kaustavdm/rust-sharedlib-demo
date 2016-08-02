stable:
	cargo build
	rustc libs/hello.rs --crate-name hello --crate-type dylib --out-dir target
	rustc libs/foo.rs --crate-name foo --crate-type dylib --out-dir target
	rustc libs/bar.rs --crate-name bar --crate-type dylib --out-dir target

stable-bin:
	cargo build --release

nightly:
	rustup run nightly cargo build
	rustup run nightly rustc libs/hello.rs --crate-name hello --crate-type dylib --out-dir target
	rustup run nightly rustc libs/foo.rs --crate-name foo --crate-type dylib --out-dir target
	rustup run nightly rustc libs/bar.rs --crate-name bar --crate-type dylib --out-dir target

musl:
	cargo build --target x86_64-unknown-linux-musl
	rustc libs/hello.rs --crate-name hello --crate-type dylib --out-dir target
	rustc libs/foo.rs --crate-name foo --crate-type dylib --out-dir target
	rustc libs/bar.rs --crate-name bar --crate-type dylib --out-dir target

package:
	cargo build --release
	rm -rf package/
	mkdir -pv package/target
	cp -v target/release/sharedlib_test package/
	cp target/lib*.* package/target/

.PHONY: build nightly musl stable-bin package
