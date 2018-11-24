extern crate imageproc;
extern crate rusttype;
extern crate image;

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

pub fn generateDiagram(classes: Vec<parser::Class>, height: u32, width: u32, diagramName: &str) -> bool
{
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

    //Changing the font size
    size = 16.0;
    scale = Scale { x: size, y: size };

    for class in classes.clone() {
    //Generating the class box
    let mut counter = 1;

    let attribute_len = class.attributes.len();
    let method_len = class.methods.len();

    let mut boxHeight = (attribute_len + method_len + 3) as u32 * size as u32;
    let mut textPosition_Y = 0;
    let mut maxCharacters = 0;
    if maxCharacters < diagramName.len()
    {
        maxCharacters = class.name.len();
    }
    draw_text_mut(&mut image, black, x as u32 + 5, y as u32 + 2, scale, &font, &class.name);
    //-----------------------------------Attributes------------------------------------------------
    for attribute in class.attributes
    {
        let attribute_line = attribute.visibility + " " + &attribute.name + " : " + &attribute.data_type;
        if(maxCharacters < attribute_line.len())
        {
            maxCharacters = attribute_line.len();
        }
        textPosition_Y = y as u32 + 10 + (counter as u32 * size as u32);

        draw_text_mut(&mut image, black, x as u32 + 5, textPosition_Y, scale, &font, &attribute_line);
        counter = counter +1;
    }
    counter = counter +1;
    for method in class.methods
    {
        let mut parameters = String::new();

        for parameter in method.parameters {
            let parameter_line = parameter.name + " : " + &parameter.data_type;
            //TODO: Parameter werden nicht richtig an einen String angehängt
            parameters.push_str(&parameter_line);
            println!("{}",parameters );
        }
        let method_line = method.visibility + " " + &method.name + "(" + &parameters + ")";
        if maxCharacters < method.name.len()
        {
            maxCharacters = method.name.len();
        }
        textPosition_Y = y as u32 + 10 + (counter as u32 * size as u32);
        draw_text_mut(&mut image, black, x as u32 + 5, textPosition_Y, scale, &font, &method.name);
        counter = counter +1;
    }
    //Generate the box
    let mut boxWidth = (size as u32 -6)*(maxCharacters as u32);
    draw_line_segment_mut(&mut image, (x as f32, y as f32 + (size*(attribute_len as f32 + 2.0)) + 5.0), (x as f32 + boxWidth as f32, y as f32 + size*(attribute_len as f32 + 2.0) + 5.0), black);
    draw_line_segment_mut(&mut image, (x as f32, y as f32 + size + 5.0), (x as f32 + boxWidth as f32, y as f32 + size + 5.0), black);
    draw_hollow_rect_mut(&mut image, Rect::at(x, y).of_size(boxWidth, boxHeight), black);

    x=boxWidth as i32 + x + 100;
    if x > width as i32
    {
        x = 50;
        y = y + boxHeight as i32 + 100;
    }
    else
    {
        x = boxWidth as i32 + x + 100;
    }
    //draw_hollow_rect_mut(&mut image, Rect::at(x, y).of_size(boxWidth, boxHeight), black);
    }
    image.save(path).unwrap();
    return true;
}
