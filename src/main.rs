

use std::fs::File;
use std::io::{BufReader, BufRead};



fn main() {
    // change the colors.css files in ~/.config/gtk-3.0 and gtk-4.0

    let gtk3 = File::open("/home/ilyes/.config/gtk-3.0/colors.css").unwrap();
    let gtk4: File = File::open("/home/ilyes/.config/gtk-4.0/colors.css").unwrap();

    let reader3 = BufReader::new(gtk3);
    let reader4 = BufReader::new(gtk4);

    let mut old_colors15: Vec<String> = Vec::new(); 
    let mut old_text: Vec<String> = Vec::new(); 
    let mut colors_pushed = 0; 

    for line in reader3.lines() {
        let line = line.unwrap();
        if line.len() >= 13 {
            if &line[..13] == "@define-color"{
                let comps: Vec<&str> = line.split(" ").collect();
                let s = comps[2].replace("#", "").replace(";", "");
                if colors_pushed == 15 {
                    old_text.push(s)
                }else {
                    old_colors15.push(s);
                    colors_pushed += 1
                }
            }
        }
    }


    println!("{:?}", old_colors15);                
    println!("{:?}", old_text);                


    // randomize the pywal wallpaper and backend
}
