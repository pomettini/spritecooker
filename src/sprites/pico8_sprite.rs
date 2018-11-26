use sprites::Sprite;

type Bitmap = Vec<u8>;

pub struct Pico8Sprite {
    sprite: Sprite,
}

impl Pico8Sprite {
    pub fn new(data: Bitmap) -> Pico8Sprite {
        let sprite = Sprite::new(8, 8, data);
        Pico8Sprite { sprite }
    }

    pub fn data(self) -> Bitmap {
        self.sprite.data
    }
}
