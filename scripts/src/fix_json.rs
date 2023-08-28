use handlebars::Handlebars;
use serde_json::json;
use std::error::Error;
use std::fs::{self, read_to_string};
use serde_json::{from_str, Value};
use std::fs::File;
use std::io::prelude::*;


fn main() {

    // jsons
    let paths = fs::read_dir("./jsons").unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }


    // propreties

    let paths = fs::read_dir("./propreties").unwrap();
    for path in paths {
        let gg = path.unwrap().path();
        println!("Name: {:?}", gg)
    }    

}





fn read_scheme() -> Result<Value, ()> {
    let binding = read_to_string("jsons/zenburn.json").unwrap();
    let colors = binding.as_str();
    let json: Value = from_str(colors).expect("JSON was not well-formatted");
    Ok(json)
}








fn create_json() -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    // render without register
    let template = fs::read_to_string("templates/json.json").unwrap();
    let json_values = read_scheme().unwrap();
    println!("{:?}", json_values["color0"]);
    
    let new_json = reg.render_template(&template, &json!({
        "color0": json_values["color_01"],
        "color1": json_values["color_02"],
        "color2": json_values["color_03"],
        "color3": json_values["color_04"],
        "color4": json_values["color_05"],
        "color5": json_values["color_06"],
        "color6": json_values["color_07"],
        "color7": json_values["color_08"],
        "color8": json_values["color_09"],
        "color9": json_values["color_10"],
        "color10": json_values["color_11"],
        "color11": json_values["color_12"],
        "color12": json_values["color_13"],
        "color13": json_values["color_14"],
        "color14": json_values["color_15"],
        "color15": json_values["color_16"],
        "background": json_values["background"],
        "foreground": json_values["foreground"],
        "cursor": json_values["cursor"],
    }))?;


    let mut file = File::create("themes/json/zenburn.json")?;
    file.write_all(new_json.as_bytes())?;

    Ok(())
}

