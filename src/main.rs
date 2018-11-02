extern crate stb_image;
extern crate exoquant;

extern crate glob;

use std::fs::File;
use std::io::prelude::*;

use stb_image::image;
use stb_image::image::LoadResult;
use stb_image::image::LoadResult::*;

use glob::glob;
use std::path::PathBuf;

use exoquant::*;

fn main() 
{
    let mut colors: [exoquant::Color; 256] = [exoquant::Color::new(0, 0, 0, 0); 256];

    /* First row */
    colors[0] = exoquant::Color::new(0, 0, 0, 1);
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
    colors[112] = exoquant::Color::new(113, 0, 0, 0);
    colors[113] = exoquant::Color::new(113, 28, 0, 0);
    colors[114] = exoquant::Color::new(113, 57, 0, 0);
    colors[115] = exoquant::Color::new(113, 85, 0, 0);
    colors[116] = exoquant::Color::new(113, 113, 0, 0);
    colors[117] = exoquant::Color::new(85, 113, 0, 0);
    colors[118] = exoquant::Color::new(57, 113, 0, 0);
    colors[119] = exoquant::Color::new(28, 113, 0, 0);
    colors[120] = exoquant::Color::new(0, 113, 0, 0);
    colors[121] = exoquant::Color::new(0, 113, 28, 0);
    colors[122] = exoquant::Color::new(0, 113, 57, 0);
    colors[123] = exoquant::Color::new(0, 113, 85, 0);
    colors[124] = exoquant::Color::new(0, 113, 113, 0);
    colors[125] = exoquant::Color::new(0, 85, 113, 0);
    colors[126] = exoquant::Color::new(0, 57, 113, 0);
    colors[127] = exoquant::Color::new(0, 28, 113, 0);

    /* Ninth row */
    colors[128] = exoquant::Color::new(57, 57, 113, 0);
    colors[129] = exoquant::Color::new(69, 57, 113, 0);
    colors[130] = exoquant::Color::new(85, 57, 113, 0);
    colors[131] = exoquant::Color::new(97, 57, 113, 0);
    colors[132] = exoquant::Color::new(113, 57, 113, 0);
    colors[133] = exoquant::Color::new(113, 57, 97, 0);
    colors[134] = exoquant::Color::new(113, 57, 85, 0);
    colors[135] = exoquant::Color::new(113, 57, 69, 0);
    colors[136] = exoquant::Color::new(113, 57, 57, 0);
    colors[137] = exoquant::Color::new(113, 69, 57, 0);
    colors[138] = exoquant::Color::new(113, 85, 57, 0);
    colors[139] = exoquant::Color::new(113, 97, 57, 0);
    colors[140] = exoquant::Color::new(113, 113, 57, 0);
    colors[141] = exoquant::Color::new(97, 113, 57, 0);
    colors[142] = exoquant::Color::new(85, 113, 57, 0);
    colors[143] = exoquant::Color::new(69, 113, 57, 0);

    /* Tenth row */
    colors[144] = exoquant::Color::new(57, 113, 57, 0);
    colors[145] = exoquant::Color::new(57, 113, 69, 0);
    colors[146] = exoquant::Color::new(57, 113, 85, 0);
    colors[147] = exoquant::Color::new(57, 113, 97, 0);
    colors[148] = exoquant::Color::new(57, 113, 113, 0);
    colors[149] = exoquant::Color::new(57, 97, 113, 0);
    colors[150] = exoquant::Color::new(57, 85, 113, 0);
    colors[151] = exoquant::Color::new(57, 69, 113, 0);
    colors[152] = exoquant::Color::new(81, 81, 113, 0);
    colors[153] = exoquant::Color::new(89, 81, 113, 0);
    colors[154] = exoquant::Color::new(97, 81, 113, 0);
    colors[155] = exoquant::Color::new(105, 81, 113, 0);
    colors[156] = exoquant::Color::new(113, 81, 113, 0);
    colors[157] = exoquant::Color::new(113, 81, 105, 0);
    colors[158] = exoquant::Color::new(113, 81, 97, 0);
    colors[159] = exoquant::Color::new(113, 81, 89, 0);

    /* Eleventh row */
    colors[160] = exoquant::Color::new(113, 81, 81, 0);
    colors[161] = exoquant::Color::new(113, 89, 81, 0);
    colors[162] = exoquant::Color::new(113, 97, 81, 0);
    colors[163] = exoquant::Color::new(113, 105, 81, 0);
    colors[164] = exoquant::Color::new(113, 113, 81, 0);
    colors[165] = exoquant::Color::new(105, 113, 81, 0);
    colors[166] = exoquant::Color::new(97, 113, 81, 0);
    colors[167] = exoquant::Color::new(89, 113, 81, 0);
    colors[168] = exoquant::Color::new(81, 113, 81, 0);
    colors[169] = exoquant::Color::new(81, 113, 89, 0);
    colors[170] = exoquant::Color::new(81, 113, 97, 0);
    colors[171] = exoquant::Color::new(81, 113, 105, 0);
    colors[172] = exoquant::Color::new(81, 113, 113, 0);
    colors[173] = exoquant::Color::new(81, 105, 113, 0);
    colors[174] = exoquant::Color::new(81, 97, 113, 0);
    colors[175] = exoquant::Color::new(81, 89, 113, 0);

    /* Twelfth row */
    colors[176] = exoquant::Color::new(0, 0, 65, 0);
    colors[177] = exoquant::Color::new(16, 0, 65, 0);
    colors[178] = exoquant::Color::new(32, 0, 65, 0);
    colors[179] = exoquant::Color::new(49, 0, 65, 0);
    colors[180] = exoquant::Color::new(65, 0, 65, 0);
    colors[181] = exoquant::Color::new(65, 0, 49, 0);
    colors[182] = exoquant::Color::new(65, 0, 32, 0);
    colors[183] = exoquant::Color::new(65, 0, 16, 0);
    colors[184] = exoquant::Color::new(65, 0, 0, 0);
    colors[185] = exoquant::Color::new(65, 16, 0, 0);
    colors[186] = exoquant::Color::new(65, 32, 0, 0);
    colors[187] = exoquant::Color::new(65, 49, 0, 0);
    colors[188] = exoquant::Color::new(65, 65, 0, 0);
    colors[189] = exoquant::Color::new(49, 65, 0, 0);
    colors[190] = exoquant::Color::new(32, 65, 0, 0);
    colors[191] = exoquant::Color::new(16, 65, 0, 0);

    /* Thirteenth row */
    colors[192] = exoquant::Color::new(0, 65, 0, 0);
    colors[193] = exoquant::Color::new(0, 65, 16, 0);
    colors[194] = exoquant::Color::new(0, 65, 32, 0);
    colors[195] = exoquant::Color::new(0, 65, 49, 0);
    colors[196] = exoquant::Color::new(0, 65, 65, 0);
    colors[197] = exoquant::Color::new(0, 49, 65, 0);
    colors[198] = exoquant::Color::new(0, 32, 65, 0);
    colors[199] = exoquant::Color::new(0, 16, 65, 0);
    colors[200] = exoquant::Color::new(32, 32, 65, 0);
    colors[201] = exoquant::Color::new(40, 32, 65, 0);
    colors[202] = exoquant::Color::new(49, 32, 65, 0);
    colors[203] = exoquant::Color::new(57, 32, 65, 0);
    colors[204] = exoquant::Color::new(65, 32, 65, 0);
    colors[205] = exoquant::Color::new(65, 32, 57, 0);
    colors[206] = exoquant::Color::new(65, 32, 49, 0);
    colors[207] = exoquant::Color::new(65, 32, 40, 0);

    /* Fourteenth row */
    colors[208] = exoquant::Color::new(65, 32, 32, 0);
    colors[209] = exoquant::Color::new(65, 40, 32, 0);
    colors[210] = exoquant::Color::new(65, 49, 32, 0);
    colors[211] = exoquant::Color::new(65, 57, 32, 0);
    colors[212] = exoquant::Color::new(65, 65, 32, 0);
    colors[213] = exoquant::Color::new(57, 65, 32, 0);
    colors[214] = exoquant::Color::new(49, 65, 32, 0);
    colors[215] = exoquant::Color::new(40, 65, 32, 0);
    colors[216] = exoquant::Color::new(32, 65, 32, 0);
    colors[217] = exoquant::Color::new(32, 65, 40, 0);
    colors[218] = exoquant::Color::new(32, 65, 49, 0);
    colors[219] = exoquant::Color::new(32, 65, 57, 0);
    colors[220] = exoquant::Color::new(32, 65, 65, 0);
    colors[221] = exoquant::Color::new(32, 57, 65, 0);
    colors[222] = exoquant::Color::new(32, 49, 65, 0);
    colors[223] = exoquant::Color::new(32, 40, 65, 0);

    /* Fifteenth row */
    colors[224] = exoquant::Color::new(45, 45, 65, 0);
    colors[225] = exoquant::Color::new(49, 45, 65, 0);
    colors[226] = exoquant::Color::new(53, 45, 65, 0);
    colors[227] = exoquant::Color::new(61, 45, 65, 0);
    colors[228] = exoquant::Color::new(65, 45, 65, 0);
    colors[229] = exoquant::Color::new(65, 45, 61, 0);
    colors[230] = exoquant::Color::new(65, 45, 53, 0);
    colors[231] = exoquant::Color::new(65, 45, 49, 0);
    colors[232] = exoquant::Color::new(65, 45, 45, 0);
    colors[233] = exoquant::Color::new(65, 49, 45, 0);
    colors[234] = exoquant::Color::new(65, 53, 45, 0);
    colors[235] = exoquant::Color::new(65, 61, 45, 0);
    colors[236] = exoquant::Color::new(65, 65, 45, 0);
    colors[237] = exoquant::Color::new(61, 65, 45, 0);
    colors[238] = exoquant::Color::new(53, 65, 45, 0);
    colors[239] = exoquant::Color::new(49, 65, 45, 0);

    /* Sixteenth row */
    colors[240] = exoquant::Color::new(45, 65, 45, 0);
    colors[241] = exoquant::Color::new(45, 65, 49, 0);
    colors[242] = exoquant::Color::new(45, 65, 53, 0);
    colors[243] = exoquant::Color::new(45, 65, 61, 0);
    colors[244] = exoquant::Color::new(45, 65, 65, 0);
    colors[245] = exoquant::Color::new(45, 61, 65, 0);
    colors[246] = exoquant::Color::new(45, 53, 65, 0);
    colors[247] = exoquant::Color::new(45, 49, 65, 0);
    colors[248] = exoquant::Color::new(0, 0, 0, 0);
    colors[249] = exoquant::Color::new(0, 0, 0, 0);
    colors[250] = exoquant::Color::new(0, 0, 0, 0);
    colors[251] = exoquant::Color::new(0, 0, 0, 0);
    colors[252] = exoquant::Color::new(0, 0, 0, 0);
    colors[253] = exoquant::Color::new(0, 0, 0, 0);
    colors[254] = exoquant::Color::new(0, 0, 0, 0);
    colors[255] = exoquant::Color::new(0, 0, 0, 0);

    let colormap = exoquant::ColorMap::new(&colors, &SimpleColorSpace::default());

    // Gets the current directory
    let args: Vec<String> = std::env::args().collect();
    let mut path = PathBuf::from(&args[0]);
    path.pop();

    // Gets only bmp files
    path.push("*.bmp");

    // For each bitmaps does the job
    for entry in glob(path.to_str().unwrap()).expect("Failed to read glob pattern")
    {
        match entry 
        {
            Ok(path) => 
            {
                println!("Begin processing: {:?}", &path);

                let image = match stb_image::image::load(&path)
                {
                    LoadResult::ImageU8(data) => data,
                    LoadResult::ImageF32(..) => panic!("HDR images are not supported"),
                    LoadResult::Error(string) => panic!(string)
                };

                if &image.width != &256
                {
                    println!("The image width must be exactly 256 px");
                    continue;
                }

                if &image.height != &256
                {
                    println!("The image height must be exactly 256 px");
                    continue;
                }

                let mut imgbuffer: [u8; 65536] = [0; 65536];
                let mut exocolors: Vec<exoquant::Color> = Vec::new();

                // This will get the name of the output file
                let mut output = path.clone();
                // TODO: the output file name must be the same as the input, except for the bin extension
                output.set_file_name("test.bin");

                for i in 0..65536 
                {
                    let r = image.data[i*3];
                    let g = image.data[i*3+1];
                    let b = image.data[i*3+2];

                    let mut c = Colorf::zero();
                    c.r = r as f64;
                    c.g = g as f64;
                    c.b = b as f64;

                    exocolors.push(exoquant::Color::new(r, g, b, 0));
                }

                let colorspace = SimpleColorSpace::default();
                let vga_ditherer = exoquant::ditherer::FloydSteinberg::new();
                let vga_remapper = exoquant::Remapper::new(&colors, &colorspace, &vga_ditherer);
                let indexed_image_data = vga_remapper.remap(&exocolors, 256);

                let mut file = File::create(&output).unwrap();
                file.write(&indexed_image_data).unwrap();

                println!("Done processing: {:?}", &path);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
