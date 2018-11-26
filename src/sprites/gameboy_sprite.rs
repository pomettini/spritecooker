use sprites::Sprite;
use bitmap::Bitmap;

pub struct GameboySprite {
    sprite: Sprite,
}

impl GameboySprite {
    pub fn new(data: Bitmap) -> GameboySprite {
        let sprite = Sprite::new(8, 8, data);
        GameboySprite { sprite }
    }
}
