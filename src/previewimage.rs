extern crate exoquant;
extern crate glob;
extern crate image;
extern crate stb_image;

#[path = "bmptovga.rs"]
pub mod bmptovga;

use stb_image::image::LoadResult;

use std::path::PathBuf;

pub struct PreviewImage {
    root: PathBuf,
    image: Vec<u8>,
}

impl PreviewImage {
    pub fn new(root: &PathBuf, image: &[u8]) -> PreviewImage {
        // Convert the image from VGA color palette to BMP
        let img_bmp_format = bmptovga::vga_to_bmp(&image);
        PreviewImage {
            root: root.to_path_buf(),
            image: img_bmp_format,
        }
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

        for i in 0..65536 {
            if offsets.data[origin_counter + 3] != 0 {
                image[destination_counter + 0] = offsets.data[origin_counter + 0];
                image[destination_counter + 1] = offsets.data[origin_counter + 1];
                image[destination_counter + 2] = offsets.data[origin_counter + 2];
            }
            origin_counter += 4;
            destination_counter += 3;
        }
    }

    pub fn add_grid(&mut self) {
        bmptovga::add_grid(&mut self.image);
        PreviewImage::add_offsets(&mut self.image);
    }

    // TODO: Must check errors
    pub fn write(&self) {
        let path = PreviewImage::generate_image_path(self, &self.root);
        image::save_buffer(&path, &self.image, 256, 256, image::RGB(8)).unwrap();
    }
}
