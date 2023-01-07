extern crate image;
extern crate gif;
extern crate core;

use image::{Rgb, RgbImage, Pixel};
use gif::{Encoder};

fn draw_line(img: &mut RgbImage,p1:(u32,u32),p2:(u32,u32))
{
    let slope:f32 = (p2.1-p1.1) as f32 / (p2.0-p1.0) as f32;
    let intercept:f32 = (p2.1 as f32) - (p2.0 as f32 * slope);
    assert_eq!((p1.0 as f32 * slope) + intercept,p1.1 as f32);
    for x in p1.0..p2.0
    {
        let y = ((slope * x as f32) + intercept )as u32;
        println!("Write to {},{}",x,y);
        img.put_pixel(x,y,Pixel::from_channels(255,0,0,0));
    }


}

fn main() {
    const HEIGHT: u32 = 64;
    const WIDTH: u32 = 64;
    let mut img:RgbImage = RgbImage::new(WIDTH,HEIGHT);
    draw_line(&mut img, (0,0),(10,10));
    draw_line(&mut img, (10,10),(20,30));
    img.save("images/output.png");
}
