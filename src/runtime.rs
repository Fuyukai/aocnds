//! Runtime components for AOC NDS.

use core::arch::asm;

use talc::{ErrOnOom, Span, Talc, Talck};

use crate::interrupts::REG_IME;

#[panic_handler]
pub fn _handle_panic(_: &core::panic::PanicInfo) -> ! {
    // TODO: Do something better than this
    unsafe {
        asm!("bkpt");
    }
    loop {}
}

// Embedded ABI C++ exception handlers. These are stubs because we are not in C++.
#[unsafe(no_mangle)]
pub fn __aeabi_unwind_cpp_pr0() -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub fn __aeabi_unwind_cpp_pr1() -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub fn __aeabi_unwind_cpp_pr2() -> ! {
    loop {}
}

#[global_allocator]
static ALLOCATOR: Talck<spin::Mutex<()>, ErrOnOom> = Talc::new(ErrOnOom).lock();
struct NitroCriticalSection;

unsafe impl critical_section::Impl for NitroCriticalSection {
    unsafe fn acquire() -> critical_section::RawRestoreState {
        // Read it off first so that it can be restored properly. If the previous value was zero,
        // we need the restore to remain zero!
        let prev = REG_IME.read();
        REG_IME.write(0);
        return prev;
    }

    unsafe fn release(restore_state: critical_section::RawRestoreState) {
        REG_IME.write(restore_state);
    }
}

critical_section::set_impl!(NitroCriticalSection);

unsafe extern "C" {
    static mut __text_end: u8;
    static mut __memory_end: u8;
}

/**
 * Sets up the heap allocator with the memory span defined by the linker.
 */
#[allow(static_mut_refs)]
pub fn setup_heap_allocator() {
    // Unbelievably and wildly unsafe!
    unsafe {
        let text_end = &mut __text_end as *mut u8;
        let memory_end = &mut __memory_end as *mut u8;
        let span = Span::new(text_end, memory_end);
        ALLOCATOR.lock().claim(span).unwrap();
    }
}
