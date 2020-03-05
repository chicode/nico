use image::{
    png::{PNGEncoder, PngDecoder},
    ColorType, ImageDecoder, ImageEncoder, RgbaImage,
};
use serde::{de::Error as _, ser::Error as _};

pub fn serialize<S>(img: &image::RgbaImage, ser: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut buf = Vec::with_capacity(img.len());
    PNGEncoder::new(&mut buf)
        .write_image(img, img.width(), img.height(), ColorType::Rgba8)
        .map_err(S::Error::custom)?;
    ser.serialize_bytes(&buf)
}

pub fn deserialize<'de, D>(de: D) -> Result<RgbaImage, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let b: &[u8] = serde::Deserialize::deserialize(de)?;
    let png = PngDecoder::new(b).map_err(D::Error::custom)?;
    let (width, height) = png.dimensions();
    let mut buf = Vec::with_capacity(png.total_bytes() as usize);
    png.read_image(&mut buf).map_err(D::Error::custom)?;
    RgbaImage::from_vec(width, height, buf).ok_or_else(|| unreachable!())
}
