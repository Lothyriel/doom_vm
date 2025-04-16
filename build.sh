cargo build --manifest-path rs_bind/Cargo.toml

gcc -o doom ./doomgeneric/src/*.c -Lrs_bind/target/debug -lrs_bind -Wl,-rpath,rs_bind/target/debug
