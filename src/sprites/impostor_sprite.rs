use sprites::Sprite;

type Bitmap = Vec<u8>;

pub struct ImpostorSprite {
    sprite: Sprite,
}

impl ImpostorSprite {
    pub fn new(data: Bitmap) -> ImpostorSprite {
        let sprite = Sprite::new(16, 16, data);
        ImpostorSprite { sprite }
    }
}
