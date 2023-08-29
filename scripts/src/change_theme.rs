use std::{env, path::Path, fs, process::Command, os::unix::prelude::OsStrExt};
use rand::seq::{SliceRandom, IteratorRandom};

const MESSAGE: &str = "
    --type=<pywal|custom>
        pywal:
            --backend=<wal|colorz|colorthief|random>
            --wallpaper=<image name in the path ~/Pictures/wallpapers | random>
        custom
            --theme_name=<name of theme in setup/scripts/json | random>
            --wallpaper=<image name in the path ~/Pictures/wallpapers | random>
";

fn main() {

    let args: Vec<_> = env::args().collect();
    if args.len() !=4 {
        println!("3 args are required type, backend/theme_name wallpaper");
        println!("{}", MESSAGE);
        std::process::exit(1)
    }
    
    if &args[1][..6] == "--type"{
        decide_process(args, 1);
    }else if &args[2][..6] == "--type"{
        decide_process(args, 2);
    }else if &args[3][..6] == "--type"{
        decide_process(args, 3);
    }else{
        println!("type needs to be specified");
        std::process::exit(1)
    }

    
}


fn decide_process(args: Vec<String>, index: usize) {
    println!("The first argument is {}", args[1]);
    println!("The second argument is {}", args[2]);
    println!("The third argument is {}", args[3]);
    if args[index] == "--type=pywal"{
        process_pywal(args);
    }else{
        process_custom(args);
    }
}

fn process_pywal(args: Vec<String>) {
    let backend = if &args[1][..9] == "--backend"{
        decide_backend(&args[1])
    }else if &args[2][..9] == "--backend"{
        decide_backend(&args[2])
    }else if &args[3][..9] == "--backend"{
        decide_backend(&args[3])
    }else{
        println!("backend not specified is required");
        std::process::exit(1)
    };


    let wallpaper_path = if &args[1][..11] == "--wallpaper"{
        decide_wallpaper(&args[1])
    }else if &args[2][..11] == "--wallpaper"{
        decide_wallpaper(&args[2])
    }else if &args[3][..11] == "--wallpaper"{
        decide_wallpaper(&args[3])
    }else{
        println!("wallpaper not specified is required");
        std::process::exit(1)
    };

    println!("for pywal selected wall:{} back:{}", wallpaper_path, backend);

    
 
    let output = Command::new(format!("wal --backend {} -i {} && /home/ilyes/setup/scripts/target/release/gtk_theme", backend, wallpaper_path))
        .output()
        .expect("Failed to execute command");
    println!("{:?}", output);
}





fn decide_backend(backend: &String) -> &'static str {
    let backend = backend.split("=").last().unwrap();
    if backend == "wal" {
        "wal"
    }else if backend == "colorz"{
        "colorz"
    }else if backend == "colorthief"{
        "colorthief"
    }else if backend == "random"{
        let arr =  vec!["colorz", "wal", "colorthief"];
        let random = arr.choose(&mut rand::thread_rng());
        random.unwrap()
    }else{
        println!("specified backend is not available required");
        println!("{}", MESSAGE);
        std::process::exit(1)
    }
}




fn decide_wallpaper(wallpaper: &String) -> String {
    let wallpaper = wallpaper.split("=").last().unwrap();
    if wallpaper == "random" {
        let mut rng = rand::thread_rng();
        let files = fs::read_dir("/home/ilyes/Pictures/wallpapers/").unwrap();
        let file = files.choose(&mut rng).unwrap().unwrap();
        
        String::from_utf8_lossy(
            &[
                b"/home/ilyes/Pictures/wallpapers/",
                file.file_name().as_bytes(),
            ]
            .concat(),
        )
        .to_string()
    }else{
        let wall_path: String = String::from_utf8_lossy(
            &[
                b"/home/ilyes/Pictures/wallpapers/",
                wallpaper.as_bytes(),
            ]
            .concat(),
        )
        .to_string();
        if Path::new(&wall_path).exists(){
        println!("************{}", wall_path);
        println!("************{}", wall_path);
        println!("************{}", wall_path);
        println!("************{}", wall_path);
        println!("************{}", wall_path);
        println!("************{}", wall_path);
        println!("************{}", wall_path);
        println!("************{}", wall_path);
        println!("************{}", wall_path);
        println!("************{}", wall_path);

            wall_path
        }else{
            println!("image names doesnt exist choose a valid image");
            std::process::exit(1)
        }
    }
}


fn process_custom(args: Vec<String>) {

}