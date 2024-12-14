use voladdress::{Safe, VolAddress};

pub mod bitmap;

pub static DISPCNT_A: VolAddress<u32, Safe, Safe> = unsafe {
    VolAddress::new(0x4000000)
};

pub static VRAMCTL_A: VolAddress<u8, (), Safe> = unsafe {
    VolAddress::new(0x4000240)
};

pub enum DisplayMode {
    /** Screen off */
    Off = 0,
    /** Regular background + obj graphics */
    Graphics = 1,
    /** Bitmap from VRAM (Disp A only) */
    VramBitmap = 2,
    /** Bitmap from main memory (Disp A only) */
    DmaBitmap = 3,
}

/**
 * Sets the display mode for display engine A.
 */
pub fn set_display_mode(mode: DisplayMode) {
    // Bit 16-17
    let bits = (mode as u32) << 16;
    let mut dispcnt = DISPCNT_A.read();
    dispcnt |= bits;
    DISPCNT_A.write(dispcnt);
}
