.section ".crt","ax"
.global _start

// TODO: Need to copy ITCM and DTCM
// TODO: Replace the C1/C0 constants with ones not stolen from the decompiled game.

setup_coprocessor:
    // The creatively named MCR and MRC stand for "Move Coprocessor to Register" and
    // "Move Register to Coprocessor", respectively. The names are misleading; it really means
    // "do coprocessor command".

    // C1/C0/0 = System control, copy the definition into R0
    mrc p15, 0x0, r0, cr1, cr0, 0x0

    // load constant control value
    // this (temporarrily) disables the protection unit, the DTCM and ITCM, and disables 
    // caching for them
    ldr r1, =0x000F90053
    // BIC = Rd AND (NOT Rn)
    // clear any set bits in R0 that are set in R1, leave the rest alone
    bic r0, r0, r1
    // write it back
    mcr p15, 0x0, r0, cr1, cr0, 0x0

    // Disable caches for both TCMs (?)
    // C7,C5,0   0    Yes  Invalidate Entire Instruction Cache
    // C7,C6,0   0    Yes  Invalidate Entire Data Cache
    mov r0, #0
    mcr p15, 0, r0, c7, c5, 0
    mcr p15, 0, r0, c7, c6, 0
    // C7,C10,4  0    -    Drain Write Buffer
    mcr p15, 0, r0, c7, c10, 4

    // == Memory Protection == //
    // The protection regions are almost identical to the ones on GBATEK, which are in themselves
    // identical to the ones setup by the CRT of the game I've decompiled.
    // 
    // Control register C6 defines the region, C0-C7 all define a specific subregion.
    // Bit   0: 1 = enable protection, 0 = disable protection
    // Bit 1-5: 2 SHL value = region size
    // Bit 6-11: reserved
    // Bit 12-31: Region address * 4096
    //
    // The official ARM docs marks sizees less than 0b01011 as unpredictable, so the base unit is
    // in 4KB blocks?
    //
    // "The address of the first byte is required to be a multiple of the region size."

    // Protection region 0: 0x04000000, 64MiB (i.e. up to 0x8000000)
    // This is the I/O registers all the way up to the end of the OAM!
    ldr r0, =(0x04000000 | 0x33)
    mcr p15, 0, r0, c6, c0, 0

    // Protection region 1: 0x02000000, 4MiB
    // The compiled game I'm looking at has it incorrectly set to 8MiB. I guess the SDK always
    // sets it that high? This is main memory.
    ldr r0, =(0x02000000 | 0x2b)
    mcr p15, 0, r0, c6, c1, 0
    
    // Protection region 2: 0x027e0000, 128KiB (what?)
    // GBATEK: Region 2 and 7 are not understood?
    // Not going to set this. All zeroes to disable memory protection.
    mov r0, 0
    mcr p15, 0, r0, c6, c2, 0

    // Protection region 3: 0x08000000, 128MiB 
    // GBATEK: GBA Slot should be max 32MB+64KB, rounded up to 64MB, no idea why it is 128MB?
    ldr r0, =(0x08000000 | 0x35)
    mcr p15, 0, r0, c6, c3, 0

    // Protection region 4: 0x027e0000, 16KiB
    // This is the DTCM.
    ldr r0, =__dtcm_region_start
    orr r0, r0, #0x1b
    mcr p15, 0, r0, c6, c4, 0

    // Protection region 5: 0x01000000, 32KiB
    // ITCM. Thanks to mirroring, this repeats itself every 32KiB.
    ldr r0, =__itcm_region_start
    orr r0, r0, #0x1d
    mcr p15, 0, r0, c6, c5, 0

    // Protection region 6: 0xFFFF0000, 32KiB.
    // This is where the BIOS is mapped.
    ldr r0, =(0xFFFF0000 | 0x1d)
    mcr p15, 0, r0, c6, c6, 0

    // Protection region 7: 0x027FF000, 4KiB.
    // GBATEK says "shared work", which is more evidence in the pile that it's based off of leaked
    // SDKs. The actual shared WRAM area is at the 0x03... addresses.
    // So... let's set it there.
    // Protection region 7: 0x037F8000, 32KiB. 
    ldr r0, =__shram_region_start
    orr r0, r0, #0x1d
    mcr p15, 0, r0, c6, c7, 0 

    // Protection region 2: 0x027FF00, 4KiB.
    // This is BIOS ram, see NDS BIOS RAM usage in GBATEK. Only realised this when looking through
    // addresses. 
    ldr r0, =(0x027FF00 | 0x17)
    mcr p15, 0, r0, c6, c2, 0

    // == Tightly Coupled Memory == //
    // C9, C1 controls the TCM Region. 
    //
    // The ARM manual states "Prior to ARMv6 it is IMPLEMENTATION DEFINED how TCMs are supported, 
    // "though generally this is through a System Control Coprocessor interface.""
    //
    // ITCM is fixed, so just set the size to 32MiB so it covers the entire first part of memory 
    // space. It'll get mirrored apparently.
    // Table B7-2: 32MiB is 0b10000 (<< 1), 16KiB is 0b00101 (<< 1).
    mov r0, 0x20
    mcr p15, 0, r0, c9, c1, 1

    // DTCM is movable, so load it at the right address and set its size to 16KiB.
    ldr r0, =__dtcm_region_start
    orr r0, r0, 0xa
    mcr p15, 0, r0, c9, c1, 0

    // == Protection Unit, Pt 2 == //
    // Register C2,C0 controls data caching and it's a bitfield for every region that needs caches.
    // 0x1 = instructions, 0x0 = data
    //
    // The only regions that needs caching is main memory, which is region 1, and the BIOS, which
    // is region 6. (The bitfield starts from the LSB.)
    mov r0, #0b01000010
    mcr p15, 0, r0, c2, c0, 0
    mcr p15, 0, r0, c2, c0, 1

    // C3,C0,0 is... write-bufferability? This is too far into the details of CPUs for me.
    // Just do what the official CRT does, which is region 1 (main memory).
    mov r0, #0b00000010
    mcr p15, 0, r0, c3, c0, 0

    // C5,C0 controls the permissions for the various memory protection regions. Immediate
    // value 2 and 3 control *extended* permissions, which give 4 bits per region with up to
    // six values. 2 = Data/Unified, 3 = Instruction. Immediate value 0 and 1 control basic 
    // permissions, with two bits per region.
    // 
    // We're just going to fill this with 0b11 for all eight regions as constructing the individual
    // permission bits is fiddly and not really needed.
    ldr r0, =0xffff
    mcr p15, 0, r0, c5, c0, 0
    mcr p15, 0, r0, c5, c0, 1

    // Re-enable ITCM, DTCM, caches, and protection unit.
    mrc p15, 0, r0, c1, c0, 0
    ldr r1, =0x0005707D
    orr r0, r0, r1
    mcr p15, 0, r0, c1, c0, 0

    bx lr

.arm 

// R0: value
// R1: ptr to start
// R2: size
_ASM_primitive_memset:
    // R12 (end) = start + size
    add r12, r1, r2
.L0:
    // Compare current pointer to end
    cmp r1, r12
    // Store if less than, increment multiple registers
    // *R1 = R0, R1 += 4
    // Also, the double braces are because this is being compiled with ``global_asm!()``, and single
    // braces means it would think it's a parameter and flip out at me for not providing an ``R0``
    // parameter.
    // Doesn't clear the condition bits so...
    stmialt r1!, {{R0}}
    // ... jump back if the condition still matches
    blt .L0
    // Return
    bx lr

_start:
    // The IME is at address 0x4000208, and is a 32-bit register.
    // The only bit in it that matters is bit zero, which acts as the enable bit.
    // 1 = interrupts controlled by IE
    // 0 = interrupts forcibly disabled.
    // 
    // The ``mov`` instruction with an immediate can only operate on any 8-bit value that is shifted
    // by an even power of two, which 0x04000208 is not. 0x04000000 (the base address for I/O 
    // registers) is, so that's loaded into register zero. Then, exploiting the fact that the 
    // LSB of 0x04000000 is 0, we store that into IME and the upper bits are ignored.

    mov r0, #0x04000000
    str r0, [r0, #0x208]

    // Wait for vertical sync.
    // Note: The syntax ``.L<name>`` signifies a local label, which isn't exported as a symbol
    // in the final compiled object.
.Lvsync:
    // Load half-word at REG_DISP_VCOUNT. R0 was already the base of the I/O registers, so we
    // can just add 0x6 to it to load it into R1.
    ldrh r1, [r0, #0x6]
    // Compare to 0x0 and set the comparison flag.
    cmp r1, #0
    // Branch if not equal back to the VSYNC label.
    bne .Lvsync

    // Jump to a separate function to set up the CP15. There's a lot of noise that we don't 
    // really want clogging up our main function.
    bl setup_coprocessor

    // Stack setup for every mode
    // 0b10010 == 0x12, IRQ mode
    mov r0, #0x12
    msr cpsr, r0
    ldr sp, =__stack_start_irq
    
    // 0b10011 == 0x13, Supervisor mode
    mov r0, #0x13
    msr cpsr, r0
    ldr sp, =__stack_start_svc

    // 0b11111 == 0x1f, System mode
    mov r0, #0x1f
    msr cpsr, r0
    ldr sp, =__stack_start_sys

    // Clear BSS
    ldr r2, =__bss_size
    ldr r1, =__bss_start
    mov r0, #0
    bl _ASM_primitive_memset

    // Less of a minefield to just set the IRQ handler in ``_start``
    ldr r0, =irq_handler
    ldr r1, =__dtcm_region_end
    str r0, [r1]

    // After all of our setup is done, we can finally switch to main.
    // For future-proofing, we do a branch-with-exchange in case ``main`` ends up as a thumb
    // function.
    ldr r0, =main
    bx r0
