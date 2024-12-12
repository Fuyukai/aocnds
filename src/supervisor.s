.section .text

// See the definitions in ``supervisor.rs`` for more information.
// Caution: When invoking SWIs from inside of ARM state specify SWI NN*10000h, instead of 
// SWI NN as in THUMB state.

.global SWI_Halt
SWI_Halt:
    push {{r0}}
    swi #0x60000
    pop {{r0}}
    bx lr
