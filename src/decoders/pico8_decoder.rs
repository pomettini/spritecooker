use sprites::pico8_sprite::Pico8Sprite;

type Bitmap = Vec<u8>;

struct Pico8Cart {
    data: Bitmap,
    sprites: Vec<Pico8Sprite>,
}

impl Pico8Cart {
    pub fn new(data: Bitmap) -> Pico8Cart {
        Pico8Cart {
            data,
            sprites: Vec::new(),
        }
    }
}

pub fn pico8_decode(sprite: &Pico8Sprite) -> Bitmap {
    Vec::new()
}

#[test]
fn pico8_decoder_test_first_green() {
    let input_test = "012300004567000089ab0000cdef0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

    // let mut output_test: [u8; 1] = [0; 1];
    // output_test[0] = 0;

    // let output = bmp_to_vga(&input_test, 1);

    // assert_eq!(&output[0..1], &output_test[0..1]);
}
