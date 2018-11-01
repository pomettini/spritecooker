extern crate stb_image;
extern crate exoquant;

extern crate glob;

use std::fs::File;
use std::io::prelude::*;

use stb_image::image;
use stb_image::image::LoadResult;
use stb_image::image::LoadResult::*;

use glob::glob;

use exoquant::*;

fn main() 
{
    let mut colors: [exoquant::Color; 112] = [exoquant::Color::new(0, 0, 0, 0); 112];

    /* First row */
    colors[0] = exoquant::Color::new(0, 0, 0, 0);
    colors[1] = exoquant::Color::new(0, 0, 170, 0);
    colors[2] = exoquant::Color::new(0, 170, 0, 0);
    colors[3] = exoquant::Color::new(0, 170, 170, 0);
    colors[4] = exoquant::Color::new(170, 0, 0, 0);
    colors[5] = exoquant::Color::new(170, 0, 170, 0);
    colors[6] = exoquant::Color::new(170, 85, 0, 0);
    colors[7] = exoquant::Color::new(170, 170, 170, 0);
    colors[8] = exoquant::Color::new(85, 85, 85, 0);
    colors[9] = exoquant::Color::new(85, 85, 255, 0);
    colors[10] = exoquant::Color::new(85, 255, 85, 0);
    colors[11] = exoquant::Color::new(85, 255, 255, 0);
    colors[12] = exoquant::Color::new(255, 85, 85, 0);
    colors[13] = exoquant::Color::new(255, 85, 255, 0);
    colors[14] = exoquant::Color::new(255, 255, 85, 0);
    colors[15] = exoquant::Color::new(255, 255, 255, 0);

    /* Second row */
    colors[16] = exoquant::Color::new(0, 0, 0, 0);
    colors[17] = exoquant::Color::new(16, 16, 16, 0);
    colors[18] = exoquant::Color::new(32, 32, 32, 0);
    colors[19] = exoquant::Color::new(53, 53, 53, 0);
    colors[20] = exoquant::Color::new(69, 69, 69, 0);
    colors[21] = exoquant::Color::new(85, 85, 85, 0);
    colors[22] = exoquant::Color::new(101, 101, 101, 0);
    colors[23] = exoquant::Color::new(117, 117, 117, 0);
    colors[24] = exoquant::Color::new(138, 138, 138, 0);
    colors[25] = exoquant::Color::new(154, 154, 154, 0);
    colors[26] = exoquant::Color::new(170, 170, 170, 0);
    colors[27] = exoquant::Color::new(186, 186, 186, 0);
    colors[28] = exoquant::Color::new(202, 202, 202, 0);
    colors[29] = exoquant::Color::new(223, 223, 223, 0);
    colors[30] = exoquant::Color::new(239, 239, 239, 0);
    colors[31] = exoquant::Color::new(255, 255, 255, 0);

    /* Third row */
    colors[32] = exoquant::Color::new(0, 0, 255, 0);
    colors[33] = exoquant::Color::new(65, 0, 255, 0);
    colors[34] = exoquant::Color::new(130, 0, 255, 0);
    colors[35] = exoquant::Color::new(190, 0, 255, 0);
    colors[36] = exoquant::Color::new(255, 0, 255, 0);
    colors[37] = exoquant::Color::new(255, 0, 190, 0);
    colors[38] = exoquant::Color::new(255, 0, 130, 0);
    colors[39] = exoquant::Color::new(255, 0, 65, 0);
    colors[40] = exoquant::Color::new(255, 0, 0, 0);
    colors[41] = exoquant::Color::new(255, 65, 0, 0);
    colors[42] = exoquant::Color::new(255, 130, 0, 0);
    colors[43] = exoquant::Color::new(255, 190, 0, 0);
    colors[44] = exoquant::Color::new(255, 255, 0, 0);
    colors[45] = exoquant::Color::new(190, 255, 0, 0);
    colors[45] = exoquant::Color::new(130, 255, 0, 0);
    colors[47] = exoquant::Color::new(65, 255, 0, 0);

    /* Fourth row */
    colors[48] = exoquant::Color::new(0, 255, 0, 0);
    colors[49] = exoquant::Color::new(0, 255, 65, 0);
    colors[50] = exoquant::Color::new(0, 255, 130, 0);
    colors[51] = exoquant::Color::new(0, 255, 190, 0);
    colors[52] = exoquant::Color::new(0, 255, 255, 0);
    colors[53] = exoquant::Color::new(0, 190, 255, 0);
    colors[54] = exoquant::Color::new(0, 130, 255, 0);
    colors[55] = exoquant::Color::new(0, 65, 255, 0);
    colors[56] = exoquant::Color::new(130, 130, 255, 0);
    colors[57] = exoquant::Color::new(158, 130, 255, 0);
    colors[58] = exoquant::Color::new(190, 130, 255, 0);
    colors[59] = exoquant::Color::new(223, 130, 255, 0);
    colors[60] = exoquant::Color::new(255, 130, 255, 0);
    colors[61] = exoquant::Color::new(255, 130, 223, 0);
    colors[62] = exoquant::Color::new(255, 130, 190, 0);
    colors[63] = exoquant::Color::new(255, 130, 158, 0);

    /* Fifth row */
    colors[64] = exoquant::Color::new(255, 130, 130, 0);
    colors[65] = exoquant::Color::new(255, 158, 130, 0);
    colors[66] = exoquant::Color::new(255, 190, 130, 0);
    colors[67] = exoquant::Color::new(255, 223, 130, 0);
    colors[68] = exoquant::Color::new(255, 255, 130, 0);
    colors[69] = exoquant::Color::new(223, 255, 130, 0);
    colors[70] = exoquant::Color::new(190, 255, 130, 0);
    colors[71] = exoquant::Color::new(158, 255, 130, 0);
    colors[72] = exoquant::Color::new(130, 255, 130, 0);
    colors[73] = exoquant::Color::new(130, 255, 158, 0);
    colors[74] = exoquant::Color::new(130, 255, 190, 0);
    colors[75] = exoquant::Color::new(130, 255, 223, 0);
    colors[76] = exoquant::Color::new(130, 255, 255, 0);
    colors[77] = exoquant::Color::new(130, 223, 255, 0);
    colors[78] = exoquant::Color::new(130, 190, 255, 0);
    colors[79] = exoquant::Color::new(130, 158, 255, 0);

    /* Sixth row */
    colors[80] = exoquant::Color::new(186, 186, 255, 0);
    colors[81] = exoquant::Color::new(202, 186, 255, 0);
    colors[82] = exoquant::Color::new(223, 186, 255, 0);
    colors[83] = exoquant::Color::new(239, 186, 255, 0);
    colors[84] = exoquant::Color::new(255, 186, 255, 0);
    colors[85] = exoquant::Color::new(255, 186, 239, 0);
    colors[86] = exoquant::Color::new(255, 186, 223, 0);
    colors[87] = exoquant::Color::new(255, 186, 202, 0);
    colors[88] = exoquant::Color::new(255, 186, 186, 0);
    colors[89] = exoquant::Color::new(255, 202, 186, 0);
    colors[90] = exoquant::Color::new(255, 223, 186, 0);
    colors[91] = exoquant::Color::new(255, 239, 186, 0);
    colors[92] = exoquant::Color::new(255, 255, 186, 0);
    colors[93] = exoquant::Color::new(239, 255, 186, 0);
    colors[94] = exoquant::Color::new(223, 255, 186, 0);
    colors[95] = exoquant::Color::new(202, 255, 186, 0);

    /* Seventh row */
    colors[96] = exoquant::Color::new(186, 255, 186, 0);
    colors[97] = exoquant::Color::new(186, 255, 202, 0);
    colors[98] = exoquant::Color::new(186, 255, 223, 0);
    colors[99] = exoquant::Color::new(186, 255, 239, 0);
    colors[100] = exoquant::Color::new(186, 255, 255, 0);
    colors[101] = exoquant::Color::new(186, 239, 255, 0);
    colors[102] = exoquant::Color::new(186, 223, 255, 0);
    colors[103] = exoquant::Color::new(186, 202, 255, 0);
    colors[104] = exoquant::Color::new(0, 0, 113, 0);
    colors[105] = exoquant::Color::new(28, 0, 113, 0);
    colors[106] = exoquant::Color::new(57, 0, 113, 0);
    colors[107] = exoquant::Color::new(85, 0, 113, 0);
    colors[108] = exoquant::Color::new(113, 0, 113, 0);
    colors[109] = exoquant::Color::new(113, 0, 85, 0);
    colors[110] = exoquant::Color::new(113, 0, 57, 0);
    colors[111] = exoquant::Color::new(113, 0, 28, 0);

    /* Eighth row */
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);

    /* Ninth row */
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);
    // colors[0] = exoquant::Color::new(0, 0, 0, 0);

    let colormap = exoquant::ColorMap::new(&colors, &SimpleColorSpace::default());

    for entry in glob("**/*.bmp").unwrap()
    {
        match entry 
        {
            Ok(path) => 
            {
                let mut imgbuffer: [u8; 65536] = [0; 65536];
                let mut exocolors: Vec<exoquant::Color> = Vec::new();

                let image = match stb_image::image::load(&path)
                {
                    LoadResult::ImageU8(data) => data,
                    LoadResult::ImageF32(..) => panic!("HDR images are not supported"),
                    LoadResult::Error(string) => panic!(string)
                };

                let mut output = path.clone();
                output.set_file_name("test.bin");

                // println!("{:?}", image.width);
                // println!("{:?}", image.height);
                // println!("{:?}", image.data.len());

                for i in 0..65536 
                {
                    let r = image.data[i*3];
                    let g = image.data[i*3+1];
                    let b = image.data[i*3+2];
                    // imgbuffer[i] = r + g + b;

                    let mut c = Colorf::zero();
                    c.r = r as f64;
                    c.g = g as f64;
                    c.b = b as f64;

                    exocolors.push(exoquant::Color::new(r, g, b, 0));
                    // println!("{:?}", &colormap.find_nearest(c));
                    // imgbuffer[i] = colormap.find_nearest(c) as u8;
                }

                let colorspace = SimpleColorSpace::default();
                let vga_ditherer = exoquant::ditherer::FloydSteinberg::new();
                let vga_remapper = exoquant::Remapper::new(&colors, &colorspace, &vga_ditherer);
                let indexed_image_data = vga_remapper.remap(&exocolors, 256);

                let mut file = File::create(&output).unwrap();
                file.write(&indexed_image_data).unwrap();
            }
            Err(e) => println!("{:?}", e),
        }
    }
}