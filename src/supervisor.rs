use core::arch::global_asm;


unsafe extern "C" {
    /**
     * Halts the current core, going into low power mode, until an interrupt is received.
     * 
     * If IME = 0, this will lock up the core forever.
     */
    pub fn SWI_Halt();
}


global_asm!(include_str!("supervisor.s"));
