#![no_std] // Obviously, we don't have a runtime.
#![no_main] // We use an extern "C" main instead of a Rust main.
#![allow(clippy::empty_loop)] // Otherwise clippy gets mad at our infinite loop.
#![allow(clippy::needless_return)]

#[instruction_set(arm::t32)]
extern crate alloc;

mod runtime;
mod interrupts;
mod supervisor;

use core::arch::global_asm;

use interrupts::wait_for_vblank;
use runtime::setup_heap_allocator;

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    setup_heap_allocator();

    loop {
        wait_for_vblank();
    }
}

global_asm!(include_str!("start.s"));
