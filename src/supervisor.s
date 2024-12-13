.section .text

// See the definitions in ``supervisor.rs`` for more information.
// Caution: When invoking SWIs from inside of ARM state specify SWI NN*10000h, instead of 
// SWI NN as in THUMB state.

.global SWI_Halt
SWI_Halt:
    swi #0x60000
    bx lr
