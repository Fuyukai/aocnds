#![no_std] // Obviously, we don't have a runtime.
#![no_main] // We use an extern "C" main instead of a Rust main.
#![allow(clippy::empty_loop)] // Otherwise clippy gets mad at our infinite loop.
#![allow(clippy::needless_return)]

#[instruction_set(arm::t32)]
extern crate alloc;

mod runtime;

use core::arch::global_asm;

use alloc::vec::Vec;
use runtime::setup_heap_allocator;
use voladdress::{Safe, VolAddress};

static PUZZLE_INPUT: &str = include_str!("day1.txt");

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    setup_heap_allocator();

    // Format of the file is XXXXX   XXXXX
    let mut first: Vec<u32> = Vec::new();
    let mut second: Vec<u32> = Vec::new();

    for line in PUZZLE_INPUT.split_terminator('\n') {
        let first_num: u32 = line[0..5].parse().unwrap();
        let second_num: u32 = line[8..13].parse().unwrap();
        first.push(first_num);
        second.push(second_num);
    }

    first.sort();
    second.sort();

    let mut sum: u32 = 0;
    for (first, second) in first.iter().zip(second) {
        let diff = ((*first as i32) - (second as i32)).abs();
        sum += diff as u32;
    }

    let output: VolAddress<u32, Safe, Safe> = unsafe { VolAddress::new(0x02200000) };
    output.write(sum);

    loop {}
}

global_asm!(include_str!("start.s"));
