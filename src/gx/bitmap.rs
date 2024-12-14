use embedded_graphics::pixelcolor::raw::{RawU16, ToBytes};
use embedded_graphics::pixelcolor::{Bgr555, Bgr565, Rgb555};
use embedded_graphics::prelude::{DrawTarget, IntoStorage, RgbColor};
use embedded_graphics::prelude::{OriginDimensions, Size};
use embedded_graphics::Pixel;
use voladdress::{Safe, VolBlock};

use super::{set_display_mode, VRAMCTL_A};

/**
 * A direct pixel framebuffer for embedded graphics purposes.
 */
pub struct LcdFramebuffer {
    vram: VolBlock<u16, Safe, Safe, 131072>,
}

impl LcdFramebuffer {
    /**
     * Enables LCD framebuffer mode, and sets the appropriate VRAM bank.
     */
    pub fn enable() {
        set_display_mode(super::DisplayMode::VramBitmap);
        // MST 0, bit 7 = enable
        VRAMCTL_A.write(0b10000000);
    }

    pub fn new() -> LcdFramebuffer {
        let vram = unsafe { voladdress::VolBlock::new(0x6800000) };
        return LcdFramebuffer { vram };
    }
}

impl OriginDimensions for LcdFramebuffer {
    fn size(&self) -> embedded_graphics::prelude::Size {
        return Size::new(256, 192);
    }
}

impl DrawTarget for LcdFramebuffer {
    type Color = Bgr555;

    type Error = core::convert::Infallible;

    #[instruction_set(arm::t32)]
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        let bound = self.size();

        for Pixel(coord, colour) in pixels {
            if coord.x < 0 || coord.x > bound.width as i32 {
                continue;
            }

            if coord.y < 0 || coord.y > bound.height as i32 {
                continue;
            }

            let pos = coord.x + (coord.y * 256);
            let offset = self.vram.index(pos as usize);
            offset.write(colour.into_storage());
        }

        return Ok(());
    }
}
