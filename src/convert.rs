use std::fs::File;

pub fn convert_png(path: &str, data: &Vec<u8>, width: u32, height: u32)
{
    File::create(path).unwrap();
    image::save_buffer_with_format(path, data, width, height, image::ColorType::Rgb8, image::ImageFormat::Png).unwrap();
}

pub fn convert_jpg(path: &str, data: &Vec<u8>, width: u32, height: u32)
{
    File::create(path).unwrap();
    image::save_buffer_with_format(path, data, width, height, image::ColorType::Rgb8, image::ImageFormat::Jpeg).unwrap();
}
