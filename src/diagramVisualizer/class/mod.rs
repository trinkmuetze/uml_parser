extern crate imageproc;
extern crate rusttype;
extern crate image;
extern crate rand;

use self::rand::Rng;
use self::rusttype::{FontCollection, Scale};
use self::image::{Rgb, RgbImage};
use self::imageproc::rect::Rect;
use self::imageproc::drawing::{
    draw_line_segment_mut,
    draw_hollow_rect_mut,
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
pub struct ClassBox{
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

pub fn draw_association(image: &mut RgbImage, association: parser::class::Relationship, class_boxes: Vec<ClassBox>)
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

    let from;
    let to;

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

pub fn draw_association_dashed(image: &mut RgbImage, association: parser::class::Relationship, class_boxes: Vec<ClassBox>){
    let mut from_box: ClassBox = ClassBox::new("".to_string(), Point::new(0,0), 0, 0, 0, 0);
    let mut to_box: ClassBox = ClassBox::new("".to_string(), Point::new(0,0), 0, 0, 0, 0);

    let num = rand::thread_rng().gen_range(0, 100);

    for class_box in class_boxes {
        if class_box.name == association.class.name { from_box = class_box.clone(); }
        if class_box.name == association.to_class.name { to_box = class_box.clone(); }
    }
    let from;
    let to;

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

pub fn drawclass_box(image: &mut RgbImage, class: parser::class::Class, x: &mut i32, y: &mut i32,
                    row: u32, column: u32) -> ClassBox
{
    //Used RGBs
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
    let mut text_position_y;
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
