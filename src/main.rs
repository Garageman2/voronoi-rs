extern crate image;
use std::collections::VecDeque;
use image::{Rgb, RgbImage};
use rand::Rng;


fn draw_line(img: &mut RgbImage,p1:(u32,u32),p2:(u32,u32),col:[u8;3])
{
    let new_col = Rgb(col);
    if p2.0 == p1.0
    {
        //handles a vertical
        for y in p1.1..p2.1
        {
            let x = p2.0;
            img.put_pixel(x,y,new_col);
        }
    }
    else
    {
        let slope:f32 = (p2.1-p1.1) as f32 / (p2.0-p1.0) as f32;
        let intercept:f32 = (p2.1 as f32) - (p2.0 as f32 * slope);
        for x in p1.0..p2.0
        {
            let y = ((slope * x as f32).floor() + intercept )as u32;
            img.put_pixel(x,y,new_col);
        }

    }




}

fn draw_voronoi2(x:u32,y:u32,col:[u8;3],img: &mut RgbImage, l:u32,w:u32, points: &mut VecDeque<((u32,u32),[u8;3])>)
{
    ///DIFFERENT METHOD FOR FILLING
    if img.get_pixel(x,y) == &Rgb([0,0,0])
    {
        img.put_pixel(x,y,Rgb(col));

        //111
        //1x1
        //111

        if (y as i32) -1 >= 0
        {
            //0,-1
            points.push_back(((x,y-1),col));
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
        if (x as i32) -1  >= 0
        {
            //-1,0
            points.push_back(((x-1,y),col));
        }

    }
}

fn draw_voronoi(x:u32,y:u32,col:[u8;3],img: &mut RgbImage, l:u32,w:u32, points: &mut VecDeque<((u32,u32),[u8;3])>)
{
    if img.get_pixel(x,y) == &Rgb([0,0,0])
    {
        img.put_pixel(x,y,Rgb(col));

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

fn draw_voronoi_with_lines(x:u32,y:u32,col:[u8;3],img: &mut RgbImage, l:u32,w:u32, points: &mut VecDeque<((u32,u32),[u8;3])>)
{
    let fcol = img.get_pixel(x,y);
    if fcol == &Rgb([0,0,0])
    {
        img.put_pixel(x,y,Rgb(col));

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
    else if  fcol != &Rgb([255,255,255]) && fcol !=  &Rgb(col)
    {
        img.put_pixel(x,y,Rgb([255,255,255]));
    }
}

fn draw_bands(img: &mut RgbImage, l:u32,w:u32, points: &mut VecDeque<((u32,u32),[u8;3])>)
{
    for y in 0..l
    {
        for x in 0..w
        {
            let mut Nearest:(f32,[u8;3]) = (1000000.0,[255,255,255]);
            for Seed in points.iter()
            {
                let dist: f32 = ((y as f32 - Seed.0.0 as f32).powf(2.0) + (x as f32 - Seed.0.0 as f32).powf(2.0)).sqrt();
                if dist < Nearest.0
                {
                    Nearest = (dist,Seed.1);
                }
            }
            img.put_pixel(x,y,Rgb(Nearest.1));


        }
    }
}

fn draw_cells(img: &mut RgbImage, l:u32,w:u32, points: &mut VecDeque<((u32,u32),[u8;3])>, lines:bool)
{
    for y in 0..l
    {
        for x in 0..w
        {
            let mut Nearest:(f32,[u8;3]) = (1000000.0,[255,255,255]);
            for Seed in points.iter()
            {
                let mut dist:f32 = ( (y as f32 - Seed.0.1 as f32).powi(2) + (x as f32 - Seed.0.0 as f32).powi(2)).sqrt();
                if (Nearest.0.floor() - dist.floor()).abs() <= 0.75 && lines
                {
                    Nearest = (dist,[255,255,255]);
                }
                else if dist < Nearest.0
                {
                    Nearest = (dist,Seed.1);
                }
            }
            img.put_pixel(x,y,Rgb(Nearest.1));


        }
    }
}


fn gen_seeds(count:u16,height:u32,width:u32,points:&mut VecDeque<((u32,u32),[u8;3])>)
{
    for _i in 0..count
    {
        let mut rng = rand::thread_rng();
        points.push_back(((rng.gen_range(0..width),rng.gen_range(0..height)),[rng.gen_range(0..255),rng.gen_range(0..255),rng.gen_range(0..255)]));
    }
}

fn main() {
    const HEIGHT: u32 = 2160;
    const WIDTH: u32 = 3840;
    const SEEDS:u16 = 128;
    let mut img:RgbImage = RgbImage::new(WIDTH,HEIGHT);

    //submit a point and a color for the point suggested.
    let mut points:VecDeque<((u32,u32),[u8;3])> = VecDeque::new();
    gen_seeds(SEEDS,HEIGHT,WIDTH,&mut points);
    draw_bands(&mut img, HEIGHT, WIDTH, &mut points);
    // while points.len() > 0
    // {
    //     let point = points.pop_front().unwrap();
    //     draw_voronoi_with_lines(point.0.0,point.0.1,point.1, &mut img, HEIGHT, WIDTH, &mut points);
    // }

    img.save("images/Demo4.png").expect("failed to save");

}
