use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, Clamped, JsCast};

use crate::util;

#[derive(Serialize, Deserialize)]
#[serde(remote = "image::RgbaImage")]
struct ImageDef {
    #[serde(getter = "image::RgbaImage::width")]
    width: u32,
    #[serde(getter = "image::RgbaImage::height")]
    height: u32,
    #[serde(getter = "_buf")]
    buf: Vec<u8>,
}
fn _buf(im: &image::RgbaImage) -> Vec<u8> {
    im.clone().into_vec()
}
impl From<ImageDef> for image::RgbaImage {
    fn from(img: ImageDef) -> Self {
        image::RgbaImage::from_raw(img.width, img.height, img.buf).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Image(#[serde(with = "ImageDef")] image::RgbaImage);
impl Image {
    pub fn load_png(b: &[u8]) -> image::ImageResult<Self> {
        Ok(Image(
            image::load_from_memory_with_format(b, image::ImageFormat::Png)?.into_rgba(),
        ))
    }

    pub fn to_img_data(&self) -> web_sys::ImageData {
        // work around web-sys's version requiring an &mut [u8] for no reason
        #[wasm_bindgen]
        extern "C" {
            type ImageData;
            #[wasm_bindgen(constructor, catch)]
            fn new(b: Clamped<&[u8]>, w: u32, h: u32) -> Result<ImageData, JsValue>;
        }
        let (w, h) = self.0.dimensions();
        ImageData::new(Clamped(&self.0), w, h)
            .unwrap()
            .unchecked_into()
    }

    pub fn from_img_data(img_data: &web_sys::ImageData) -> Self {
        let w = img_data.width();
        let h = img_data.height();
        let Clamped(data) = img_data.data();
        Image(image::RgbaImage::from_raw(w, h, data).unwrap())
    }

    /// Create an image from a [CanvasImageSource] value or an ImageData
    ///
    /// [CanvasImageSource]: https://developer.mozilla.org/en-US/docs/Web/API/CanvasImageSource
    pub fn from_js(val: JsValue) -> Result<Self, JsValue> {
        let img_data = val.dyn_into().or_else(|val| {
            let w = js_sys::Reflect::get(&val, &"width".into())?
                .as_f64()
                .ok_or_else(|| js_sys::TypeError::new("width isn't a number"))?;
            let h = js_sys::Reflect::get(&val, &"height".into())?
                .as_f64()
                .ok_or_else(|| js_sys::TypeError::new("height isn't a number"))?;

            let ctx = util::create_ctx(w as u32, h as u32);
            // the type we cast it to doesn't matter; it's gonna get to ctx.drawImage() anyway
            ctx.draw_image_with_html_canvas_element(val.unchecked_ref(), 0.0, 0.0)?;
            ctx.get_image_data(0.0, 0.0, w, h)
        })?;

        Ok(Self::from_img_data(&img_data))
    }

    pub fn as_data(&self) -> &[u8] {
        &***self
    }
}
impl std::ops::Deref for Image {
    type Target = image::RgbaImage;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for Image {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.width() == other.width()
            && self.height() == other.height()
            && self.as_data() == other.as_data()
    }
}
impl Default for Image {
    fn default() -> Self {
        let c = u8::max_value();
        Image(image::RgbaImage::from_pixel(16, 16, [c, c, c, c].into()))
    }
}
