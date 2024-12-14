#![no_std] // Obviously, we don't have a runtime.
#![no_main] // We use an extern "C" main instead of a Rust main.
#![allow(clippy::empty_loop)] // Otherwise clippy gets mad at our infinite loop.
#![allow(clippy::needless_return)]

#[instruction_set(arm::t32)]
extern crate alloc;

mod interrupts;
mod runtime;
mod supervisor;

mod gx;

use core::arch::global_asm;

use alloc::format;
use alloc::vec::Vec;
use embedded_graphics::image::Image;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::FONT_8X13;
use embedded_graphics::pixelcolor::Bgr555;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;
use embedded_graphics::{
    pixelcolor::Rgb555,
    prelude::{Point, Primitive},
    primitives::{PrimitiveStyle, Triangle},
};
use gx::{DISPCNT_A, VRAMCTL_A};
use gx::{bitmap::LcdFramebuffer, set_display_mode};
use interrupts::wait_for_vblank;
use runtime::setup_heap_allocator;
use tinybmp::Bmp;
use voladdress::{Safe, VolAddress};

static BITMAP: &[u8] = include_bytes!("bitmap.bmp");
static PUZZLE_INPUT: &str = include_str!("day1.txt");

fn solve_aoc2021_pt1() -> u32 {
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

    return sum;
}

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    setup_heap_allocator();

    VRAMCTL_A.write(0b10000000);

    let mut dispcnt = DISPCNT_A.read();
    let bits = 2u32 << 16;
    dispcnt |= bits;
    DISPCNT_A.write(dispcnt);

    set_display_mode(gx::DisplayMode::VramBitmap);
    LcdFramebuffer::enable();
    let mut lcd = LcdFramebuffer::new();

    let bmp = Bmp::from_slice(BITMAP).unwrap();
    Image::new(&bmp, Point::zero()).draw(&mut lcd).unwrap();

    let tri = Triangle::new(Point::new(10, 10), Point::new(100, 10), Point::new(10, 100))
        .into_styled(PrimitiveStyle::with_fill(Bgr555::new(31, 0, 0)));
    tri.draw(&mut lcd).unwrap();

    let font = MonoTextStyle::new(&FONT_8X13, Bgr555::CSS_PINK);
    Text::new(
        format!("The solution is: {}", solve_aoc2021_pt1()).as_str(),
        Point::new(0, 175),
        font,
    )
    .draw(&mut lcd)
    .unwrap();

    loop {
        wait_for_vblank();
    }
}

global_asm!(include_str!("start.s"));
