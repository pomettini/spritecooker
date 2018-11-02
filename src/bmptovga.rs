extern crate exoquant;
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
