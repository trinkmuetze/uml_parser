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
    draw_hollow_ellipse_mut,
    draw_text_mut,
};
use std::clone::Clone;

use super::parser;

#[derive(Clone)]
pub struct Point{
    x: u32,
    y: u32,
}

impl Point{
    pub fn new(x: u32, y: u32) -> Point{
        Point{
            x: x,
            y: y,
        }
    }
}

#[derive(Clone)]
pub struct UseCase{
    name: String,
    center: Point,
    height_radius: u32,
    width_radius: u32,
    priority: u32,
    associations: u32,
}

impl UseCase {
    fn new(name: String, center: Point, height_radius: u32, width_radius: u32, priority: u32)
            -> UseCase {
        UseCase {
            name: name,
            center: center,
            height_radius: height_radius,
            width_radius: width_radius,
            priority: priority,
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

pub fn draw_acteur(image: &mut RgbImage, position: Point) {
    //HEAD
    draw_hollow_circle_mut(image, (position.x as i32, position.y as i32), 25, Rgb([0u8, 0u8, 0u8]));
    //BODY
    draw_line_segment_mut(image, (position.x as f32, position.y as f32 + 25.0),
                                    (position.x as f32, position.y as f32 + 100.0),Rgb([0u8, 0u8, 0u8]));
    //LEFT LEG
    draw_line_segment_mut(image, (position.x as f32, position.y as f32 + 100.0),
                                    (position.x as f32 - 25.0, position.y as f32 + 125.0), Rgb([0u8, 0u8, 0u8]));
    //RIGHT LEG
    draw_line_segment_mut(image, (position.x as f32, position.y as f32 + 100.0),
                                    (position.x as f32 + 25.0, position.y as f32 + 125.0), Rgb([0u8, 0u8, 0u8]));
    //ARMS
    draw_line_segment_mut(image, (position.x as f32 - 25.0, position.y as f32 + 50.0),
                                    (position.x as f32 + 25.0, position.y as f32 + 50.0),Rgb([0u8, 0u8, 0u8]));

}

/*fn draw_association_dashed(image: &mut RgbImage, association: parser::class::Relationship, use_cases: Vec<UseCase>){
    let mut from_box: UseCase = UseCase::new("".to_string(), Point::new(0,0), 0, 0, 0);
    let mut to_box: UseCase = UseCase::new("".to_string(), Point::new(0,0), 0, 0, 0);

    let num = rand::thread_rng().gen_range(0, 100);

    for use_case in use_cases {
        if use_case.name == association.class.name { from_box = use_case.clone(); }
        if use_case.name == association.to_class.name { to_box = use_case.clone(); }
    }
    let mut from = Point::new(from_box.center.x +from_box.radius/2,
                                from_box.center.y);
    let mut to = Point::new(to_box.center.x + to_box.radius/2,
                                to_box.center.y + to_box.radius);

    if from_box.center.y == to_box.center.y {
        from = Point::new(from_box.center.x +from_box.radius/2 + num,
                                    from_box.center.y + from_box.radius);
        to = Point::new(to_box.center.x + to_box.radius/2 + num,
                                    to_box.center.y + to_box.radius);

        if from_box.radius > to_box.radius {
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
    else if from_box.center.y < to_box.center.y {
        from = Point::new(from_box.center.x +from_box.radius/2 + num,
                                    from_box.center.y + from_box.radius);
        to = Point::new(to_box.center.x + to_box.radius/2 + num,
                                    to_box.center.y);
        draw_dashed_line(image, Point::new(from.x, from.y),
                                Point::new(from.x, from.y + num));
        draw_dashed_line(image, Point::new(from.x, from.y + num),
                                Point::new(to.x, from.y + num));
        draw_dashed_line(image, Point::new(to.x, from.y + num),
                                Point::new(to.x, to.y));
        draw_arrow(image, to.clone(), Direction::Down);
    }
    else if from_box.center.y > to_box.center.y {
        from = Point::new(from_box.center.x + from_box.radius/2 + num,
                                    from_box.center.y);
        to = Point::new(to_box.center.x + to_box.radius/2 + num,
                                    to_box.center.y + to_box.radius);
        draw_dashed_line(image, Point::new(from.x, from.y),
                                Point::new(from.x, from.y - num));
        draw_dashed_line(image, Point::new(from.x, from.y - num),
                                Point::new(to.x, from.y - num));
        draw_dashed_line(image, Point::new(to.x, from.y - num),
                                Point::new(to.x, to.y));
        draw_arrow(image, to.clone(), Direction::Up);

    }
}*/

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

pub fn draw_usecase_box(image: &mut RgbImage, usecase: String, x: &mut i32, y: &mut i32,
                    row: u32, column: u32) -> UseCase
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
    let usecase_box: UseCase;

    max_characters = usecase.len();

    if column == 0 { *x = 50; }

    if row > 0 && column == 0 { *y = *y + 300; }
    let ellipse_height = size as i32 + 5;
    let ellipse_width = max_characters as i32 * (size as i32/4) + 5;

    draw_text_mut(image, black, *x as u32 + 5, *y as u32 + 2, scale, &font, &usecase);
    draw_hollow_ellipse_mut(image, (*x as i32 + (max_characters as i32 * (size as i32/4)), *y as i32 + size as i32/2),
                                 ellipse_width, ellipse_height, black);
    //Generate the box

    let nametag_box_width = (size as u32 -6)*(max_characters as u32);
    let box_width = (size as u32 -6)*(max_characters as u32)+50;

    usecase_box = UseCase::new(usecase.clone(), Point::new(*x as u32,*y as u32), ellipse_height as u32, ellipse_width as u32, 1);
    *x = box_width as i32 + *x + 50;
    return usecase_box;
}
