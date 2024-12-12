use core::arch::global_asm;

use voladdress::{Safe, VolAddress};

use crate::supervisor::SWI_Halt;

// Whilst only the lower bit is used, this isn't a boolean because representing non-0 or 1 as
// a boolean is UB.
pub static REG_IME: VolAddress<u32, Safe, Safe> =
    unsafe { VolAddress::new(0x4000208) };

pub static REG_IE: VolAddress<u32, Safe, Safe> =
    unsafe { VolAddress::new(0x4000210) };

pub static REG_IF: VolAddress<u32, Safe, Safe> =
    unsafe { VolAddress::new(0x4000214) };

pub static REG_DISPSTAT: VolAddress<u32, Safe, Safe> =
    unsafe { VolAddress::new(0x4000004) };


global_asm!("
    .section .text
    .global _set_irq_flags
    _set_irq_flags:
        ldr r1, =__dtcm_region_end
        str r0, [r1, #-0x4]
        bx lr
");

unsafe extern "C" {
    fn _set_irq_flags(flags: u32);
}


#[unsafe(no_mangle)]
pub extern "C" fn irq_handler() {
    REG_IF.write(0b1);
    unsafe {
        _set_irq_flags(0b1);
    }
}

/**
 * Waits for the next interrupt, based on the provided mask.
 */
pub fn wait_for_interrupt(mask: u32) {
    let old_mask = REG_IE.read();

    REG_IE.write(mask);
    // enable global interrupts
    REG_IME.write(1);

    unsafe {
        SWI_Halt();
    }

    REG_IME.write(0);
    REG_IE.write(old_mask);
}

/**
 * Waits for the next vertical blank interrupt.
 */
pub fn wait_for_vblank() {
    let old_disp_stat = REG_DISPSTAT.read();
    let new_disp_stat = old_disp_stat | 0b1000;

    REG_DISPSTAT.write(new_disp_stat);
    wait_for_interrupt(0b1);
    REG_DISPSTAT.write(old_disp_stat);
}
