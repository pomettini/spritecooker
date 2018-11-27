extern crate exoquant;
extern crate stb_image;

use bitmap::*;
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

pub fn bmp_to_2bpp(image: BitmapRef, image_width: usize) -> Bitmap {
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

    gb_remapper.remap(&exocolors, image_width)
}

pub fn twopp_to_bmp(image: BitmapRef) -> Bitmap {
    let gb_colors = get_gb_colors();
    let mut colors: Bitmap = Vec::new();

    for pixel in image {
        let color_in_palette = gb_colors[*pixel as usize];
        colors.push(color_in_palette.r);
        colors.push(color_in_palette.g);
        colors.push(color_in_palette.b);
    }

    colors
}
