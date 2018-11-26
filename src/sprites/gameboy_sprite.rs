use sprites::Sprite;

type Bitmap = Vec<u8>;

pub struct GameboySprite {
    sprite: Sprite,
}

impl GameboySprite {
    pub fn new(data: Bitmap) -> GameboySprite {
        let sprite = Sprite::new(8, 8, data);
        GameboySprite { sprite }
    }
}
