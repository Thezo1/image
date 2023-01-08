mod ppm;
mod png;
mod convert;

fn main()
{
    let image = ppm::parse_from_file("test_images/output.ppm").unwrap();
    convert::convert_png("test_images/image.png", &image, 640, 426);
    convert::convert_jpg("test_images/image.jpg", &image, 640, 426);
}
