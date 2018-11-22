extern crate exoquant;
extern crate stb_image;

use exoquant::*;

pub fn get_gb_colors() -> [Color; 4] {
    let mut gb_colors: [Color; 4] = [Color::new(0, 0, 0, 0); 4];

    /* First row */
    gb_colors[0] = Color::new(155, 188, 15, 0);
    gb_colors[1] = Color::new(139, 172, 15, 0);
    gb_colors[2] = Color::new(48, 98, 48, 0);
    gb_colors[3] = Color::new(15, 56, 15, 0);

    gb_colors
}

pub fn bmp_to_2bpp(image: &[u8], image_width: &usize) -> Vec<u8> {
    let gb_colors = get_gb_colors();
    let mut exocolors: Vec<Color> = Vec::new();

    let image_size = image.len() / 3;

    for i in 0..image_size {
        let r = image[i * 3];
        let g = image[i * 3 + 1];
        let b = image[i * 3 + 2];

        let mut c = Colorf::zero();
        c.r = f64::from(r);
        c.g = f64::from(g);
        c.b = f64::from(b);

        exocolors.push(Color::new(r, g, b, 0));
    }

    let colorspace = SimpleColorSpace::default();
    let gb_ditherer = ditherer::FloydSteinberg::new();
    let gb_remapper = Remapper::new(&gb_colors, &colorspace, &gb_ditherer);

    gb_remapper.remap(&exocolors, *image_width)
}

pub fn twopp_to_bmp(image: &[u8]) -> Vec<u8> {
    let gb_colors = get_gb_colors();
    let mut colors: Vec<u8> = Vec::new();

    for pixel in image {
        let color_in_palette = gb_colors[*pixel as usize];
        colors.push(color_in_palette.r);
        colors.push(color_in_palette.g);
        colors.push(color_in_palette.b);
    }

    colors
}

pub fn tile_to_hex(tile_pixels: &[u8]) -> Vec<u8> {
    // Tile size must be 64 bytes
    assert_eq!(tile_pixels.len(), 64);

    let mut result: Vec<u8> = vec![0; 16];

    for y in 0..8 {
        // Each bit in the horizontal line is a pixel on the screen
        let mut first_bitplane_row: u8 = 0b00000000;
        let mut second_bitplane_row: u8 = 0b00000000;
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

pub fn print_tile(tile: Vec<u8>, tile_name: &str) {
    println!("{}", tile_name);
    println!();
}

pub fn set_bit(index: usize) -> u8 {
    let mut result: u8 = 0b10000000;
    result >>= index;
    result
}

#[test]
fn set_bit_green() {
    assert_eq!(set_bit(5), 0b00000100);
}

#[test]
#[should_panic]
fn set_bit_red() {
    assert_eq!(set_bit(5), 0b00000010);
}

#[test]
fn bitwise_or_green() {
    let first_value = set_bit(0);
    assert_eq!(first_value, 0b10000000);

    let second_value = set_bit(7);
    assert_eq!(second_value, 0b00000001);

    let result = first_value | second_value;
    assert_eq!(result, 0b10000001);
}

#[test]
#[should_panic]
fn bitwise_or_red() {
    let first_value = set_bit(0);
    assert_eq!(first_value, 0b11000000);

    let second_value = set_bit(7);
    assert_eq!(second_value, 0b00000011);

    let result = first_value | second_value;
    assert_eq!(result, 0b10000001);
}

#[test]
fn tile_to_hex_first_green() {
    let mut input_test: [u8; 64] = [0; 64];
    input_test[0] = 3;

    let mut output_test: [u8; 16] = [0; 16];
    output_test[0] = 0b10000000;
    output_test[8] = 0b10000000;

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
    output_test[0] = 0b11111111;
    output_test[1] = 0b10100101;
    output_test[6] = 0b10100101;
    output_test[7] = 0b11111111;
    // Second bit plane
    output_test[8] = 0b11111111;
    output_test[9] = 0b11000011;
    output_test[14] = 0b11000011;
    output_test[15] = 0b11111111;

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
    output_test[0] = 0b11111111;
    output_test[1] = 0b11000011;
    output_test[6] = 0b11000011;
    output_test[7] = 0b11111111;
    // Second bit plane
    output_test[8] = 0b11111111;
    output_test[9] = 0b10100101;
    output_test[14] = 0b10100101;
    output_test[15] = 0b11111111;

    let output = tile_to_hex(&input_test);

    // Test first bit plane
    assert_eq!(&output[0..2], &output_test[0..2]);
    assert_eq!(&output[6..8], &output_test[6..8]);
    // Test second bit plane
    assert_eq!(&output[8..10], &output_test[8..10]);
    assert_eq!(&output[14..16], &output_test[14..16]);
}
