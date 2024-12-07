#![no_std]   // Obviously, we don't have a runtime.
#![no_main]  // We use an extern "C" main instead of a Rust main.
#![allow(clippy::empty_loop)]  // Otherwise clippy gets mad at our infinite loop.

use core::arch::global_asm;

// This is an empty panic handler as we don't have any infastructure to actually... well, handle
// panics in any form.
//
// On platforms with a runtime, ``std`` provides lots of helpful code to unwind all of the code
// and print a stacktrace, but we don't have that, so instead when ``panic!()`` (or, rather,
// ``core::panicking::panic``) is called, it calls this function instead.
//
// See https://fractalfir.github.io/generated_html/rustc_codegen_clr_v0_2_1.html for the
// nitty-gritty details.
#[panic_handler]
fn _handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Unsafe attributes are a new feature in Rust 2024, and means "you now need to prefix this
// attribute with unsafe". 
//
// Without ``no_mangle``, this function would get optimised out (as nothing calls it), and even
// if it didn't, it would be given an unintelligible name so that it wouldn't conflict with
// functions from other packages called ``main`` (as unwise as it would be to call themselves)
// that.
#[unsafe(no_mangle)]
#[allow(static_mut_refs)]
extern "C" fn main() -> ! {
    loop {}
}

global_asm!(include_str!("start.s"));
