use bitmap::Bitmap;
use sprites::Sprite;

pub fn tile_to_hex(tile_pixels: &[u8]) -> Bitmap {
    // Tile size must be 64 bytes
    assert_eq!(tile_pixels.len(), 64);

    let mut result: Bitmap = vec![0; 16];

    for y in 0..8 {
        // Each bit in the horizontal line is a pixel on the screen
        let mut first_bitplane_row: u8 = 0b0000_0000;
        let mut second_bitplane_row: u8 = 0b0000_0000;
        // For each pixel in the horizontal line
        for x in 0..8 {
            // If the pixel is not white I set the respective bit
            // That corresponds to the pixel that I want to turn on
            if tile_pixels[(y * 8) + x] == 1 {
                first_bitplane_row |= set_bit(x);
            }

            if tile_pixels[(y * 8) + x] == 2 {
                second_bitplane_row |= set_bit(x);
            }

            if tile_pixels[(y * 8) + x] == 3 {
                first_bitplane_row |= set_bit(x);
                second_bitplane_row |= set_bit(x);
            }
        }
        // First bit plane
        result[y] = first_bitplane_row;
        // Second bit plane
        result[y + 8] = second_bitplane_row;
    }

    // Result must be 16 bytes
    assert_eq!(result.len(), 16);

    result
}

pub fn set_bit(index: usize) -> u8 {
    let mut result: u8 = 0b1000_0000;
    result >>= index;
    result
}

#[test]
fn set_bit_green() {
    assert_eq!(set_bit(5), 0b0000_0100);
}

#[test]
#[should_panic]
fn set_bit_red() {
    assert_eq!(set_bit(5), 0b0000_0010);
}

#[test]
fn bitwise_or_green() {
    let first_value = set_bit(0);
    assert_eq!(first_value, 0b1000_0000);

    let second_value = set_bit(7);
    assert_eq!(second_value, 0b0000_0001);

    let result = first_value | second_value;
    assert_eq!(result, 0b1000_0001);
}

#[test]
#[should_panic]
fn bitwise_or_red() {
    let first_value = set_bit(0);
    assert_eq!(first_value, 0b1100_0000);

    let second_value = set_bit(7);
    assert_eq!(second_value, 0b0000_0011);

    let result = first_value | second_value;
    assert_eq!(result, 0b1000_0001);
}

#[test]
fn tile_to_hex_first_green() {
    let mut input_test: [u8; 64] = [0; 64];
    input_test[0] = 3;

    let mut output_test: [u8; 16] = [0; 16];
    output_test[0] = 0b1000_0000;
    output_test[8] = 0b1000_0000;

    let output = tile_to_hex(&input_test);

    assert_eq!(&output[0..16], &output_test[0..16]);
}

#[test]
#[should_panic]

fn tile_to_hex_first_red() {
    let mut input_test: [u8; 64] = [0; 64];
    input_test[0] = 3;

    let mut output_test: [u8; 16] = [0; 16];
    output_test[0] = 0b00000000;
    output_test[8] = 0b00000000;

    let output = tile_to_hex(&input_test);

    assert_eq!(&output[0..16], &output_test[0..16]);
}

#[test]
fn tile_to_hex_second_green() {
    #[rustfmt::skip]
    let input_test: [u8; 64] = [
        3, 3, 3, 3, 3, 3, 3, 3, 
        3, 2, 1, 0, 0, 1, 2, 3, 
        3, 1, 0, 0, 0, 0, 1, 3, 
        3, 0, 0, 0, 0, 0, 0, 3, 
        3, 0, 0, 0, 0, 0, 0, 3, 
        3, 1, 0, 0, 0, 0, 1, 3, 
        3, 2, 1, 0, 0, 1, 2, 3, 
        3, 3, 3, 3, 3, 3, 3, 3,
    ];

    let mut output_test: [u8; 16] = [0; 16];

    // First bit plane
    output_test[0] = 0b1111_1111;
    output_test[1] = 0b1010_0101;
    output_test[6] = 0b1010_0101;
    output_test[7] = 0b1111_1111;
    // Second bit plane
    output_test[8] = 0b1111_1111;
    output_test[9] = 0b1100_0011;
    output_test[14] = 0b1100_0011;
    output_test[15] = 0b1111_1111;

    let output = tile_to_hex(&input_test);

    // Test first bit plane
    assert_eq!(&output[0..2], &output_test[0..2]);
    assert_eq!(&output[6..8], &output_test[6..8]);
    // Test second bit plane
    assert_eq!(&output[8..10], &output_test[8..10]);
    assert_eq!(&output[14..16], &output_test[14..16]);
}

#[test]
#[should_panic]

fn tile_to_hex_second_red() {
    #[rustfmt::skip]
    let input_test: [u8; 64] = [
        3, 3, 3, 3, 3, 3, 3, 3, 
        3, 2, 1, 0, 0, 1, 2, 3, 
        3, 1, 0, 0, 0, 0, 1, 3, 
        3, 0, 0, 0, 0, 0, 0, 3, 
        3, 0, 0, 0, 0, 0, 0, 3, 
        3, 1, 0, 0, 0, 0, 1, 3, 
        3, 2, 1, 0, 0, 1, 2, 3, 
        3, 3, 3, 3, 3, 3, 3, 3,
    ];

    let mut output_test: [u8; 16] = [0; 16];

    // First bit plane
    output_test[0] = 0b1111_1111;
    output_test[1] = 0b1100_0011;
    output_test[6] = 0b1100_0011;
    output_test[7] = 0b1111_1111;
    // Second bit plane
    output_test[8] = 0b1111_1111;
    output_test[9] = 0b1010_0101;
    output_test[14] = 0b1010_0101;
    output_test[15] = 0b1111_1111;

    let output = tile_to_hex(&input_test);

    // Test first bit plane
    assert_eq!(&output[0..2], &output_test[0..2]);
    assert_eq!(&output[6..8], &output_test[6..8]);
    // Test second bit plane
    assert_eq!(&output[8..10], &output_test[8..10]);
    assert_eq!(&output[14..16], &output_test[14..16]);
}
