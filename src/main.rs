mod bmplib;

use std::io::{BufReader, BufWriter, Write};
use std::{fs::File, ops::Deref};

use bmplib::decoder::Decoder;
use bmplib::encoder::Encoder;
use image::{Pixel, Pixels as IMGPixels, Rgb, RgbImage, Rgba, RgbaImage};

fn main() {
    test_encode();
    test_decoder();
}

fn test_encode() {
    let img = image::open("img.png").unwrap();

    let img_vec: Vec<[u8; 3]> = img
        .to_rgb8()
        .pixels()
        .map(|p| [p.channels()[0], p.channels()[1], p.channels()[2]])
        .collect();

    let img_w = img.width();
    let img_h = img.height();

    let mut encoder = Encoder::new(img_vec.as_slice(), img_w, img_h, 24);

    let mut op_file = File::create("img_op.bmp").unwrap();
    let mut buffer = BufWriter::new(op_file);

    // start encoding
    encoder.encode_to_buffer(&mut buffer);
}

fn test_decoder() {
    let mut op_file = File::open("img_op.bmp").unwrap();
    let mut decoder = Decoder::new(op_file);

    // start decoding
    let (pxs, header) = decoder.decode().unwrap();
    let img_pxs: Vec<Rgb<u8>> = pxs.iter().map(|p| Rgb(p.to_rgb())).collect();

    let mut op_img = RgbImage::new(header.width, header.height);
    for i in 0..op_img.width() {
        for j in 0..op_img.height() {
            op_img.put_pixel(i, j, img_pxs[(j * op_img.width() + i) as usize]);
        }
    }
    op_img.save("img_op.png").unwrap();
}
