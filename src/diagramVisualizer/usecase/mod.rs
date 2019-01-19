extern crate imageproc;
extern crate rusttype;
extern crate image;
extern crate rand;

use self::rand::Rng;
use std::path::Path;
use self::rusttype::{FontCollection, Scale};
use self::image::{Rgb, RgbImage};
use self::imageproc::rect::Rect;
use self::imageproc::drawing::{
    draw_cross_mut,
    draw_line_segment_mut,
    draw_hollow_rect_mut,
    draw_filled_rect_mut,
    draw_hollow_circle_mut,
    draw_filled_circle_mut,
    draw_text_mut,
};
use std::clone::Clone;

use super::parser;

#[derive(Clone)]
struct Point{
    x: u32,
    y: u32,
}

impl Point{
    fn new(x: u32, y: u32) -> Point{
        Point{
            x: x,
            y: y,
        }
    }
}

#[derive(Clone)]
pub struct PackageBox{
    name: String,
    start: Point,
    box_width: u32,
    box_height: u32,
    row: u32,
    column: u32,
    associations: u32,
}

impl PackageBox{
    fn new(name: String, start: Point, box_width: u32, box_height: u32,
                row: u32, column: u32)
            -> PackageBox{
        PackageBox{
            name: name,
            start: start,
            box_width: box_width,
            box_height: box_height,
            row: row,
            column: column,
            associations: 0,
        }
    }
}

enum Direction{
    ToRight,
    Up,
    Down,
    ToLeft,
}

fn draw_acteur(image: &mut RgbImage, position: Point) {
    draw_hollow_circle_mut(image, (position.x as i32, position.y as i32), 5,Rgb([0u8, 0u8, 0u8]));
    draw_line_segment_mut(image, (position.x as i32, position.y as i32 + 5),
                                    (position.x as i32, position.y as i32 + 20),Rgb([0u8, 0u8, 0u8]));
    draw_line_segment_mut(image, (position.x as i32 - 5, position.y as i32 + 10),
                                    (position.x as i32 + 5, position.y as i32 + 10),Rgb([0u8, 0u8, 0u8]));

}

fn draw_association_dashed(image: &mut RgbImage, association: parser::class::Relationship, class_boxes: Vec<PackageBox>){
    let mut from_box: PackageBox = PackageBox::new("".to_string(), Point::new(0,0), 0, 0, 0, 0);
    let mut to_box: PackageBox = PackageBox::new("".to_string(), Point::new(0,0), 0, 0, 0, 0);

    let num = rand::thread_rng().gen_range(0, 100);

    for class_box in class_boxes {
        if class_box.name == association.class.name { from_box = class_box.clone(); }
        if class_box.name == association.to_class.name { to_box = class_box.clone(); }
    }
    let mut from = Point::new(from_box.start.x +from_box.box_width/2,
                                from_box.start.y);
    let mut to = Point::new(to_box.start.x + to_box.box_width/2,
                                to_box.start.y + to_box.box_height);

    if from_box.start.y == to_box.start.y {
        from = Point::new(from_box.start.x +from_box.box_width/2 + num,
                                    from_box.start.y + from_box.box_height);
        to = Point::new(to_box.start.x + to_box.box_width/2 + num,
                                    to_box.start.y + to_box.box_height);

        if from_box.box_height > to_box.box_height {
            draw_dashed_line(image, Point::new(from.x, from.y),
                                    Point::new(from.x, from.y + 20));
            draw_dashed_line(image, Point::new(from.x, from.y + 20),
                                    Point::new(to.x, from.y + 20));
            draw_dashed_line(image, Point::new(to.x, from.y + 20),
                                    Point::new(to.x, to.y));
        }
        else {
            draw_dashed_line(image, Point::new(from.x, from.y),
                                    Point::new(from.x, to.y + 20));
            draw_dashed_line(image, Point::new(from.x, to.y + 20),
                                    Point::new(to.x, to.y + 20));
            draw_dashed_line(image, Point::new(to.x, to.y + 20),
                                    Point::new(to.x, to.y));
        }
        draw_arrow(image, to.clone(), Direction::Up);
    }
    else if from_box.start.y < to_box.start.y {
        from = Point::new(from_box.start.x +from_box.box_width/2 + num,
                                    from_box.start.y + from_box.box_height);
        to = Point::new(to_box.start.x + to_box.box_width/2 + num,
                                    to_box.start.y);
        draw_dashed_line(image, Point::new(from.x, from.y),
                                Point::new(from.x, from.y + num));
        draw_dashed_line(image, Point::new(from.x, from.y + num),
                                Point::new(to.x, from.y + num));
        draw_dashed_line(image, Point::new(to.x, from.y + num),
                                Point::new(to.x, to.y));
        draw_arrow(image, to.clone(), Direction::Down);
    }
    else if from_box.start.y > to_box.start.y {
        from = Point::new(from_box.start.x + from_box.box_width/2 + num,
                                    from_box.start.y);
        to = Point::new(to_box.start.x + to_box.box_width/2 + num,
                                    to_box.start.y + to_box.box_height);
        draw_dashed_line(image, Point::new(from.x, from.y),
                                Point::new(from.x, from.y - num));
        draw_dashed_line(image, Point::new(from.x, from.y - num),
                                Point::new(to.x, from.y - num));
        draw_dashed_line(image, Point::new(to.x, from.y - num),
                                Point::new(to.x, to.y));
        draw_arrow(image, to.clone(), Direction::Up);

    }
}

fn draw_dashed_line(image: &mut RgbImage, mut from: Point, to: Point){
    let counter = 0.0;
    let mut to_right = false;let mut to_left = false; let mut up = false;let mut down = false;
    if from.x < to.x { to_right = true; }
    else if from.x > to.x { to_left = true; }
    else if from.y < to.y { up = true; }
    else if from.y > to.y { down = true; }

    if to_right == true {
        while from.x < to.x {
            draw_line_segment_mut(image, (from.x as f32, from.y as f32),
                                    (from.x as f32 +10.0, from.y as f32), Rgb([0u8, 0u8, 0u8]));
            from.x = from.x + 20;
        }
    }
    else if to_left == true {
        while from.x > to.x {
            draw_line_segment_mut(image, (from.x as f32, from.y as f32),
                                    (from.x as f32 - 10.0, from.y as f32), Rgb([0u8, 0u8, 0u8]));
            from.x = from.x - 20;
        }
    }
    else if up == true {
        while from.y < to.y {
            draw_line_segment_mut(image, (from.x as f32, from.y as f32),
                                    (from.x as f32, from.y as f32 + 10.0), Rgb([0u8, 0u8, 0u8]));
            from.y = from.y + 20;
        }
    }
    else if down == true {
        while from.y > to.y {
            draw_line_segment_mut(image, (from.x as f32, from.y as f32),
                                    (from.x as f32, from.y as f32 - 10.0), Rgb([0u8, 0u8, 0u8]));
            from.y = from.y - 20;
        }
    }
}

fn draw_arrow(image: &mut RgbImage, point: Point, direction: Direction){
    match direction {
        Direction::Up => {
        draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                (point.x as f32 - 10.0, point.y as f32 + 15.0), Rgb([0u8, 0u8, 0u8]));
        draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                (point.x as f32 + 10.0, point.y as f32 + 15.0), Rgb([0u8, 0u8, 0u8]));
                            },
        Direction::Down => {
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 + 10.0, point.y as f32 - 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 - 10.0, point.y as f32 - 15.0), Rgb([0u8, 0u8, 0u8]));
                                },
        Direction::ToLeft => {

        },
        Direction::ToRight =>{

        },
    }
}

pub fn draw_package_box(image: &mut RgbImage, package: parser::package::Package, x: &mut i32, y: &mut i32,
                    row: u32, column: u32) -> PackageBox
{
    //Used RGBs
    let white = Rgb([255u8, 255u8, 255u8]);
    let black = Rgb([0u8, 0u8, 0u8]);

    //Configuring the font
    let font_data = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font_data).unwrap().into_font().unwrap();

    let mut counter = 1;

    let size = 16.0;
    let scale = Scale { x: size, y: size };
    let box_height = size as u32;
    let mut max_characters = 0;
    let package_box: PackageBox;

    max_characters = package.name.len();

    if column == 0 { *x = 50; }

    if row > 0 && column == 0 { *y = *y + 300; }

    draw_text_mut(image, black, *x as u32 + 5, *y as u32 + 2, scale, &font, &package.name);

    //Generate the box

    let nametag_box_width = (size as u32 -6)*(max_characters as u32);
    let box_width = (size as u32 -6)*(max_characters as u32)+50;

    draw_hollow_rect_mut(image, Rect::at(*x,*y).of_size(nametag_box_width, size as u32 + 5), black);

    draw_hollow_rect_mut(image, Rect::at(*x,*y + size as i32 +5).of_size(box_width, box_height), black);

    package_box = PackageBox::new(package.name, Point::new(*x as u32,*y as u32), box_width, box_height, row, column);
    *x = box_width as i32 + *x + 50;
    //draw_hollow_rect_mut(&mut image, Rect::at(x, y).of_size(box_width, box_height), black);
    return package_box;
}
