use bitmap::Bitmap;

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
