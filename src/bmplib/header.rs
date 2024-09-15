pub use byteorder::BigEndian;
pub use byteorder::ByteOrder;
pub use byteorder::LittleEndian;

#[derive(Debug)]
pub enum Compression {
    BI_RGB,
    BI_RLE8,
    BI_RLE4,
    BI_BITFIELDS,
    BI_JPEG,
    BI_PNG,
    BI_ALPHABITFIELDS,
}

impl Compression {
    fn from_u32(val: u32) -> Compression {
        match val {
            1 => Compression::BI_RLE8,
            2 => Compression::BI_RLE4,
            3 => Compression::BI_BITFIELDS,
            4 => Compression::BI_JPEG,
            5 => Compression::BI_PNG,
            6 => Compression::BI_ALPHABITFIELDS,
            _ => Compression::BI_RGB,
        }
    }

    fn to_u32(self) -> u32 {
        match self {
            Compression::BI_RGB => 0,
            Compression::BI_RLE8 => 1,
            Compression::BI_RLE4 => 2,
            Compression::BI_BITFIELDS => 3,
            Compression::BI_JPEG => 4,
            Compression::BI_PNG => 5,
            Compression::BI_ALPHABITFIELDS => 6,
        }
    }
}

#[derive(Debug)]
pub struct bmpheader {
    magic: [u8; 2],
    pub file_size: u32,
    pub reserved_1: u16,
    pub reserved_2: u16,
    pub offset: u32,
}

impl bmpheader {
    pub fn new(data_size: u32) -> Self {
        bmpheader {
            magic: BM_MAGIC.to_owned(),
            file_size: BMP_HEADER_SIZE + DIB_HEADER_SIZE + data_size,
            reserved_1: 0,
            reserved_2: 0,
            offset: BMP_HEADER_SIZE + DIB_HEADER_SIZE,
        }
    }
    pub fn to_bytes(&self) -> [u8; BMP_HEADER_SIZE as usize] {
        let mut header: [u8; BMP_HEADER_SIZE as usize] = [0; BMP_HEADER_SIZE as usize];

        for i in 0..2 {
            header[i] = self.magic[i];
        }

        LittleEndian::write_u32(&mut header[2..6], self.file_size);
        LittleEndian::write_u16(&mut header[6..8], self.reserved_1);
        LittleEndian::write_u16(&mut header[8..10], self.reserved_2);
        LittleEndian::write_u32(&mut header[10..14], self.offset);

        header
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        bmpheader {
            magic: BM_MAGIC.to_owned(),
            file_size: LittleEndian::read_u32(&bytes[2..6]),
            reserved_1: LittleEndian::read_u16(&bytes[6..8]),
            reserved_2: LittleEndian::read_u16(&bytes[8..10]),
            offset: LittleEndian::read_u32(&bytes[10..14]),
        }
    }
}

#[derive(Debug)]
pub struct dibheader {
    header_size: u32,
    // left to right
    pub width: u32,
    // bottom to top
    pub height: u32,
    // always 1
    pub planes: u16,
    // 1 4 8 16 24 32
    pub bits_per_px: u16,
    pub compression: u32,
    pub raw_bmp_data_size: u32,
    pub n_px_per_meter_h: u32,
    pub n_px_per_meter_v: u32,
    pub n_color_palatte: u32,
    pub important: u32,
}

impl dibheader {
    pub fn new(width: u32, height: u32, bits_per_px: u16, data_size: u32) -> Self {
        dibheader {
            header_size: DIB_HEADER_SIZE,
            width: width,
            height: height,
            planes: 1,
            bits_per_px: bits_per_px,
            compression: Compression::BI_RGB.to_u32(),
            raw_bmp_data_size: data_size,
            n_px_per_meter_h: 0,
            n_px_per_meter_v: 0,
            n_color_palatte: 0,
            important: 0,
        }
    }

    pub fn to_bytes(&self) -> [u8; DIB_HEADER_SIZE as usize] {
        let mut header: [u8; DIB_HEADER_SIZE as usize] = [0; DIB_HEADER_SIZE as usize];

        LittleEndian::write_u32(&mut header[0..4], self.header_size);
        LittleEndian::write_u32(&mut header[4..8], self.width);
        LittleEndian::write_u32(&mut header[8..12], self.height);
        LittleEndian::write_u16(&mut header[12..14], self.planes);
        LittleEndian::write_u16(&mut header[14..16], self.bits_per_px);
        LittleEndian::write_u32(&mut header[16..20], self.compression);
        LittleEndian::write_u32(&mut header[20..24], self.raw_bmp_data_size);
        LittleEndian::write_u32(&mut header[24..28], self.n_px_per_meter_h);
        LittleEndian::write_u32(&mut header[28..32], self.n_px_per_meter_v);

        header
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        dibheader {
            header_size: DIB_HEADER_SIZE,
            width: LittleEndian::read_u32(&bytes[4..8]),
            height: LittleEndian::read_u32(&bytes[8..12]),
            planes: LittleEndian::read_u16(&bytes[12..14]),
            bits_per_px: LittleEndian::read_u16(&bytes[14..16]),
            compression: LittleEndian::read_u32(&bytes[16..20]),
            raw_bmp_data_size: LittleEndian::read_u32(&bytes[20..24]),
            n_px_per_meter_h: LittleEndian::read_u32(&bytes[24..28]),
            n_px_per_meter_v: LittleEndian::read_u32(&bytes[28..32]),
            n_color_palatte: LittleEndian::read_u32(&bytes[32..36]),
            important: LittleEndian::read_u32(&bytes[36..40]),
        }
    }
}

pub(crate) const BM_MAGIC: &[u8; 2] = b"BM";

pub(crate) const BMP_HEADER_SIZE: u32 = 14;

pub(crate) const DIB_HEADER_SIZE: u32 = 124;
