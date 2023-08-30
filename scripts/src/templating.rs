use handlebars::Handlebars;
use rand::seq::IteratorRandom;
use serde_json::{from_str, json, Value};
use std::error::Error;
use std::path::Path;
use std::{
    fs::{self, read_to_string, File},
    io::Write,
    path::PathBuf,
};

pub fn template(theme_name: Option<String>) -> Result<(), Box<dyn Error>> {
    // reads files from themes/json and create all the color scheme files for alacritty i3 polybar ......
    let json_values = match theme_name {
        Some(name) => {
            read_scheme_json(&Path::new(&name)).unwrap()
        }
        None => {
            read_scheme_json(&Path::new(
                "/home/ilyes/setup/scripts/themes/active/active.json",
            ))
            .unwrap()
        }
    };
    let _ = create_json(&json_values).unwrap();
    let _ = create_alacritty(&json_values).unwrap();
    let _ = create_rofi(&json_values).unwrap();
    let _ = create_polybar(&json_values).unwrap();
    let _ = create_i3(&json_values).unwrap();
    let _ = create_i3_bar(&json_values).unwrap();

    // render without register

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

fn create_i3(s: &Value) -> Result<(), Box<dyn Error>>  {
    let reg = Handlebars::new();
    let template = fs::read_to_string("templates/colors").unwrap();
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
    let mut file = File::create("themes/active/colors").unwrap();
    file.write_all(new_json.as_bytes()).unwrap();
    Ok(())
}
fn create_i3_bar(s: &Value) -> Result<(), Box<dyn Error>>  {
    let reg = Handlebars::new();
    let template = fs::read_to_string("templates/bar_config").unwrap();
    let new_json = reg.render_template(
        &template,
        &json!({
            "color1": s["color1"],
            "color2": s["color2"],
            "color3": s["color3"],
        }),
    )?;
    let mut file = File::create("themes/active/bar_config").unwrap();
    file.write_all(new_json.as_bytes()).unwrap();
    Ok(())
}

fn read_scheme_json(path: &Path) -> Result<Value, ()> {
    let binding = read_to_string(&path).unwrap();
    let colors = binding.as_str();
    let json: Value = from_str(colors).expect("JSON was not well-formatted");
    Ok(json)
}

fn set_random_wallpaper(wallpaper: Option<&str>) {
    match wallpaper {
        Some(path) => {
            //set wall from path
        }
        None => {
            //set random wall
        }
    }
}
