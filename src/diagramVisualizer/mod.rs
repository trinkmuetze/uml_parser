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
    fn increaseAssociations(&mut self){
        self.associations = self.associations + 1;
        println!("{}", self.associations);
    }
}

enum Direction{
    to_right,
    up,
    down,
    to_left,
}

fn drawAssociation(image: &mut RgbImage, association: parser::Relationship, classBoxes: Vec<ClassBox>)
{
    let font_data = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font_data).unwrap().into_font().unwrap();

    let mut size = 10.0;
    let mut scale = Scale { x: size, y: size };

    let num = rand::thread_rng().gen_range(0, 100);
    let mut fromBox: ClassBox = ClassBox::new("".to_string(), Point::new(0,0), 0, 0, 0, 0);
    let mut toBox: ClassBox = ClassBox::new("".to_string(), Point::new(0,0), 0, 0, 0, 0);

    for classBox in classBoxes{
        if classBox.name == association.class.name{ fromBox = classBox.clone(); }
        if classBox.name == association.to_class.name{ toBox = classBox.clone(); }
    }

    let mut from = Point::new(fromBox.start.x +fromBox.box_width/2,
                                fromBox.start.y);
    let mut to = Point::new(toBox.start.x + toBox.box_width/2,
                                toBox.start.y + toBox.box_height);

    if(fromBox.start.y == toBox.start.y){
        from = Point::new(fromBox.start.x + fromBox.box_width/2,
                                    fromBox.start.y + fromBox.box_height);
        to = Point::new(toBox.start.x + toBox.box_width/2 + num,
                                    toBox.start.y + toBox.box_height);

        if(fromBox.box_height > toBox.box_height){
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

        if(association.relation_type == "association") {}
        else if (association.relation_type == "aggregation") { draw_aggregation_arrow(image, to.clone(), Direction::up);}
        else if (association.relation_type == "composition") {draw_composition_arrow(image, to.clone(), Direction::up)}
        else if (association.relation_type == "inheritance") { draw_inheritanceArrow(image, to.clone(), Direction::up);}
        else if (association.relation_type == "implementation") {}
        else if (association.relation_type == "dependency") {drawArrow(image, to.clone(), Direction::up);}
    }
    else if (fromBox.start.y < toBox.start.y){
        from = Point::new(fromBox.start.x +fromBox.box_width/2 + num,
                                    fromBox.start.y + fromBox.box_height);
        to = Point::new(toBox.start.x + toBox.box_width/2 + num,
                                    toBox.start.y);
        draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), from.x + 5, from.y + 5, scale, &font, &association.class.multiplicity.to_string());

        draw_line_segment_mut(image, (from.x as f32, from.y as f32),
                                (from.x as f32, from.y as f32 + num as f32), Rgb([0u8, 0u8, 0u8]));
        draw_line_segment_mut(image, (from.x as f32, from.y as f32 + num as f32),
                                (to.x as f32, from.y as f32 + num as f32), Rgb([0u8, 0u8, 0u8]));
        draw_line_segment_mut(image, (to.x as f32, from.y as f32 + num as f32),
                                (to.x as f32, to.y as f32), Rgb([0u8, 0u8, 0u8]));
        draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), to.x + 5, to.y + 5, scale, &font, &association.to_class.multiplicity.to_string());

        if(association.relation_type == "association") {}
        else if (association.relation_type == "aggregation") { draw_aggregation_arrow(image, to.clone(), Direction::down);}
        else if (association.relation_type == "composition") {draw_composition_arrow(image, to.clone(), Direction::down)}
        else if (association.relation_type == "inheritance") { draw_inheritanceArrow(image, to.clone(), Direction::down);}
        else if (association.relation_type == "dependency") {drawArrow(image, to.clone(), Direction::up);}
    }
    else if (fromBox.start.y > toBox.start.y){
        from = Point::new(fromBox.start.x + fromBox.box_width/2 + num,
                                    fromBox.start.y);
        to = Point::new(toBox.start.x + toBox.box_width/2 + num,
                                    toBox.start.y + toBox.box_height);
        draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), from.x + 5, from.y - 10, scale, &font, &association.class.multiplicity.to_string());
        draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), to.x + 5, to.y + 20, scale, &font, &association.to_class.multiplicity.to_string());

        draw_line_segment_mut(image, (from.x as f32, from.y as f32),
                                (from.x as f32, from.y as f32 - num as f32), Rgb([0u8, 0u8, 0u8]));
        draw_line_segment_mut(image, (from.x as f32, from.y as f32 - num as f32),
                                (to.x as f32, from.y as f32 - num as f32), Rgb([0u8, 0u8, 0u8]));
        draw_line_segment_mut(image, (to.x as f32, from.y as f32 - num as f32),
                                (to.x as f32, to.y as f32), Rgb([0u8, 0u8, 0u8]));

        if(association.relation_type == "association") {}
        else if (association.relation_type == "aggregation") { draw_aggregation_arrow(image, to.clone(), Direction::up);}
        else if (association.relation_type == "composition") {draw_composition_arrow(image, to.clone(), Direction::up)}
        else if (association.relation_type == "inheritance") { draw_inheritanceArrow(image, to.clone(), Direction::up);}
        else if (association.relation_type == "dependency") {drawArrow(image, to.clone(), Direction::up);}
    }
}

fn drawAssociation_dashed(image: &mut RgbImage, association: parser::Relationship, classBoxes: Vec<ClassBox>){
    let mut fromBox: ClassBox = ClassBox::new("".to_string(), Point::new(0,0), 0, 0, 0, 0);
    let mut toBox: ClassBox = ClassBox::new("".to_string(), Point::new(0,0), 0, 0, 0, 0);

    let num = rand::thread_rng().gen_range(0, 100);

    for classBox in classBoxes{
        if classBox.name == association.class.name{ fromBox = classBox.clone(); }
        if classBox.name == association.to_class.name{ toBox = classBox.clone(); }
    }

    let mut from = Point::new(fromBox.start.x +fromBox.box_width/2,
                                fromBox.start.y);
    let mut to = Point::new(toBox.start.x + toBox.box_width/2,
                                toBox.start.y + toBox.box_height);

    if(fromBox.start.y == toBox.start.y){
        from = Point::new(fromBox.start.x +fromBox.box_width/2 + num,
                                    fromBox.start.y + fromBox.box_height);
        to = Point::new(toBox.start.x + toBox.box_width/2 + num,
                                    toBox.start.y + toBox.box_height);

        if(fromBox.box_height > toBox.box_height){
            draw_dashed_line(image, Point::new(from.x, from.y),
                                    Point::new(from.x, from.y + 20));
            draw_dashed_line(image, Point::new(from.x, from.y + 20),
                                    Point::new(to.x, from.y + 20));
            draw_dashed_line(image, Point::new(to.x, from.y + 20),
                                    Point::new(to.x, to.y));
        }
        else{
            draw_dashed_line(image, Point::new(from.x, from.y),
                                    Point::new(from.x, to.y + 20));
            draw_dashed_line(image, Point::new(from.x, to.y + 20),
                                    Point::new(to.x, to.y + 20));
            draw_dashed_line(image, Point::new(to.x, to.y + 20),
                                    Point::new(to.x, to.y));
        }
        draw_inheritanceArrow(image, to.clone(), Direction::up);
    }
    else if (fromBox.start.y < toBox.start.y){
        from = Point::new(fromBox.start.x +fromBox.box_width/2 + num,
                                    fromBox.start.y + fromBox.box_height);
        to = Point::new(toBox.start.x + toBox.box_width/2 + num,
                                    toBox.start.y);
        draw_dashed_line(image, Point::new(from.x, from.y),
                                Point::new(from.x, from.y + num));
        draw_dashed_line(image, Point::new(from.x, from.y + num),
                                Point::new(to.x, from.y + num));
        draw_dashed_line(image, Point::new(to.x, from.y + num),
                                Point::new(to.x, to.y));
        draw_inheritanceArrow(image, to.clone(), Direction::down);
    }
    else if (fromBox.start.y > toBox.start.y){
        from = Point::new(fromBox.start.x + fromBox.box_width/2 + num,
                                    fromBox.start.y);
        to = Point::new(toBox.start.x + toBox.box_width/2 + num,
                                    toBox.start.y + toBox.box_height);
        draw_dashed_line(image, Point::new(from.x, from.y),
                                Point::new(from.x, from.y - num));
        draw_dashed_line(image, Point::new(from.x, from.y - num),
                                Point::new(to.x, from.y - num));
        draw_dashed_line(image, Point::new(to.x, from.y - num),
                                Point::new(to.x, to.y));
        draw_inheritanceArrow(image, to.clone(), Direction::up);

    }
}

fn draw_dashed_line(image: &mut RgbImage, mut from: Point, mut to: Point){
    let mut counter = 0.0;
    let mut to_right = false;let mut to_left = false; let mut up = false;let mut down = false;
    if(from.x < to.x) { to_right = true; }
    else if(from.x > to.x) { to_left = true; }
    else if(from.y < to.y) { up = true; }
    else if(from.y > to.y) { down = true; }

    if(to_right == true){
        while(from.x < to.x){
            draw_line_segment_mut(image, (from.x as f32, from.y as f32),
                                    (from.x as f32 +10.0, from.y as f32), Rgb([0u8, 0u8, 0u8]));
            from.x = from.x + 20;
        }
    }
    else if(to_left == true){
        while(from.x > to.x){
            draw_line_segment_mut(image, (from.x as f32, from.y as f32),
                                    (from.x as f32 - 10.0, from.y as f32), Rgb([0u8, 0u8, 0u8]));
            from.x = from.x - 20;
        }
    }
    else if(up == true){
        while(from.y < to.y){
            draw_line_segment_mut(image, (from.x as f32, from.y as f32),
                                    (from.x as f32, from.y as f32 + 10.0), Rgb([0u8, 0u8, 0u8]));
            from.y = from.y + 20;
        }
    }
    else if(down == true){
        while(from.y > to.y){
            draw_line_segment_mut(image, (from.x as f32, from.y as f32),
                                    (from.x as f32, from.y as f32 - 10.0), Rgb([0u8, 0u8, 0u8]));
            from.y = from.y - 20;
        }
    }
}

fn drawArrow(image: &mut RgbImage, point: Point, direction: Direction){
    match direction{
        Direction::up =>{
        draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                (point.x as f32 - 10.0, point.y as f32 + 15.0), Rgb([0u8, 0u8, 0u8]));
        draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                (point.x as f32 + 10.0, point.y as f32 + 15.0), Rgb([0u8, 0u8, 0u8]));
                            },
        Direction::down =>{
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 + 10.0, point.y as f32 - 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 - 10.0, point.y as f32 - 15.0), Rgb([0u8, 0u8, 0u8]));
                                },
        Direction::to_left =>{

        },
        Direction::to_right =>{

        },
    }
}

fn draw_inheritanceArrow(image: &mut RgbImage, point: Point, direction: Direction) {
    match direction{
        Direction::up =>{
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 - 10.0, point.y as f32 + 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 + 10.0, point.y as f32 + 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32 - 10.0, point.y as f32 + 15.0),
                                    (point.x as f32 + 10.0, point.y as f32 + 15.0), Rgb([0u8, 0u8, 0u8]));
                            },
        Direction::down =>{
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 + 10.0, point.y as f32 - 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 - 10.0, point.y as f32 - 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32 + 10.0, point.y as f32 - 15.0),
                                    (point.x as f32 - 10.0, point.y as f32 - 15.0), Rgb([0u8, 0u8, 0u8]));
                                },
        Direction::to_left =>{

        },
        Direction::to_right =>{

        },
    }
}

fn draw_aggregation_arrow(image: &mut RgbImage, point: Point, direction: Direction){
    match direction{
        Direction::up =>{
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 - 10.0, point.y as f32 + 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 + 10.0, point.y as f32 + 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32 - 10.0, point.y as f32 + 15.0),
                                    (point.x as f32, point.y as f32 + 30.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32 + 10.0, point.y as f32 + 15.0),
                                    (point.x as f32, point.y as f32 + 30.0), Rgb([0u8, 0u8, 0u8]));
                            },
        Direction::down =>{
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 + 10.0, point.y as f32 - 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32, point.y as f32),
                                    (point.x as f32 - 10.0, point.y as f32 - 15.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32 + 10.0, point.y as f32 - 15.0),
                                    (point.x as f32, point.y as f32 - 30.0), Rgb([0u8, 0u8, 0u8]));
            draw_line_segment_mut(image, (point.x as f32 - 10.0, point.y as f32 - 15.0),
                                    (point.x as f32, point.y as f32 - 30.0), Rgb([0u8, 0u8, 0u8]));
                                },
        Direction::to_left =>{

        },
        Direction::to_right =>{

        },
    }
}

fn draw_composition_arrow(image: &mut RgbImage, point: Point, direction: Direction){
    match direction{
        Direction::up =>{
            let mut point_x = point.x + 10;
            while point_x > point.x{
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
        Direction::down =>{
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
        Direction::to_left =>{

        },
        Direction::to_right =>{

        },
    }
}

fn drawClassBox(image: &mut RgbImage, class: parser::Class, x: &mut i32, y: &mut i32, width: u32, height: u32,
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

    let mut size = 16.0;
    let mut scale = Scale { x: size, y: size };
    let mut boxHeight = (attribute_len + method_len + 3) as u32 * size as u32;
    let mut textPosition_Y = 0;
    let mut maxCharacters = 0;
    let mut classBox: ClassBox;

    if maxCharacters < class.name.len()
    {
        maxCharacters = class.name.len();
    }

    if (column == 0) { *x = 50; }

    if (row > 0 && column == 0) { *y = *y + 300; }

    draw_text_mut(image, black, *x as u32 + 5, *y as u32 + 2, scale, &font, &class.name);
    //-----------------------------------Attributes------------------------------------------------
    for attribute in class.attributes
    {
        let attribute_line = attribute.visibility + &attribute.name + " : " + &attribute.data_type;
        if(maxCharacters < attribute_line.len())
        {
            maxCharacters = attribute_line.len();
        }
        textPosition_Y = *y as u32 + 10 + (counter as u32 * size as u32);

        draw_text_mut(image, black, *x as u32 + 5, textPosition_Y, scale, &font, &attribute_line);
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
        if maxCharacters < method_line.len()
        {
            maxCharacters = method_line.len();
        }
        textPosition_Y = *y as u32 + 10 + (counter as u32 * size as u32);
        draw_text_mut(image, black, *x as u32 + 5, textPosition_Y, scale, &font, &method_line);
        counter = counter +1;
    }
    //Generate the box
    let mut boxWidth = (size as u32 -6)*(maxCharacters as u32);
    draw_line_segment_mut(image, (*x as f32, *y as f32 + (size*(attribute_len as f32 + 2.0)) + 5.0),
                        (*x as f32 + boxWidth as f32, *y as f32 + size*(attribute_len as f32 + 2.0) + 5.0), black);
    draw_line_segment_mut(image, (*x as f32, *y as f32 + size + 5.0),
                                    (*x as f32 + boxWidth as f32, *y as f32 + size + 5.0), black);
    draw_hollow_rect_mut(image, Rect::at(*x,*y).of_size(boxWidth, boxHeight), black);

    classBox = ClassBox::new(class.name, Point::new(*x as u32,*y as u32), boxWidth, boxHeight, row, column);
    *x = boxWidth as i32 + *x + 50;
    //draw_hollow_rect_mut(&mut image, Rect::at(x, y).of_size(boxWidth, boxHeight), black);
    return classBox;
}


pub fn generateDiagram(relationships: Vec<parser::Relationship>,
    classes: Vec<parser::Class>, height: u32, width: u32, diagramName: &str) -> bool
{
    let mut boxes: Vec<ClassBox> = Vec::new();
    //Path of the diagram
    let path = Path::new("diagram.png");

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
    let mut scale = Scale { x: size, y: size };

    //White Background with title of the diagram
    draw_filled_rect_mut(&mut image, Rect::at(0,0).of_size(width,height), white);
    draw_text_mut(&mut image, black, (width/2)-(diagramName.len() as u32*size as u32/2), 10, scale, &font, &diagramName);

    let mut row = 0;
    let mut column = 0;

    for class in classes.clone() {
        //Generating the class box
        column = column + 1;
        if (column > 3){
            column = 0;
            row = row + 1;
        }
        boxes.push(drawClassBox(&mut image, class.clone(), &mut x, &mut y, width.clone(), height.clone(), row, column));
    }

    //-----------------Relationships------------------------------------------//
    for relationship in relationships.clone(){
        if(relationship.relation_type == "implementation"){
            drawAssociation_dashed(&mut image, relationship.clone(), boxes.clone());
        }
        else{
            drawAssociation(&mut image, relationship.clone(), boxes.clone());
        }
    }
    image.save(path).unwrap();
    return true;
}
