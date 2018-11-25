extern crate exoquant;
extern crate glob;
extern crate image;
extern crate stb_image;

#[path = "bmpto2bpp.rs"]
pub mod bmpto2bpp;
#[path = "bmptovga.rs"]
pub mod bmptovga;

use stb_image::image::LoadResult;

use std::path::PathBuf;

pub struct PreviewImage {
    width: usize,
    height: usize,
    root: PathBuf,
    image: Vec<u8>,
}

impl PreviewImage {
    pub fn new(root: &PathBuf, image: &[u8], width: usize, height: usize) -> PreviewImage {
        // Convert the image from VGA color palette to BMP
        // TODO: Must accept every type of converter
        // let img_bmp_format = bmptovga::vga_to_bmp(&image);
        PreviewImage {
            width,
            height,
            root: root.to_path_buf(),
            image: image.to_vec(),
        }
    }

    pub fn convert_from_2bpp(&mut self)
    {
        let img_bmp_format = bmpto2bpp::twopp_to_bmp(&self.image);
        self.image = img_bmp_format;
    }

    pub fn convert_from_vga(&mut self)
    {
        let img_bmp_format = bmptovga::vga_to_bmp(&self.image);
        self.image = img_bmp_format;
    }

    fn generate_image_path(&self, root: &PathBuf) -> PathBuf {
        let mut path = root.clone();
        path.set_extension("png");
        path
    }

    // Please don't look at this code, it's a mess, I know. Sorry :(
    fn add_offsets(image: &mut Vec<u8>) {
        let args: Vec<String> = std::env::args().collect();
        let mut path = PathBuf::from(&args[0]);
        path.pop();
        path.push("impostor_offsets.png");

        if !path.exists() {
            println!("Cannot find offsets image");
            return;
        }

        let offsets = match stb_image::image::load(&path) {
            LoadResult::ImageU8(data) => data,
            LoadResult::ImageF32(..) => panic!("HDR images are not supported"),
            LoadResult::Error(string) => panic!(string),
        };

        let mut origin_counter = 0;
        let mut destination_counter = 0;

        for _i in 0..65536 {
            if offsets.data[origin_counter + 3] != 0 {
                image[destination_counter + 0] = offsets.data[origin_counter + 0];
                image[destination_counter + 1] = offsets.data[origin_counter + 1];
                image[destination_counter + 2] = offsets.data[origin_counter + 2];
            }
            origin_counter += 4;
            destination_counter += 3;
        }
    }

    pub fn add_grid(&mut self, image_width: usize, tile_size: usize) {
        bmptovga::add_grid(&mut self.image, image_width, tile_size);
        // PreviewImage::add_offsets(&mut self.image);
    }

    // TODO: Must check errors
    pub fn write(&self) {
        let path = PreviewImage::generate_image_path(self, &self.root);
        image::save_buffer(
            &path,
            &self.image,
            self.width as u32,
            self.height as u32,
            image::RGB(8),
        ).unwrap();
    }
}
