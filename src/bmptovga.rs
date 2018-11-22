extern crate exoquant;
extern crate stb_image;

#[path = "vgapalette.rs"]
pub mod vgapalette;

use exoquant::*;

pub fn bmp_to_vga(image: &[u8], image_width: &usize) -> Vec<u8> {
    let vga_colors = vgapalette::get_vga_colors();
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
    let vga_ditherer = ditherer::FloydSteinberg::new();
    let vga_remapper = Remapper::new(&vga_colors, &colorspace, &vga_ditherer);

    vga_remapper.remap(&exocolors, *image_width)
}

pub fn vga_to_bmp(image: &[u8]) -> Vec<u8> {
    let vga_colors = vgapalette::get_vga_colors();
    let mut colors: Vec<u8> = Vec::new();

    for pixel in image {
        let color_in_palette = vga_colors[*pixel as usize];
        colors.push(color_in_palette.r);
        colors.push(color_in_palette.g);
        colors.push(color_in_palette.b);
    }

    colors
}

pub fn add_grid(image: &mut Vec<u8>, grid_width: usize, cell_size: usize) {
    let mut x_counter = 0;
    let mut y_counter = 1;

    for i in (0..image.len()).step_by(3) {
        x_counter += 1;

        if x_counter % &grid_width == 0 {
            y_counter += 1;
        }

        if x_counter % &cell_size == 0 || y_counter % &cell_size == 0 {
            image[i + 0] = 0;
            image[i + 1] = 0;
            image[i + 2] = 0;
        }
    }
}

#[test]
fn bmp_to_vga_first_green() {
    let mut input_test: [u8; 3] = [0; 3];
    input_test[0] = 128;
    input_test[1] = 128;
    input_test[2] = 128;

    let mut output_test: [u8; 1] = [0; 1];
    output_test[0] = 24;

    let output = bmp_to_vga(&input_test, &1);

    assert_eq!(&output[0..1], &output_test[0..1]);
}

#[test]
#[should_panic]
fn bmp_to_vga_first_red() {
    let mut input_test: [u8; 3] = [0; 3];
    input_test[0] = 128;
    input_test[1] = 128;
    input_test[2] = 128;

    let mut output_test: [u8; 1] = [0; 1];
    output_test[0] = 0;

    let output = bmp_to_vga(&input_test, &1);

    assert_eq!(&output[0..1], &output_test[0..1]);
}

#[test]
fn bmp_to_vga_second_green() {
    let mut input_test: [u8; 3] = [0; 3];
    input_test[0] = 0;
    input_test[1] = 0;
    input_test[2] = 0;

    let mut output_test: [u8; 1] = [0; 1];
    output_test[0] = 46;

    let output = bmp_to_vga(&input_test, &1);

    assert_eq!(&output[0..1], &output_test[0..1]);
}

#[test]
#[should_panic]
fn bmp_to_vga_second_red() {
    let mut input_test: [u8; 3] = [0; 3];
    input_test[0] = 0;
    input_test[1] = 0;
    input_test[2] = 0;

    let mut output_test: [u8; 1] = [0; 1];
    output_test[0] = 0;

    let output = bmp_to_vga(&input_test, &1);

    assert_eq!(&output[0..1], &output_test[0..1]);
}

#[test]
fn bmp_to_vga_third_green() {
    let mut input_test: [u8; 3] = [0; 3];
    input_test[0] = 228;
    input_test[1] = 216;
    input_test[2] = 212;

    let mut output_test: [u8; 1] = [0; 1];
    output_test[0] = 29;

    let output = bmp_to_vga(&input_test, &1);

    assert_eq!(&output[0..1], &output_test[0..1]);
}

#[test]
#[should_panic]
fn bmp_to_vga_third_red() {
    let mut input_test: [u8; 3] = [0; 3];
    input_test[0] = 228;
    input_test[1] = 216;
    input_test[2] = 212;

    let mut output_test: [u8; 1] = [0; 1];
    output_test[0] = 0;

    let output = bmp_to_vga(&input_test, &1);

    assert_eq!(&output[0..1], &output_test[0..1]);
}
