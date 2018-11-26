pub mod gameboy_sprite;
pub mod impostor_sprite;
pub mod pico8_sprite;

type Bitmap = Vec<u8>;

pub struct Sprite {
    width: usize,
    height: usize,
    data: Bitmap,
}

impl Sprite {
    pub fn new(width: usize, height: usize, data: Bitmap) -> Sprite {
        Sprite {
            width,
            height,
            data,
        }
    }
}
