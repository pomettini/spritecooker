extern crate exoquant;
extern crate font8x8;
extern crate stb_image;

#[path = "vgapalette.rs"]
pub mod vgapalette;

use exoquant::*;

pub fn bmp_to_vga(image: &[u8]) -> Vec<u8> {
    let vga_colors = vgapalette::get_vga_colors();
    let mut exocolors: Vec<Color> = Vec::new();

    for i in 0..65536 {
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

    vga_remapper.remap(&exocolors, 256)
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

pub fn add_grid(image: &mut Vec<u8>) {
    let mut x_counter = 0;
    let mut y_counter = 1;

    for i in (0..image.len()).step_by(3) {
        x_counter += 1;

        if x_counter % 256 == 0 {
            y_counter += 1;
        }

        if x_counter % 16 == 0 || y_counter % 16 == 0 {
            image[i + 0] = 0;
            image[i + 1] = 0;
            image[i + 2] = 0;
        }
    }
}
