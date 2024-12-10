build-rom-debug: build-debug
    ndspacker ./target/armv5te-none-eabi/debug/aocnds "./Infinite Space (USA).nds"

build-debug:
    cargo build --profile dev
