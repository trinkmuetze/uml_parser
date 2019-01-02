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
struct ClassBox{
    name: String,
    start: Point,
    box_width: u32,
    box_height: u32,
    row: u32,
    column: u32,
    associations: u32,
}

impl ClassBox{
    fn new(name: String, start: Point, box_width: u32, box_height: u32,
                row: u32, column: u32)
            -> ClassBox{
        ClassBox{
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

fn draw_association(image: &mut RgbImage, association: parser::class::Relationship, class_boxes: Vec<ClassBox>)
{
    let font_data = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font_data).unwrap().into_font().unwrap();

    let size = 10.0;
    let scale = Scale { x: size, y: size };

    let num = rand::thread_rng().gen_range(0, 100);
    let mut from_box: ClassBox = ClassBox::new("".to_string(), Point::new(0,0), 0, 0, 0, 0);
    let mut to_box: ClassBox = ClassBox::new("".to_string(), Point::new(0,0), 0, 0, 0, 0);

    for class_box in class_boxes{
        if class_box.name == association.class.name{ from_box = class_box.clone(); }
        if class_box.name == association.to_class.name{ to_box = class_box.clone(); }
    }

    let mut from = Point::new(from_box.start.x +from_box.box_width/2,
                                from_box.start.y);
    let mut to = Point::new(to_box.start.x + to_box.box_width/2,
                                to_box.start.y + to_box.box_height);

    if from_box.start.y == to_box.start.y {
        from = Point::new(from_box.start.x + from_box.box_width/2,
                                    from_box.start.y + from_box.box_height);
        to = Point::new(to_box.start.x + to_box.box_width/2 + num,
                                    to_box.start.y + to_box.box_height);

        if from_box.box_height > to_box.box_height {
            draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), from.x + 5, from.y + 5, scale, &font, &association.class.multiplicity.to_string());

            draw_line_segment_mut(image, (from.x as f32, from.y as f32),
                                    (from.x as f32, from.y as f32 + 20.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (from.x as f32, from.y as f32 + 20.0),
                                    (to.x as f32, from.y as f32 + 20.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (to.x as f32, from.y as f32 + 20.0),
                                    (to.x as f32, to.y as f32), Rgb([0u8, 0u8, 0u8]));
            draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), to.x + 5, to.y + 5, scale, &font, &association.to_class.multiplicity.to_string());

        }
        else{
            draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), from.x + 5, from.y + 5, scale, &font, &association.class.multiplicity.to_string());

            draw_line_segment_mut(image, (from.x as f32, from.y as f32),
                                    (from.x as f32, to.y as f32 + 20.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (from.x as f32, to.y as f32 + 20.0),
                                    (to.x as f32, to.y as f32 + 20.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (to.x as f32, to.y as f32 + 20.0),
                                    (to.x as f32, to.y as f32), Rgb([0u8, 0u8, 0u8]));
            draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), to.x + 5, to.y + 5, scale, &font, &association.to_class.multiplicity.to_string());
        }

        if association.relation_type == "association" {}
        else if association.relation_type == "aggregation" { draw_aggregation_arrow(image, to.clone(), Direction::Up);}
        else if association.relation_type == "composition" {draw_composition_arrow(image, to.clone(), Direction::Up)}
        else if association.relation_type == "inheritance" { draw_inheritance_arrow(image, to.clone(), Direction::Up);}
        else if association.relation_type == "implementation" {}
        else if association.relation_type == "dependency" {draw_arrow(image, to.clone(), Direction::Up);}
    }
    else if from_box.start.y < to_box.start.y {
        from = Point::new(from_box.start.x +from_box.box_width/2 + num,
                                    from_box.start.y + from_box.box_height);
        to = Point::new(to_box.start.x + to_box.box_width/2 + num,
                                    to_box.start.y);
        draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), from.x + 5, from.y + 5, scale, &font, &association.class.multiplicity.to_string());

        draw_line_segment_mut(image, (from.x as f32, from.y as f32),
                                (from.x as f32, from.y as f32 + num as f32), Rgb([0u8, 0u8, 0u8]));
        draw_line_segment_mut(image, (from.x as f32, from.y as f32 + num as f32),
                                (to.x as f32, from.y as f32 + num as f32), Rgb([0u8, 0u8, 0u8]));
        draw_line_segment_mut(image, (to.x as f32, from.y as f32 + num as f32),
                                (to.x as f32, to.y as f32), Rgb([0u8, 0u8, 0u8]));
        draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), to.x + 5, to.y + 5, scale, &font, &association.to_class.multiplicity.to_string());

        if association.relation_type == "association" {}
        else if association.relation_type == "aggregation" { draw_aggregation_arrow(image, to.clone(), Direction::Down);}
        else if association.relation_type == "composition" {draw_composition_arrow(image, to.clone(), Direction::Down)}
        else if association.relation_type == "inheritance" { draw_inheritance_arrow(image, to.clone(), Direction::Down);}
        else if association.relation_type == "dependency" {draw_arrow(image, to.clone(), Direction::Up);}
    }
    else if from_box.start.y > to_box.start.y {
        from = Point::new(from_box.start.x + from_box.box_width/2 + num,
                                    from_box.start.y);
        to = Point::new(to_box.start.x + to_box.box_width/2 + num,
                                    to_box.start.y + to_box.box_height);
        draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), from.x + 5, from.y - 10, scale, &font, &association.class.multiplicity.to_string());
        draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), to.x + 5, to.y + 20, scale, &font, &association.to_class.multiplicity.to_string());

        draw_line_segment_mut(image, (from.x as f32, from.y as f32),
                                (from.x as f32, from.y as f32 - num as f32), Rgb([0u8, 0u8, 0u8]));
        draw_line_segment_mut(image, (from.x as f32, from.y as f32 - num as f32),
                                (to.x as f32, from.y as f32 - num as f32), Rgb([0u8, 0u8, 0u8]));
        draw_line_segment_mut(image, (to.x as f32, from.y as f32 - num as f32),
                                (to.x as f32, to.y as f32), Rgb([0u8, 0u8, 0u8]));

        if association.relation_type == "association" {}
        else if association.relation_type == "aggregation" { draw_aggregation_arrow(image, to.clone(), Direction::Up);}
        else if association.relation_type == "composition" {draw_composition_arrow(image, to.clone(), Direction::Up)}
        else if association.relation_type == "inheritance" { draw_inheritance_arrow(image, to.clone(), Direction::Up);}
        else if association.relation_type == "dependency" {draw_arrow(image, to.clone(), Direction::Up);}
    }
}

fn draw_association_dashed(image: &mut RgbImage, association: parser::class::Relationship, class_boxes: Vec<ClassBox>){
    let mut from_box: ClassBox = ClassBox::new("".to_string(), Point::new(0,0), 0, 0, 0, 0);
    let mut to_box: ClassBox = ClassBox::new("".to_string(), Point::new(0,0), 0, 0, 0, 0);

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
        draw_inheritance_arrow(image, to.clone(), Direction::Up);
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
        draw_inheritance_arrow(image, to.clone(), Direction::Down);
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
        draw_inheritance_arrow(image, to.clone(), Direction::Up);

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

fn draw_inheritance_arrow(image: &mut RgbImage, point: Point, direction: Direction) {
    match direction{
        Direction::Up => {
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 - 10.0, point.y as f32 + 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 + 10.0, point.y as f32 + 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32 - 10.0, point.y as f32 + 15.0),
                                    (point.x as f32 + 10.0, point.y as f32 + 15.0), Rgb([0u8, 0u8, 0u8]));
                            },
        Direction::Down => {
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 + 10.0, point.y as f32 - 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 - 10.0, point.y as f32 - 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32 + 10.0, point.y as f32 - 15.0),
                                    (point.x as f32 - 10.0, point.y as f32 - 15.0), Rgb([0u8, 0u8, 0u8]));
                                },
        Direction::ToLeft => {

        },
        Direction::ToRight => {

        },
    }
}

fn draw_aggregation_arrow(image: &mut RgbImage, point: Point, direction: Direction){
    match direction{
        Direction::Up => {
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 - 10.0, point.y as f32 + 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 + 10.0, point.y as f32 + 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32 - 10.0, point.y as f32 + 15.0),
                                    (point.x as f32, point.y as f32 + 30.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32 + 10.0, point.y as f32 + 15.0),
                                    (point.x as f32, point.y as f32 + 30.0), Rgb([0u8, 0u8, 0u8]));
                            },
        Direction::Down => {
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 + 10.0, point.y as f32 - 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 - 10.0, point.y as f32 - 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32 + 10.0, point.y as f32 - 15.0),
                                    (point.x as f32, point.y as f32 - 30.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32 - 10.0, point.y as f32 - 15.0),
                                    (point.x as f32, point.y as f32 - 30.0), Rgb([0u8, 0u8, 0u8]));
                                },
        Direction::ToLeft => {

        },
        Direction::ToRight => {

        },
    }
}

fn draw_composition_arrow(image: &mut RgbImage, point: Point, direction: Direction){
    match direction {
        Direction::Up => {
            let mut point_x = point.x + 10;
            while point_x > point.x {
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point_x as f32 - 10.0, point.y as f32 + 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point_x as f32, point.y as f32 + 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point_x as f32 - 10.0, point.y as f32 + 15.0),
                                    (point.x as f32, point.y as f32 + 30.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point_x as f32, point.y as f32 + 15.0),
                                    (point.x as f32, point.y as f32 + 30.0), Rgb([0u8, 0u8, 0u8]));
            point_x = point_x - 1;
                                }
                            },
        Direction::Down => {
            let mut point_x = point.x + 10;
            while point_x > point.x{
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point_x as f32, point.y as f32 - 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point_x as f32 - 10.0, point.y as f32 - 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point_x as f32, point.y as f32 - 15.0),
                                    (point.x as f32, point.y as f32 - 30.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point_x as f32 - 10.0, point.y as f32 - 15.0),
                                    (point.x as f32, point.y as f32 - 30.0), Rgb([0u8, 0u8, 0u8]));

            point_x = point_x -1;
                                }
                                },
        Direction::ToLeft => {

        },
        Direction::ToRight => {

        },
    }
}

fn draw_package_box(image: &mut RgbImage, package: parser::package::Package, x: &mut i32, y: &mut i32,
                    row: u32, column: u32) -> ClassBox
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
    let package_box: ClassBox;

    max_characters = package.name.len();

    if column == 0 { *x = 50; }

    if row > 0 && column == 0 { *y = *y + 300; }

    draw_text_mut(image, black, *x as u32 + 5, *y as u32 + 2, scale, &font, &package.name);

    //Generate the box

    let nametag_box_width = (size as u32 -6)*(max_characters as u32);
    let box_width = (size as u32 -6)*(max_characters as u32);

    draw_hollow_rect_mut(image, Rect::at(*x,*y).of_size(nametag_box_width, size as u32), black);

    draw_hollow_rect_mut(image, Rect::at(*x,*y + size as i32).of_size(box_width, box_height), black);

    package_box = ClassBox::new(package.name, Point::new(*x as u32,*y as u32), box_width, box_height, row, column);
    *x = box_width as i32 + *x + 50;
    //draw_hollow_rect_mut(&mut image, Rect::at(x, y).of_size(box_width, box_height), black);
    return package_box;
}

fn drawclass_box(image: &mut RgbImage, class: parser::class::Class, x: &mut i32, y: &mut i32, width: u32, height: u32,
                    row: u32, column: u32) -> ClassBox
{
    //Used RGBs
    let white = Rgb([255u8, 255u8, 255u8]);
    let black = Rgb([0u8, 0u8, 0u8]);

    //Configuring the font
    let font_data = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font_data).unwrap().into_font().unwrap();

    let mut counter = 1;

    let attribute_len = class.attributes.len();
    let method_len = class.methods.len();

    let size = 16.0;
    let scale = Scale { x: size, y: size };
    let box_height = (attribute_len + method_len + 3) as u32 * size as u32;
    let mut text_position_y = 0;
    let mut max_characters = 0;
    let class_box: ClassBox;

    if max_characters < class.name.len()
    {
        max_characters = class.name.len();
    }

    if column == 0 { *x = 50; }

    if row > 0 && column == 0 { *y = *y + 300; }

    draw_text_mut(image, black, *x as u32 + 5, *y as u32 + 2, scale, &font, &class.name);
    //-----------------------------------Attributes------------------------------------------------
    for attribute in class.attributes
    {
        let attribute_line = attribute.visibility + &attribute.name + " : " + &attribute.data_type;
        if max_characters < attribute_line.len()
        {
            max_characters = attribute_line.len();
        }
        text_position_y = *y as u32 + 10 + (counter as u32 * size as u32);

        draw_text_mut(image, black, *x as u32 + 5, text_position_y, scale, &font, &attribute_line);
        counter = counter +1;
    }
    counter = counter +1;
    for method in class.methods
    {
        let mut parameters : String = "".to_string();
        for parameter in method.parameters {
            let parameter_line : String = parameter.name + " : " + &parameter.data_type + ", ";
            parameters.push_str(&parameter_line);
        }
        parameters.pop();
        parameters.pop();
        let method_line = method.visibility + &method.name + "(" + &parameters + ")";
        if max_characters < method_line.len()
        {
            max_characters = method_line.len();
        }
        text_position_y = *y as u32 + 10 + (counter as u32 * size as u32);
        draw_text_mut(image, black, *x as u32 + 5, text_position_y, scale, &font, &method_line);
        counter = counter +1;
    }
    //Generate the box
    let box_width = (size as u32 -6)*(max_characters as u32);
    draw_line_segment_mut(image, (*x as f32, *y as f32 + (size*(attribute_len as f32 + 2.0)) + 5.0),
                        (*x as f32 + box_width as f32, *y as f32 + size*(attribute_len as f32 + 2.0) + 5.0), black);
    draw_line_segment_mut(image, (*x as f32, *y as f32 + size + 5.0),
                                    (*x as f32 + box_width as f32, *y as f32 + size + 5.0), black);
    draw_hollow_rect_mut(image, Rect::at(*x,*y).of_size(box_width, box_height), black);

    class_box = ClassBox::new(class.name, Point::new(*x as u32,*y as u32), box_width, box_height, row, column);
    *x = box_width as i32 + *x + 50;
    //draw_hollow_rect_mut(&mut image, Rect::at(x, y).of_size(box_width, box_height), black);
    return class_box;
}

/*fn draw_object_box(image: &mut RgbImage, object: parser::Object, x: &mut i32, y: &mut i32, width: u32, height: u32,
                    row: u32, column: u32) -> ClassBox
{
    //Used RGBs
    let white = Rgb([255u8, 255u8, 255u8]);
    let black = Rgb([0u8, 0u8, 0u8]);

    //Configuring the font
    let font_data = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font_data).unwrap().into_font().unwrap();

    let mut counter = 1;

    let attribute_len = class.attributes.len();
    let method_len = class.methods.len();

    let size = 16.0;
    let scale = Scale { x: size, y: size };
    let box_height = (attribute_len + method_len + 3) as u32 * size as u32;
    let mut text_position_y = 0;
    let mut max_characters = 0;
    let class_box: ClassBox;

    if max_characters < class.name.len()
    {
        max_characters = class.name.len();
    }

    if column == 0 { *x = 50; }

    if row > 0 && column == 0 { *y = *y + 300; }

    draw_text_mut(image, black, *x as u32 + 5, *y as u32 + 2, scale, &font, &class.name);
    //-----------------------------------Attributes------------------------------------------------
    for attribute in class.attributes
    {
        let attribute_line = attribute.visibility + &attribute.name + " : " + &attribute.data_type;
        if max_characters < attribute_line.len()
        {
            max_characters = attribute_line.len();
        }
        text_position_y = *y as u32 + 10 + (counter as u32 * size as u32);

        draw_text_mut(image, black, *x as u32 + 5, text_position_y, scale, &font, &attribute_line);
        counter = counter +1;
    }
    counter = counter +1;
    for method in class.methods
    {
        let mut parameters : String = "".to_string();
        for parameter in method.parameters {
            let parameter_line : String = parameter.name + " : " + &parameter.data_type + ", ";
            parameters.push_str(&parameter_line);
        }
        parameters.pop();
        parameters.pop();
        let method_line = method.visibility + &method.name + "(" + &parameters + ")";
        if max_characters < method_line.len()
        {
            max_characters = method_line.len();
        }
        text_position_y = *y as u32 + 10 + (counter as u32 * size as u32);
        draw_text_mut(image, black, *x as u32 + 5, text_position_y, scale, &font, &method_line);
        counter = counter +1;
    }
    //Generate the box
    let box_width = (size as u32 -6)*(max_characters as u32);
    draw_line_segment_mut(image, (*x as f32, *y as f32 + (size*(attribute_len as f32 + 2.0)) + 5.0),
                        (*x as f32 + box_width as f32, *y as f32 + size*(attribute_len as f32 + 2.0) + 5.0), black);
    draw_line_segment_mut(image, (*x as f32, *y as f32 + size + 5.0),
                                    (*x as f32 + box_width as f32, *y as f32 + size + 5.0), black);
    draw_hollow_rect_mut(image, Rect::at(*x,*y).of_size(box_width, box_height), black);

    class_box = ClassBox::new(class.name, Point::new(*x as u32,*y as u32), box_width, box_height, row, column);
    *x = box_width as i32 + *x + 50;
    //draw_hollow_rect_mut(&mut image, Rect::at(x, y).of_size(box_width, box_height), black);
    return class_box;
}*/

pub fn generate_package_diagram(packages: Vec<parser::package::Package>, height: u32, width: u32, diagram_name: &str) -> bool
{
    let mut boxes: Vec<ClassBox> = Vec::new();
    //Path of the diagram
    let path = Path::new("class_diagram.png");

    //Used RGBs
    let white = Rgb([255u8, 255u8, 255u8]);
    let black = Rgb([0u8, 0u8, 0u8]);

    //Origin of the first class
    let mut x = 50;
    let mut y = 70;

    let mut image:RgbImage = RgbImage::new(width, height);
    //Configuring the font
    let font_data = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font_data).unwrap().into_font().unwrap();
    let mut size = 30.0;
    let scale = Scale { x: size, y: size };

    //White Background with title of the diagram
    draw_filled_rect_mut(&mut image, Rect::at(0,0).of_size(width,height), white);
    draw_text_mut(&mut image, black, (width/2)-(diagram_name.len() as u32*size as u32/2), 10, scale, &font, &diagram_name);

    let mut row = 0;
    let mut column = 0;
    let mut class_row = 0;
    let mut class_column = 0;

    for package in packages.clone() {
        //Generating the class box
        column = column + 1;
        if column > 3 {
            column = 0;
            row = row + 1;
        }
        for class in package.packages.clone()
        {
            class_column = class_column + 1;
            if class_column > 3 {
                class_column = 0;
                class_row = class_row + 1;
            }
        }
        boxes.push(draw_package_box(&mut image, package.clone(), &mut x, &mut y, row, column));
    }

    //-----------------Relationships------------------------------------------//
    /*for relationship in relationships.clone(){
            draw_association_dashed(&mut image, relationship.clone(), boxes.clone());
    }*/
    image.save(path).unwrap();
    return true;
}

pub fn generate_class_diagram(relationships: Vec<parser::class::Relationship>,
    classes: Vec<parser::class::Class>, height: u32, width: u32, diagram_name: &str) -> bool
{
    let mut boxes: Vec<ClassBox> = Vec::new();
    //Path of the diagram
    let path = Path::new("class_diagram.png");

    //Used RGBs
    let white = Rgb([255u8, 255u8, 255u8]);
    let black = Rgb([0u8, 0u8, 0u8]);

    //Origin of the first class
    let mut x = 50;
    let mut y = 70;

    let mut image:RgbImage = RgbImage::new(width, height);
    //Configuring the font
    let font_data = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font_data).unwrap().into_font().unwrap();
    let mut size = 30.0;
    let scale = Scale { x: size, y: size };

    //White Background with title of the diagram
    draw_filled_rect_mut(&mut image, Rect::at(0,0).of_size(width,height), white);
    draw_text_mut(&mut image, black, (width/2)-(diagram_name.len() as u32*size as u32/2), 10, scale, &font, &diagram_name);

    let mut row = 0;
    let mut column = 0;

    for class in classes.clone() {
        //Generating the class box
        column = column + 1;
        if column > 3 {
            column = 0;
            row = row + 1;
        }
        boxes.push(drawclass_box(&mut image, class.clone(), &mut x, &mut y, width.clone(), height.clone(), row, column));
    }

    //-----------------Relationships------------------------------------------//
    for relationship in relationships.clone(){
        if relationship.relation_type == "implementation" {
            draw_association_dashed(&mut image, relationship.clone(), boxes.clone());
        }
        else {
            draw_association(&mut image, relationship.clone(), boxes.clone());
        }
    }
    image.save(path).unwrap();
    return true;
}

/*pub fn generate_object_diagram (relationships: Vec<parser::Relationship>,
    objects: Vec<parser::Object>, height: u32, width: u32, diagram_name: &str) -> bool
{
    let mut boxes: Vec<ClassBox> = Vec::new();
    //Path of the diagram
    let path = Path::new("class_diagram.png");

    //Used RGBs
    let white = Rgb([255u8, 255u8, 255u8]);
    let black = Rgb([0u8, 0u8, 0u8]);

    //Origin of the first class
    let mut x = 50;
    let mut y = 70;

    let mut image:RgbImage = RgbImage::new(width, height);
    //Configuring the font
    let font_data = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font_data).unwrap().into_font().unwrap();
    let mut size = 30.0;
    let scale = Scale { x: size, y: size };

    //White Background with title of the diagram
    draw_filled_rect_mut(&mut image, Rect::at(0,0).of_size(width,height), white);
    draw_text_mut(&mut image, black, (width/2)-(diagram_name.len() as u32*size as u32/2), 10, scale, &font, &diagram_name);

    let mut row = 0;
    let mut column = 0;

    for class in classes.clone() {
        //Generating the class box
        column = column + 1;
        if column > 3 {
            column = 0;
            row = row + 1;
        }
        boxes.push(drawclass_box(&mut image, class.clone(), &mut x, &mut y, width.clone(), height.clone(), row, column));
    }

    //-----------------Relationships------------------------------------------//
    for relationship in relationships.clone(){
        if relationship.relation_type == "implementation" {
            draw_association_dashed(&mut image, relationship.clone(), boxes.clone());
        }
        else {
            draw_association(&mut image, relationship.clone(), boxes.clone());
        }
    }
    image.save(path).unwrap();
    return true;
}*/
