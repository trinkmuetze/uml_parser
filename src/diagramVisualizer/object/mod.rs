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
pub struct ObjectBox{
    name: String,
    start: Point,
    box_width: u32,
    box_height: u32,
    row: u32,
    column: u32,
    associations: u32,
}

impl ObjectBox{
    fn new(name: String, start: Point, box_width: u32, box_height: u32,
                row: u32, column: u32)
            -> ObjectBox{
        ObjectBox{
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

pub fn draw_association(image: &mut RgbImage, association: parser::object::Link, object_boxes: Vec<ObjectBox>)
{
    let font_data = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font_data).unwrap().into_font().unwrap();

    let size = 10.0;
    let scale = Scale { x: size, y: size };

    let num = rand::thread_rng().gen_range(0, 100);
    let mut from_box: ObjectBox = ObjectBox::new("".to_string(), Point::new(0,0), 0, 0, 0, 0);
    let mut to_box: ObjectBox = ObjectBox::new("".to_string(), Point::new(0,0), 0, 0, 0, 0);

    for object_box in object_boxes{
        if object_box.name == association.object.name{ from_box = object_box.clone(); }
        if object_box.name == association.to_object.name{ to_box = object_box.clone(); }
    }

    let from;
    let to;

    if from_box.start.y == to_box.start.y {
        from = Point::new(from_box.start.x + from_box.box_width/2 + num,
                                    from_box.start.y + from_box.box_height);
        to = Point::new(to_box.start.x + to_box.box_width/2 + num,
                                    to_box.start.y + to_box.box_height);

        if from_box.box_height > to_box.box_height {
            draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), from.x + 5, from.y + 5, scale, &font, &association.object.role.to_string());

            draw_line_segment_mut(image, (from.x as f32, from.y as f32),
                                    (from.x as f32, from.y as f32 + 20.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (from.x as f32, from.y as f32 + 20.0),
                                    (to.x as f32, from.y as f32 + 20.0), Rgb([0u8, 0u8, 0u8]));
            draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), from.x + 100, from.y + 25, scale, &font, &association.name.to_string());
            draw_line_segment_mut(image, (to.x as f32, from.y as f32 + 20.0),
                                    (to.x as f32, to.y as f32), Rgb([0u8, 0u8, 0u8]));
            draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), to.x + 5, to.y + 5, scale, &font, &association.to_object.role.to_string());

        }
        else{
            draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), from.x + 5, from.y + 5, scale, &font, &association.object.role.to_string());

            draw_line_segment_mut(image, (from.x as f32, from.y as f32),
                                    (from.x as f32, to.y as f32 + 20.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (from.x as f32, to.y as f32 + 20.0),
                                    (to.x as f32, to.y as f32 + 20.0), Rgb([0u8, 0u8, 0u8]));
            draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), from.x + 100, to.y + 25, scale, &font, &association.name.to_string());
            if from.x > to.x { draw_arrow(image, Point::new(from.x + 80, to.y + 25), Direction::ToLeft );}
            else { draw_arrow(image, Point::new(from.x + 80, to.y + 25), Direction::ToRight );}
            draw_line_segment_mut(image, (to.x as f32, to.y as f32 + 20.0),
                                    (to.x as f32, to.y as f32), Rgb([0u8, 0u8, 0u8]));
            draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), to.x + 5, to.y + 5, scale, &font, &association.to_object.role.to_string());
        }
    }
    else if from_box.start.y < to_box.start.y {
        from = Point::new(from_box.start.x +from_box.box_width/2 + num,
                                    from_box.start.y + from_box.box_height);
        to = Point::new(to_box.start.x + to_box.box_width/2 + num,
                                    to_box.start.y);
        draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), from.x + 5, from.y + 5, scale, &font, &association.object.role.to_string());

        draw_line_segment_mut(image, (from.x as f32, from.y as f32),
                                (from.x as f32, from.y as f32 + num as f32), Rgb([0u8, 0u8, 0u8]));
        draw_line_segment_mut(image, (from.x as f32, from.y as f32 + num as f32),
                                (to.x as f32, from.y as f32 + num as f32), Rgb([0u8, 0u8, 0u8]));
        draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), to.x + 20, from.y + 100, scale, &font, &association.name.to_string());
        draw_line_segment_mut(image, (to.x as f32, from.y as f32 + num as f32),
                                (to.x as f32, to.y as f32), Rgb([0u8, 0u8, 0u8]));
        draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), to.x + 5, to.y + 5, scale, &font, &association.to_object.role.to_string());
    }
    else if from_box.start.y > to_box.start.y {
        from = Point::new(from_box.start.x + from_box.box_width/2 + num,
                                    from_box.start.y);
        to = Point::new(to_box.start.x + to_box.box_width/2 + num,
                                    to_box.start.y + to_box.box_height);
        draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), from.x + 5, from.y - 10, scale, &font, &association.object.role.to_string());
        draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), to.x + 5, to.y + 20, scale, &font, &association.to_object.role.to_string());

        draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), from.x, from.y - num, scale, &font, &association.name.to_string());

        draw_line_segment_mut(image, (from.x as f32, from.y as f32),
                                (from.x as f32, from.y as f32 - num as f32), Rgb([0u8, 0u8, 0u8]));
        draw_line_segment_mut(image, (from.x as f32, from.y as f32 - num as f32),
                                (to.x as f32, from.y as f32 - num as f32), Rgb([0u8, 0u8, 0u8]));
        draw_line_segment_mut(image, (to.x as f32, from.y as f32 - num as f32),
                                (to.x as f32, to.y as f32), Rgb([0u8, 0u8, 0u8]));
    }
}

pub fn draw_object_box(image: &mut RgbImage, mut object: parser::object::Object, x: &mut i32, y: &mut i32,
                    row: u32, column: u32) -> ObjectBox
{
    //Used RGBs
    let black = Rgb([0u8, 0u8, 0u8]);

    //Configuring the font
    let font_data = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font_data).unwrap().into_font().unwrap();

    let mut counter = 1;

    let attribute_len = object.attributes.len();

    let size = 16.0;
    let scale = Scale { x: size, y: size };
    let box_height = (attribute_len + 3) as u32 * size as u32;
    let mut text_position_y;
    let mut max_characters = 0;
    let class_box: ObjectBox;

    if object.name == "" {
        object.name = object.class.clone();
    }

    if max_characters < object.name.len()
    {
        max_characters = object.name.len() + object.class.len() + 3;
    }

    if column == 0 { *x = 50; }

    if row > 0 && column == 0 { *y = *y + 300; }

    let mut name_line = object.name.clone() + " : " + &object.class;

    if object.class == object.name {
        name_line = object.class.clone();
    }

    draw_text_mut(image, black, *x as u32 + 5, *y as u32 + 2, scale, &font, &name_line);
    draw_line_segment_mut(image, (*x as f32 + 5.0, *y as f32 + size + 1.0),
                                    (*x as f32 + max_characters as f32, *y as f32 + size + 1.0), black);

    //-----------------------------------Attributes------------------------------------------------
    for attribute in object.attributes
    {
        let attribute_line = attribute.attrib_type + ": " + &attribute.name + " = " + &attribute.value;
        if max_characters < attribute_line.len()
        {
            max_characters = attribute_line.len();
        }
        text_position_y = *y as u32 + 10 + (counter as u32 * size as u32);

        draw_text_mut(image, black, *x as u32 + 5, text_position_y, scale, &font, &attribute_line);
        counter = counter +1;
    }

    //Generate the box
    let box_width = (size as u32 -6)*(max_characters as u32);

    draw_hollow_rect_mut(image, Rect::at(*x,*y).of_size(box_width, box_height), black);

    class_box = ObjectBox::new(object.name, Point::new(*x as u32,*y as u32), box_width, box_height, row, column);
    *x = box_width as i32 + *x + 50;
    //draw_hollow_rect_mut(&mut image, Rect::at(x, y).of_size(box_width, box_height), black);
    return class_box;
}
