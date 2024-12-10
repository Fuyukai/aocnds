build-rom-debug: build-debug
    ndspacker ./target/armv5te-none-eabi/debug/aocnds "./Infinite Space (USA).nds"

build-debug:
    cargo build --profile dev

gdb: build-rom-debug
    sh -c "RUST_GDB=arm-none-eabi-gdb exec rust-gdb ./target/armv5te-none-eabi/debug/aocnds"
