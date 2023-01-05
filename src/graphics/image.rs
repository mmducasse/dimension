use macroquad::{texture::Image, prelude::BLUE};
use xf::{
    gl::{bitmap::Bitmap, color::Color}, 
    num::{ivec2::{i2, IVec2}, irect::IRect}
};

/// A wrapper for Macroquad's Image struct
/// that implements xf's Bitmap trait.
pub struct ImageW(Option<Image>);

impl ImageW {
    pub const fn new() -> Self {
        Self(None)
    }

    pub fn init(&mut self, screen_size: IVec2) {
        self.0 = Some(
            Image::gen_image_color(screen_size.x as u16, screen_size.y as u16, BLUE)
        );
    }

    pub fn image<'a>(&'a self) -> &'a Image {
        if let Some(image) = &self.0 {
            image
        } else {
            panic!()
        }
    }
}

/// A trait to abstract over any 2D array of pixels.
impl Bitmap for ImageW {
    fn size(&self) -> IVec2 {
        let image = self.image();
        let w = image.width as i32;
        let h = image.height as i32;
        i2(w, h)
    }

    fn get_pixel(&self, _: IVec2) -> Color {
        todo!()
    }

    fn set_pixel(&mut self, pos: IVec2, color: Color) {
        if IRect::of_size(self.size()).contains(pos) {
            let color = macroquad::color::Color::from_rgba(
                color.r, color.g, color.b, color.a);
        
            let x = pos.x as u32;
            let y = pos.y as u32;
            if let Some(image) = &mut self.0 {
                image.set_pixel(x, y, color);
            }
        }
    }
}