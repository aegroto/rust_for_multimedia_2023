use image::GrayImage;

fn main() {
    let image = image::io::Reader::open("assets/myownlena.png")
        .unwrap()
        .decode()
        .unwrap();

    let width = image.width();
    let height = image.height();
    let num_pixels = (width * height) as usize;

    let mut r_plane = vec![0u8; num_pixels];
    let mut g_plane = vec![0u8; num_pixels];
    let mut b_plane = vec![0u8; num_pixels];

    for (i, pixel) in image.as_rgb8().unwrap().pixels().enumerate() {
        r_plane[i] = pixel.0[0];
        g_plane[i] = pixel.0[1];
        b_plane[i] = pixel.0[2];
    }

    save_grayscale("output/r_plane.png", r_plane, width, height);
    save_grayscale("output/g_plane.png", g_plane, width, height);
    save_grayscale("output/b_plane.png", b_plane, width, height);
}

fn save_grayscale(path: &str, pixels: Vec<u8>, width: u32, height: u32) {
    let plane = GrayImage::from_raw(width, height, pixels).unwrap();
    plane.save(path).unwrap();
}
