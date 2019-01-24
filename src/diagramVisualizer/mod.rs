pub mod class;
pub mod package;
pub mod object;
pub mod usecase;

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


pub fn generate_package_diagram(models: Vec<parser::package::Model>, height: u32, width: u32) -> bool
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

    for model in models.clone() {
        draw_text_mut(&mut image, black, (width/2)-(model.name.len() as u32*size as u32/2), 10, scale, &font, &model.name);
        //Generating the class box
        column = column + 1;
        if column > 3 {
            column = 0;
            row = row + 1;
        }
        for package in model.packages.clone()
        {
            class_column = class_column + 1;
            if class_column > 3 {
                class_column = 0;
                class_row = class_row + 1;
            }
            package::draw_package_box(&mut image, package.clone(), &mut x, &mut y, row, column);
        }
    }

    //-----------------Relationships------------------------------------------//
    /*for relationship in relationships.clone(){
            draw_association_dashed(&mut image, relationship.clone(), boxes.clone());
    }*/
    image.save(path).unwrap();
    return true;
}

pub fn generate_usecase_diagram(systems: Vec<parser::use_case::System>,relations: Vec<parser::use_case::Relations>, height: u32, width: u32) -> bool
{
    //Path of the diagram
    let path = Path::new("usecase_diagram.png");

    //Used RGBs
    let white = Rgb([255u8, 255u8, 255u8]);
    let black = Rgb([0u8, 0u8, 0u8]);

    //Origin of the first class
    let mut x = 130;
    let mut y = 100;
    let mut ac_x = 50;
    let mut ac_y = 100;

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

    for system in systems.clone() {
        let system_size = (100,100);
        let mut cases = system.use_cases;
        let mut acteurs = system.akteurs;
        let mut case_vec : Vec<usecase::UseCase> = Vec::new();
        let mut ac_vec : Vec<usecase::Acteur> = Vec::new();

        let mut farthest_pos = 0;

        let height_factor = cases.len() / 3;
        draw_text_mut(&mut image, black, (width/2)-(system.name.len() as u32*size as u32/2), 10, scale, &font, &system.name);
        //Generating the class box
        column = column + 1;
        if column > 3 {
            column = 0;
            row = row + 1;
        }

        let mut class_row = 0;
        let mut class_column = 0;

        for case in cases.clone()
        {
            class_column = class_column + 1;
            if class_column > 3 {
                class_column = 0;
                class_row = class_row + 1;
            }
            let cusee = usecase::draw_usecase_box(&mut image, case.name, &mut x, &mut y, class_row, class_column);
            case_vec.push(cusee.clone());
            if cusee.center.x > farthest_pos {farthest_pos = cusee.center.x; }
        }
        for acteur in acteurs.clone()
        {
            let cusee = usecase::draw_acteur(&mut image, usecase::Point::new(ac_x, ac_y), &acteur.name);
            ac_vec.push(cusee.clone());
            ac_y = ac_y + 150;
        }
        draw_hollow_rect_mut(&mut image, Rect::at(100,80).of_size(farthest_pos + 100 ,100 * height_factor as u32), black);

        //-----------------Relationships------------------------------------------//
        for relationship in relations.clone(){
                for aa in relationship.akteur_akteur {
                    let mut from : usecase::Acteur = usecase::Acteur::new("".to_string(),usecase::Point::new(0,0), usecase::Point::new(0,0));
                    let mut to : usecase::Acteur = usecase::Acteur::new("".to_string(),usecase::Point::new(0,0), usecase::Point::new(0,0));
                    for a in ac_vec.clone() {
                        if  a.name == aa.akteur {
                            from = a.clone();
                        }
                        if a.name == aa.to_akteur {
                            to = a.clone();
                        }
                    }
                    usecase::draw_acteur_acteur_association(&mut image, from, to);
                }
                for cc in relationship.use_case_use_case {
                    let mut from : usecase::UseCase = usecase::UseCase::new("".to_string(),usecase::Point::new(0,0), 0, 0, 0);
                    let mut to : usecase::UseCase = usecase::UseCase::new("".to_string(),usecase::Point::new(0,0), 0, 0, 0);

                    for c in case_vec.clone() {
                        if  c.name == cc.use_case {
                            from = c.clone();
                        }
                        if c.name == cc.to_use_case {
                            to = c.clone();
                        }
                    }
                    usecase::draw_case_case_association(&mut image, from.clone(), to.clone(), cc.relation_type);
                    print!("{:?},{:?}\n", from.name, to.name);

                }
                for ac in relationship.akteur_use_case {
                    let mut from : usecase::Acteur = usecase::Acteur::new("".to_string(),usecase::Point::new(0,0), usecase::Point::new(0,0));
                    let mut to : usecase::UseCase = usecase::UseCase::new("".to_string(),usecase::Point::new(0,0), 0, 0, 0);

                    for a in ac_vec.clone() {
                        if a.name == ac.akteur {
                            from = a;
                        }
                    }
                    for c in case_vec.clone() {
                        if c.name == ac.to_use_case {
                            to = c;
                        }
                    }
                    usecase::draw_acteur_case_association(&mut image, from, to);
                }
        }
    }
    image.save(path).unwrap();
    return true;
}
