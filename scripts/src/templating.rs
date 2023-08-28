use std::{path::PathBuf, fs::{read_to_string, File, self}, io::Write};
use std::error::Error;
use handlebars::Handlebars;
use rand::seq::IteratorRandom;
use serde_json::{from_str, Value, json};


fn main() -> Result<(), Box<dyn Error>> {
    // reads files from themes/json and create all the color scheme files for alacritty i3 polybar ......


    // render without register
    let mut rng = rand::thread_rng();
    let files = fs::read_dir("themes/json/").unwrap();
    let file = files.choose(&mut rng).unwrap().unwrap();
    
    println!("{}", file.path().display());
    
    let json_values = read_scheme_json(&file.path()).unwrap();


    let _ = create_json(&json_values).unwrap();
    let _ = create_alacritty(&json_values).unwrap();
    let _ = create_rofi(&json_values).unwrap();
    let _ = create_polybar(&json_values).unwrap();
    Ok(())

}




fn create_json(s: &Value) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template = fs::read_to_string("templates/json.json").unwrap();
    let new_json = reg.render_template(
        &template,
        &json!({
            "color0": s["color0"],
            "color1": s["color1"],
            "color2": s["color2"],
            "color3": s["color3"],
            "color4": s["color4"],
            "color5": s["color5"],
            "color6": s["color6"],
            "color7": s["color7"],
            "color8": s["color8"],
            "color9": s["color9"],
            "color10": s["color10"],
            "color11": s["color11"],
            "color12": s["color12"],
            "color13": s["color13"],
            "color14": s["color14"],
            "color15": s["color15"],
            "background": s["background"],
            "foreground": s["foreground"],
            "cursor": s["cursor"],
        }),
    )?;
    let mut file = File::create("themes/active/active.json").unwrap();
    file.write_all(new_json.as_bytes()).unwrap();
    Ok(())
}





fn create_alacritty(s: &Value) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template = fs::read_to_string("templates/alacritty.yml").unwrap();
    let new_json = reg.render_template(
        &template,
        &json!({
            "color0": s["color0"],
            "color1": s["color1"],
            "color2": s["color2"],
            "color3": s["color3"],
            "color4": s["color4"],
            "color5": s["color5"],
            "color6": s["color6"],
            "color7": s["color7"],
            "color8": s["color8"],
            "color9": s["color9"],
            "color10": s["color10"],
            "color11": s["color11"],
            "color12": s["color12"],
            "color13": s["color13"],
            "color14": s["color14"],
            "color15": s["color15"],
            "background": s["background"],
            "foreground": s["foreground"],
            "cursor": s["cursor"],
        }),
    )?;
    let mut file = File::create("themes/active/alacritty.yml").unwrap();
    file.write_all(new_json.as_bytes()).unwrap();
    Ok(())
}





fn create_polybar(s: &Value) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template = fs::read_to_string("templates/colors.ini").unwrap();
    let new_json = reg.render_template(
        &template,
        &json!({
            "color0": s["color0"],
            "color1": s["color1"],
            "color2": s["color2"],
            "color3": s["color3"],
            "color4": s["color4"],
            "color5": s["color5"],
            "color6": s["color6"],
            "color7": s["color7"],
            "color8": s["color8"],
            "color9": s["color9"],
            "color10": s["color10"],
            "color11": s["color11"],
            "color12": s["color12"],
            "color13": s["color13"],
            "color14": s["color14"],
            "color15": s["color15"],
            "background": s["background"],
            "foreground": s["foreground"],
            "cursor": s["cursor"],
        }),
    )?;
    let mut file = File::create("themes/active/colors.ini").unwrap();
    file.write_all(new_json.as_bytes()).unwrap();
    Ok(())
}



fn create_rofi(s: &Value) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    let template = fs::read_to_string("templates/rofi.rasi").unwrap();
    let new_json = reg.render_template(
        &template,
        &json!({
            "color0": s["color0"],
            "color1": s["color1"],
            "color2": s["color2"],
            "color3": s["color3"],
            "color4": s["color4"],
            "color5": s["color5"],
            "color6": s["color6"],
            "color7": s["color7"],
            "color8": s["color8"],
            "color9": s["color9"],
            "color10": s["color10"],
            "color11": s["color11"],
            "color12": s["color12"],
            "color13": s["color13"],
            "color14": s["color14"],
            "color15": s["color15"],
            "background": s["background"],
            "foreground": s["foreground"],
            "cursor": s["cursor"],
        }),
    )?;
    let mut file = File::create("themes/active/rofi.rasi").unwrap();
    file.write_all(new_json.as_bytes()).unwrap();
    Ok(())
}



fn create_gtk() {}
fn create_i3() {}





fn read_scheme_json(path: &PathBuf) -> Result<Value, ()> {
    let binding = read_to_string(path).unwrap();
    let colors = binding.as_str();
    let json: Value = from_str(colors).expect("JSON was not well-formatted");
    Ok(json)
}
