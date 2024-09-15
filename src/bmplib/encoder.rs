use crate::bmplib::header::{bmpheader, dibheader, BMP_HEADER_SIZE, DIB_HEADER_SIZE};
use crate::bmplib::pixel::Pixels;
use image::{EncodableLayout, Pixel};
use std::{env::current_dir, fmt::Error, io::Write};

pub struct Encoder<'a> {
    data: &'a [[u8; 3]],
    height: u32,
    width: u32,
    bpp: u16,
}

impl<'a> Encoder<'a> {
    pub fn new(data: &'a [[u8; 3]], width: u32, height: u32, bpp: u16) -> Self {
        Encoder {
            data,
            width,
            height,
            bpp,
        }
    }

    pub fn encode_to_buffer<W>(
        &self,
        buffer: &mut std::io::BufWriter<W>,
    ) -> Result<usize, std::fmt::Error>
    where
        W: Write,
    {
        // row size must be a multiple of 4 bytes
        let row_size = (((self.bpp as u32 * self.width + 31) / 32) * 4) as usize;
        let px_data_size = (row_size * self.height as usize) as u32;
        let px_data: Vec<Pixels> = self
            .data
            .iter()
            .map(|p| Pixels::from(p.to_owned()))
            .collect();

        let mut bmpheader = bmpheader::new(px_data_size);
        let mut dibheader = dibheader::new(self.width, self.height, self.bpp, px_data_size);

        // bmp header
        buffer
            .write(&bmpheader.to_bytes())
            .expect("bmp header write error");

        // dib header
        buffer
            .write(&dibheader.to_bytes())
            .expect("dib header write error");

        // px data
        // from bottom to top, so we reverse
        for row in (0..self.height).rev() {
            let row_start = (row * row_size as u32) as usize;
            let row_end = row_start + (self.width * 3) as usize;

            if dibheader.bits_per_px == 24 {
                for r in (row_start / 3..row_end / 3) {
                    buffer
                        .write_all(&px_data[r].to_rgb())
                        .expect("pixel write error");
                }
            }

            // padding
            // if row_end % 4 != 0 {
            //     let padding = 4 - (row_end % 4);
            //     buffer
            //         .write(&vec![0; padding])
            //         .expect("padding write error");
            // }
        }

        Ok(0)
    }
}
