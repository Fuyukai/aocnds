#![no_std]   // Obviously, we don't have a runtime.
#![no_main]  // We use an extern "C" main instead of a Rust main.
#![allow(clippy::empty_loop)]  // Otherwise clippy gets mad at our infinite loop.
#![allow(clippy::needless_return)]

#[instruction_set(arm::t32)]
extern crate alloc;

mod runtime;

use core::arch::global_asm;

use runtime::setup_heap_allocator;


#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    setup_heap_allocator();
    loop {}
}

global_asm!(include_str!("start.s"));
