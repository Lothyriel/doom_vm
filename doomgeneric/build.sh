cargo build --manifest-path rs_bind/Cargo.toml
gcc -o doom src/*.c -Lrs_bind/target/debug -lbinds -Wl,-rpath,rs_bind/target/debug
