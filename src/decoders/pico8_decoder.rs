use sprite::Sprite;

struct Pico8Cart
{
    data: Vec<u8>,
    sprites: Vec<Sprite>
}

impl Pico8Cart
{
    pub fn new(data: Vec<u8>) -> Pico8Cart {
        Pico8Cart {
            data,
            sprites: Vec::new()
        }
    }
}

#[test]
fn pico8_decoder_test_first_green()
{
    assert_eq!("", "012300004567000089ab0000cdef0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000")
}