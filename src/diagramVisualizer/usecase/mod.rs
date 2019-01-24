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
    pub x: u32,
    pub y: u32,
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
pub struct Acteur {
    pub name: String,
    pub hand_position: Point,
    pub head_position: Point,
}

impl Acteur {
    pub fn new(name: String, hand_position: Point,
            head_position: Point) -> Acteur {
        Acteur {
            name: name,
            hand_position: hand_position,
            head_position: head_position,
        }
    }
}

#[derive(Clone)]
pub struct UseCase{
    pub name: String,
    pub center: Point,
    height_radius: u32,
    width_radius: u32,
    priority: u32,
    associations: u32,
}

impl UseCase {
    pub fn new(name: String, center: Point, height_radius: u32, width_radius: u32, priority: u32)
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

pub fn draw_acteur(image: &mut RgbImage, position: Point, name: &str) -> Acteur {

    let black = Rgb([0u8, 0u8, 0u8]);
    let mut size = 16.0;
    let scale = Scale { x: size, y: size };
    //Configuring the font
    let font_data = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font_data).unwrap().into_font().unwrap();

    //HEAD
    draw_hollow_circle_mut(image, (position.x as i32, position.y as i32), 10, black);
    //BODY
    draw_line_segment_mut(image, (position.x as f32, position.y as f32 + 10.0),
                                    (position.x as f32, position.y as f32 + 50.0),black);
    //LEFT LEG
    draw_line_segment_mut(image, (position.x as f32, position.y as f32 + 50.0),
                                    (position.x as f32 - 10.0, position.y as f32 + 75.0), black);
    //RIGHT LEG
    draw_line_segment_mut(image, (position.x as f32, position.y as f32 + 50.0),
                                    (position.x as f32 + 10.0, position.y as f32 + 75.0), black);
    //ARMS
    draw_line_segment_mut(image, (position.x as f32 - 10.0, position.y as f32 + 20.0),
                                    (position.x as f32 + 10.0, position.y as f32 + 20.0), black);
    //ACTEUR NAME
    draw_text_mut(image, black, position.x - 20, position.y + 80, scale, &font, &name);

    let acteur: Acteur = Acteur::new(name.to_string(), Point::new(position.x + 10, position.y + 20), position);

    return acteur;
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

pub fn draw_acteur_case_association(image: &mut RgbImage, acteur: Acteur, case: UseCase) {
    draw_line_segment_mut(image, (acteur.hand_position.x as f32,acteur.hand_position.y as f32),
                            (case.center.x as f32 - case.width_radius as f32, case.center.y as f32), Rgb([0u8, 0u8, 0u8]));
}

pub fn draw_case_case_association(image: &mut RgbImage, from_case: UseCase, to_case: UseCase, relation_type: String) {
    let mut size = 16.0;
    let scale = Scale { x: size, y: size };
    //Configuring the font
    let font_data = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font_data).unwrap().into_font().unwrap();
    let black = Rgb([0u8, 0u8, 0u8]);
    let mut label = "<<".to_string();
    label.push_str(&relation_type);
    label.push_str(">>");

    if from_case.center.y > to_case.center.y {
        draw_dashed_line(image, Point::new(from_case.center.x,from_case.center.y - from_case.height_radius),
                                Point::new(from_case.center.x, from_case.center.y - from_case.height_radius - 10));
        draw_dashed_line(image, Point::new(from_case.center.x, from_case.center.y - from_case.height_radius - 10),
                                Point::new(to_case.center.x, from_case.center.y - from_case.height_radius - 10));
        draw_dashed_line(image, Point::new(to_case.center.x, from_case.center.y - from_case.height_radius - 10),
                                Point::new(to_case.center.x, to_case.center.y + to_case.height_radius));
        draw_arrow(image, Point::new(to_case.center.x, to_case.center.y + to_case.height_radius), Direction::Up);
        draw_text_mut(image, black, to_case.center.x - (label.len()as u32 * size as u32/2) -15,
                            from_case.center.y - from_case.height_radius - 25, scale, &font, &label);
    }
    else if from_case.center.y < to_case.center.y {

        draw_dashed_line(image, Point::new(from_case.center.x,from_case.center.y + from_case.height_radius),
                                Point::new(from_case.center.x, from_case.center.y + from_case.height_radius - 10));
        draw_dashed_line(image, Point::new(from_case.center.x, from_case.center.y + from_case.height_radius - 10),
                                Point::new(to_case.center.x, from_case.center.y + from_case.height_radius - 10));
        draw_dashed_line(image, Point::new(to_case.center.x, from_case.center.y + from_case.height_radius - 10),
                                Point::new(to_case.center.x, to_case.center.y - to_case.height_radius));
        draw_arrow(image, Point::new(to_case.center.x, to_case.center.y - to_case.height_radius), Direction::Down);
        draw_text_mut(image, black, to_case.center.x - (label.len()as u32 * size as u32/2) -15,
                            from_case.center.y - from_case.height_radius - 25, scale, &font, &label);
    }
    else if from_case.center.x > to_case.center.x {
        draw_dashed_line(image, Point::new(from_case.center.x - from_case.width_radius, from_case.center.y),
                                Point::new(to_case.center.x + to_case.width_radius, to_case.center.y));
        draw_arrow(image, Point::new(to_case.center.x + to_case.width_radius, to_case.center.y), Direction::ToLeft);
        draw_text_mut(image, black, to_case.center.x + 60, from_case.center.y - 20, scale, &font, &label);
    }
    else if from_case.center.x < to_case.center.x {
        draw_dashed_line(image, Point::new(from_case.center.x + from_case.width_radius, from_case.center.y),
                                Point::new(to_case.center.x - to_case.width_radius, to_case.center.y));
        draw_arrow(image, Point::new(to_case.center.x - to_case.width_radius, to_case.center.y),Direction::ToRight);
        draw_text_mut(image, black, from_case.center.x + 60, from_case.center.y - 20, scale, &font, &label);

    }
}

pub fn draw_acteur_acteur_association(image: &mut RgbImage, from_acteur: Acteur, to_acteur: Acteur) {
        draw_dashed_line(image, Point::new(from_acteur.hand_position.x - 25,from_acteur.hand_position.y),
                                Point::new(from_acteur.hand_position.x - 45, from_acteur.hand_position.y));
        draw_dashed_line(image, Point::new(from_acteur.hand_position.x - 45, from_acteur.hand_position.y),
                                Point::new(to_acteur.hand_position.x - 45, to_acteur.hand_position.y));
        draw_dashed_line(image, Point::new(to_acteur.hand_position.x - 45, to_acteur.hand_position.y),
                                Point::new(to_acteur.hand_position.x - 25, to_acteur.hand_position.y));
        draw_arrow(image, Point::new(to_acteur.hand_position.x - 25, to_acteur.hand_position.y), Direction::ToRight);
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
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 + 15.0, point.y as f32 - 10.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 + 15.0, point.y as f32 + 10.0), Rgb([0u8, 0u8, 0u8]));
        },
        Direction::ToRight =>{
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 - 15.0, point.y as f32 + 10.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 - 15.0, point.y as f32 - 10.0), Rgb([0u8, 0u8, 0u8]));
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

    if column == 0 { *x = 130; }

    if row > 0 && column == 0 { *y = *y + 100; }
    let ellipse_height = size as i32 + 5;
    let ellipse_width = max_characters as i32 * (size as i32/4) + 5;

    draw_text_mut(image, black, *x as u32 + 5, *y as u32 + 2, scale, &font, &usecase);
    draw_hollow_ellipse_mut(image, (*x as i32 + (max_characters as i32 * (size as i32/4)), *y as i32 + size as i32/2),
                                 ellipse_width, ellipse_height, black);
    //Generate the box

    let nametag_box_width = (size as u32 -6)*(max_characters as u32);
    let box_width = (size as u32 -6)*(max_characters as u32)+50;

    usecase_box = UseCase::new(usecase.clone(), Point::new(*x as u32 + (max_characters as u32 * (size as u32/4)),
                                *y as u32 + size as u32/2), ellipse_height as u32, ellipse_width as u32, 1);
    *x = box_width as i32 + *x + 50;
    return usecase_box;
}
