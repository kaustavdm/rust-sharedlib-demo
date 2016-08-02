# Runtime loading of dynamic libaries

This is a demo of loading libraries at runtime using [sharedlib](https://github.com/Tyleo/sharedlib) in Rust.

## Usage


### Build

To build using default toolchain (assuming `stable`) run:

```
$ make stable
```

To build using nightly toolchain, run:

```
$ make nightly
```

Both of these will create dynamic libaries in the "./target/" folder in the current project root.

### Run

To run, execute the compiled `sharedlib_test` binary from the project root with the name of the library to load as the first argument. e.g.

```
$ ./target/debug/sharedlib_test foo
$ ./target/debug/sharedlib_test bar
$ ./target/debug/sharedlib_test hello
```