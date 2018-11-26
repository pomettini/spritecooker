use sprites::Sprite;
use bitmap::Bitmap;

pub struct ImpostorSprite {
    sprite: Sprite,
}

impl ImpostorSprite {
    pub fn new(data: Bitmap) -> ImpostorSprite {
        let sprite = Sprite::new(16, 16, data);
        ImpostorSprite { sprite }
    }
}
