pub mod class;
pub mod package;
pub mod object;

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

pub fn generate_class_diagram(relationships: Vec<parser::class::Relationship>,
    classes: Vec<parser::class::Class>, height: u32, width: u32, diagram_name: &str) -> bool
{
    let mut boxes: Vec<class::ClassBox> = Vec::new();
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
        boxes.push(class::drawclass_box(&mut image, class.clone(), &mut x, &mut y, width.clone(), height.clone(), row, column));
    }

    //-----------------Relationships------------------------------------------//
    for relationship in relationships.clone(){
        if relationship.relation_type == "implementation" {
            class::draw_association_dashed(&mut image, relationship.clone(), boxes.clone());
        }
        else {
            class::draw_association(&mut image, relationship.clone(), boxes.clone());
        }
    }
    image.save(path).unwrap();
    return true;
}

pub fn generate_object_diagram (relationships: Vec<parser::object::Link>,
    objects: Vec<parser::object::Object>, height: u32, width: u32, diagram_name: &str) -> bool
{
    let mut boxes: Vec<object::ObjectBox> = Vec::new();
    //Path of the diagram
    let path = Path::new("object_diagram.png");

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

    for object in objects.clone() {
        //Generating the class box
        column = column + 1;
        if column > 3 {
            column = 0;
            row = row + 1;
        }
        boxes.push(object::draw_object_box(&mut image, object.clone(), &mut x, &mut y, width.clone(), height.clone(), row, column));
    }

    //-----------------Relationships------------------------------------------//
    for relationship in relationships
    {
        object::draw_association(&mut image, relationship.clone(), boxes.clone());
    }
    image.save(path).unwrap();
    return true;
}


pub fn generate_package_diagram(packages: Vec<parser::package::Model>, height: u32, width: u32) -> bool
{
    let mut boxes: Vec<package::PackageBox> = Vec::new();
    //Path of the diagram
    let path = Path::new("package_diagram.png");

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

    let mut row = 0;
    let mut column = 0;
    let mut class_row = 0;
    let mut class_column = 0;

    for package in packages.clone() {
        draw_text_mut(&mut image, black, (width/2)-(package.name.len() as u32*size as u32/2), 10, scale, &font, &package.name);
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
            boxes.push(package::draw_package_box(&mut image, class.clone(), &mut x, &mut y, row, column));
        }
    }

    //-----------------Relationships------------------------------------------//
    /*for relationship in relationships.clone(){
            draw_association_dashed(&mut image, relationship.clone(), boxes.clone());
    }*/
    image.save(path).unwrap();
    return true;
}
