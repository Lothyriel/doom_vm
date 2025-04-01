cargo build --manifest-path binds/Cargo.toml
gcc -o doom src/*.c -Lbinds/target/debug -lbinds -Wl,-rpath,binds/target/debug
