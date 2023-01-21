extern crate image;
extern crate gif;
extern crate core;

use image::{Rgb, RgbImage, Pixel};
use gif::{Encoder};

fn draw_line(img: &mut RgbImage,p1:(u32,u32),p2:(u32,u32),col:[u8;3])
{
    let RCol = Rgb(col);
    if p2.0 == p1.0
    {
        println!("strike");
        //handles a vertical
        for y in p1.1..p2.1
        {
            let x = p2.0;
            img.put_pixel(x,y,RCol);
        }
    }
    else
    {
        let slope:f32 = (p2.1-p1.1) as f32 / (p2.0-p1.0) as f32;
        let intercept:f32 = (p2.1 as f32) - (p2.0 as f32 * slope);
        for x in p1.0..p2.0
        {
            let y = ((slope * x as f32).floor() + intercept )as u32;
            img.put_pixel(x,y,RCol);
        }

    }




}

fn main() {
    const HEIGHT: u32 = 64;
    const WIDTH: u32 = 64;
    let mut img:RgbImage = RgbImage::new(WIDTH,HEIGHT);
    draw_line(&mut img, (0,0),(10,10),[0,0,255]);
    draw_line(&mut img, (10,10),(20,30),[0,0,255]);
    draw_line(&mut img, (0,0),(0,40),[0,0,255]);
    draw_line(&mut img, (0,0),(40,0),[0,0,255]);
    img.save("images/output.png");
}
