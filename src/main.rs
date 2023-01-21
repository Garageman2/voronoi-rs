extern crate image;
extern crate gif;
extern crate core;
use std::collections::VecDeque;
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

fn draw_voronoi(x:u32,y:u32,col:[u8;3],img: &mut RgbImage, l:u32,w:u32, points: &mut VecDeque<((u32,u32),[u8;3])>)
{
    println!("{},{}",x,y);
    if img.get_pixel(x,y) == &Rgb([0,0,0])
    {
        img.put_pixel(x,y,Rgb(col));

        //111
        //1x1
        //111

        if ((x as i32) -1 >= 0) && ((y as i32) -1 >= 0)
        {
            //-1,-1
            points.push_back(((x-1,y-1),col));
        }
        if (y as i32) -1 >= 0
        {
            //0,-1
            points.push_back(((x,y-1),col));
        }
        if (x+1 < w) && ((y as i32) -1 > 0)
        {
            //+1,-1
            points.push_back(((x+1,y-1),col));
        }
        if y+1 < l
        {
            //0,+1
            points.push_back(((x,y+1),col));
        }
        if x+1 < w
        {
            //1,0
            points.push_back(((x+1,y),col));
        }
        if (x+1 < w) && (y+1 < l)
        {
            //1,1
            points.push_back(((x+1,y+1),col));
        }
        if (x as i32) -1  >= 0
        {
            //-1,0
            points.push_back(((x-1,y),col));
        }
        if ((x as i32) -1 >= 0) && (y+1 < l)
        {
            //-1,1
            points.push_back(((x-1,y+1),col));
        }

    }
}

fn main() {
    const HEIGHT: u32 = 256;
    const WIDTH: u32 = 256;
    let mut img:RgbImage = RgbImage::new(WIDTH,HEIGHT);
    // draw_line(&mut img, (0,0),(10,10),[0,0,255]);
    // draw_line(&mut img, (10,10),(20,30),[0,0,255]);
    // draw_line(&mut img, (0,0),(0,40),[0,0,255]);
    // draw_line(&mut img, (0,0),(40,0),[0,0,255]);

    //submit a point and a color for the point suggested.
    let mut points:VecDeque<((u32,u32),[u8;3])> = VecDeque::new();
    points.push_front(((1,1),[0,0,255]));
    points.push_front(((128,128),[0,255,0]));
    points.push_front(((255,255),[100,100,0]));
    points.push_front(((40,100),[0,0,100]));
    points.push_front(((170,90),[160,0,120]));
    points.push_front(((80,210),[90,10,200]));
    while points.len() > 0
    {
        let point = points.pop_front().unwrap();
        draw_voronoi(point.0.0,point.0.1,point.1, &mut img, HEIGHT, WIDTH, &mut points);
    }

    img.save("images/output.png");
}
