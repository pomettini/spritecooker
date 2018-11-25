pub struct Sprite
{
    width: usize,
    height: usize,
    data: Vec<u8>
}

impl Sprite
{
    pub fn new(width: usize, height: usize, data: Vec<u8>) -> Sprite {
        Sprite {
            width,
            height,
            data
        }
    }
}