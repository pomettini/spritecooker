pub mod gameboy;
pub mod impostor;
pub mod pico8;

use image::*;

trait Encode {
    fn encode(&self) -> Image;
}

trait Decode {
    fn decode(source: Image);
}
