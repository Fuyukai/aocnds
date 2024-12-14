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

use embedded_graphics::image::Image;
use embedded_graphics::mono_font::ascii::FONT_8X13;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Bgr555;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;
use embedded_graphics::{
    pixelcolor::Rgb555,
    prelude::{Point, Primitive},
    primitives::{PrimitiveStyle, Triangle},
};
use gx::{bitmap::LcdFramebuffer, set_display_mode};
use interrupts::wait_for_vblank;
use runtime::setup_heap_allocator;
use tinybmp::Bmp;
use voladdress::{Safe, VolAddress};

static BITMAP: &[u8] = include_bytes!("bitmap.bmp");

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    setup_heap_allocator();

    let vram: VolAddress<u8, Safe, Safe> = unsafe { VolAddress::new(0x4_000_240) };
    vram.write(0b10000000);

    set_display_mode(gx::DisplayMode::VramBitmap);
    LcdFramebuffer::enable();
    let mut lcd = LcdFramebuffer::new();

    let bmp = Bmp::from_slice(BITMAP).unwrap();
    Image::new(&bmp, Point::zero()).draw(&mut lcd).unwrap();

    let tri = Triangle::new(Point::new(10, 10), Point::new(100, 10), Point::new(10, 100))
        .into_styled(PrimitiveStyle::with_fill(Bgr555::new(31, 0, 0)));
    tri.draw(&mut lcd).unwrap();

    let font = MonoTextStyle::new(&FONT_8X13, Bgr555::CSS_PURPLE);
    Text::new("yum yum squid :)", Point::new(0, 175), font)
        .draw(&mut lcd).unwrap();

    loop {
        wait_for_vblank();
    }
}

global_asm!(include_str!("start.s"));
