use byteorder::ByteOrder;
use byteorder::LittleEndian;
use std::io::Read;

use super::header::bmpheader;
use super::header::dibheader;
use super::header::{BMP_HEADER_SIZE, DIB_HEADER_SIZE};
use super::Pixels;

pub struct Decoder<R> {
    reader: R,
}

impl<R> Decoder<R>
where
    R: Read,
{
    pub fn new(reader: R) -> Self {
        Decoder { reader }
    }

    pub fn decode(&mut self) -> Result<(Vec<Pixels>, dibheader), std::fmt::Error> {
        let mut bmphbuffer: [u8; BMP_HEADER_SIZE as usize] = [0; BMP_HEADER_SIZE as usize];
        self.reader.read_exact(&mut bmphbuffer).unwrap();
        let mut bmpheader = bmpheader::from_bytes(&mut bmphbuffer);

        let mut dibhbuffer: [u8; DIB_HEADER_SIZE as usize] = [0; DIB_HEADER_SIZE as usize];
        self.reader.read_exact(&mut dibhbuffer).unwrap();
        let mut dibheader = dibheader::from_bytes(&mut dibhbuffer);

        // println!("{:?} {:?}", bmpheader, dibheader);

        let width = dibheader.width;
        let height = dibheader.height;

        // let mut pxs: Vec<Pixels> = Vec::with_capacity((width * height) as usize);
        let mut pxs: Vec<Pixels> = Vec::new();

        for h in 0..height {
            let mut temp = Vec::new();
            for w in 0..width {
                if dibheader.bits_per_px == 24 {
                    let mut tempbuf: [u8; 3] = [0; 3];
                    self.reader.read_exact(&mut tempbuf).unwrap();
                    let mut temppx = Pixels::from(tempbuf);
                    temp.push(temppx);
                }
            }
            temp.append(&mut pxs);
            pxs = temp;
        }

        Ok((pxs, dibheader))
    }
}
