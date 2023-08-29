use array2d::Array2D;
use handlebars::Handlebars;
use rand::seq::{IteratorRandom, SliceRandom};
use serde_json::{from_str, json, Value};
use std::iter::FromIterator;
use std::{
    env,
    fs::{self, read_to_string, File},
    io::Write,
    os::unix::prelude::OsStrExt,
    path::Path,
    process::Command,
};
use tabled::{
    builder::Builder,
    settings::{object::Rows, split::Split, Border, Modify, Style},
    Table,
};

mod gtk_theme;
mod templating;

const MESSAGE: &str = "
    --type=<pywal|custom>
        pywal:
            --backend=<wal|colorz|colorthief|random>
            --wallpaper=<image name in the path ~/Pictures/wallpapers | random>
        custom
            --theme_name=<name of theme in setup/scripts/json | random>
            --wallpaper=<image name in the path ~/Pictures/wallpapers | random | none>
";

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        println!("3 args are required type, backend/theme_name wallpaper");
        println!("{}", MESSAGE);
        std::process::exit(1)
    }

    if &args[1][..6] == "--type" {
        decide_process(args, 1);
    } else if &args[2][..6] == "--type" {
        decide_process(args, 2);
    } else if &args[3][..6] == "--type" {
        decide_process(args, 3);
    } else {
        println!("type needs to be specified");
        std::process::exit(1)
    }
}

fn decide_process(args: Vec<String>, index: usize) {
    if args[index] == "--type=pywal" {
        process_pywal(args);
    } else if args[index] == "--type=custom" {
        process_custom(args);
    } else {
        println!("specifed wrong type");
        println!("{}", MESSAGE);
        std::process::exit(1)
    }
}

fn process_pywal(args: Vec<String>) {
    let backend = if &args[1][..9] == "--backend" {
        decide_backend(&args[1])
    } else if &args[2][..9] == "--backend" {
        decide_backend(&args[2])
    } else if &args[3][..9] == "--backend" {
        decide_backend(&args[3])
    } else {
        println!("backend not specified is required");
        std::process::exit(1)
    };

    let wallpaper_path = if &args[1][..11] == "--wallpaper" {
        decide_wallpaper(&args[1])
    } else if &args[2][..11] == "--wallpaper" {
        decide_wallpaper(&args[2])
    } else if &args[3][..11] == "--wallpaper" {
        decide_wallpaper(&args[3])
    } else {
        println!("wallpaper not specified is required");
        std::process::exit(1)
    };

    println!(
        "for pywal selected\n\twall:{}\n\tback:{}",
        wallpaper_path, backend
    );

    // apparantly putting the entire command in a string doesnt work yopu need to put each part of the command in an .arg
    //let output = Command::new(format!("wal --backend {} -i {} && /home/ilyes/setup/scripts/target/release/gtk_theme", backend, wallpaper_path))
    //let cmd = format!("wal --backend {} -i {} && /home/ilyes/setup/scripts/target/release/gtk_theme", backend, wallpaper_path);

    let output = Command::new("wal")
        //.arg(format!("--backend {}", backend))
        .arg("--backend")
        .arg(backend)
        .arg("-i")
        .arg(wallpaper_path)
        .output()
        .expect("Failed to execute command");

    println!("status: {}", output.status);
    println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    println!("setting the gtk themes\n");
    gtk_theme::main();
    //let output = Command::new("/home/ilyes/setup/scripts/target/release/gtk_theme")
    //    .output()
    //    .expect("Failed to execute command");
    //println!("status: {}", output.status);
    //println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    //println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    // here we do the templating with the aquired theme ~/.cache/wal/colors.json
    pywal_json_to_json();
    templating::template(None).unwrap();
}

fn decide_backend(backend: &String) -> &'static str {
    let backend = backend.split("=").last().unwrap();
    if backend == "wal" {
        "wal"
    } else if backend == "colorz" {
        "colorz"
    } else if backend == "colorthief" {
        "colorthief"
    } else if backend == "random" {
        let arr = vec!["colorz", "wal", "colorthief"];
        let random = arr.choose(&mut rand::thread_rng());
        random.unwrap()
    } else {
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
    } else {
        let wall_path: String = String::from_utf8_lossy(
            &[b"/home/ilyes/Pictures/wallpapers/", wallpaper.as_bytes()].concat(),
        )
        .to_string();
        if Path::new(&wall_path).exists() {
            wall_path
        } else {
            println!("image names doesnt exist choose a valid image");
            std::process::exit(1)
        }
    }
}

fn pywal_json_to_json() {
    let binding = read_to_string("/home/ilyes/.cache/wal/colors.json").unwrap();
    let colors = binding.as_str();
    let s: Value = from_str(colors).expect("JSON was not well-formatted");

    let reg = Handlebars::new();
    let template = fs::read_to_string("templates/json.json").unwrap();
    let new_json = reg
        .render_template(
            &template,
            &json!({
                "color0": s["colors"]["color0"],
                "color1": s["colors"]["color1"],
                "color2": s["colors"]["color2"],
                "color3": s["colors"]["color3"],
                "color4": s["colors"]["color4"],
                "color5": s["colors"]["color5"],
                "color6": s["colors"]["color6"],
                "color7": s["colors"]["color7"],
                "color8": s["colors"]["color8"],
                "color9": s["colors"]["color9"],
                "color10": s["colors"]["color10"],
                "color11": s["colors"]["color11"],
                "color12": s["colors"]["color12"],
                "color13": s["colors"]["color13"],
                "color14": s["colors"]["color14"],
                "color15": s["colors"]["color15"],
                "background": s["special"]["background"],
                "foreground": s["special"]["foreground"],
                "cursor": s["special"]["cursor"],
            }),
        )
        .unwrap();
    let mut file = File::create("themes/active/active.json").unwrap();
    file.write_all(new_json.as_bytes()).unwrap();
}

fn custom_wallpaper_selection(wallpaper: &String) -> Option<String> {
    let wall = wallpaper.split("=").last().unwrap();
    if wall == "none" {
        None
    } else {
        let gg = decide_wallpaper(wallpaper);
        Some(gg)
    }
}
fn decide_theme_name(theme_name: &str) -> String {
    if theme_name == "random" {
        //select random theme
        let mut rng = rand::thread_rng();
        let files = fs::read_dir("/home/ilyes/setup/scripts/themes/json/").unwrap();
        let file = files.choose(&mut rng).unwrap().unwrap();
        String::from_utf8_lossy(
            &[
                b"/home/ilyes/setup/scripts/themes/json/",
                file.file_name().as_bytes(),
            ]
            .concat(),
        )
        .to_string()
    } else {
        let theme = String::from_utf8_lossy(
            &[
                b"/home/ilyes/setup/scripts/themes/json/",
                theme_name.as_bytes(),
            ]
            .concat(),
        )
        .to_string();
        if Path::new(&theme).exists() {
            return theme;
        } else {
            print_table();
            println!("selected theme is unavaillable please select a valid theme");
            std::process::exit(1)
        }
    }
}

fn process_custom(args: Vec<String>) {
    let theme_name = if &args[1][..12] == "--theme_name" {
        decide_theme_name(args[1].split("=").last().unwrap())
    } else if &args[2][..12] == "--theme_name" {
        decide_theme_name(args[2].split("=").last().unwrap())
    } else if &args[3][..12] == "--theme_name" {
        decide_theme_name(args[3].split("=").last().unwrap())
    } else {
        println!("wallpaper not specified is required");
        std::process::exit(1)
    };

    let wallpaper_path: Option<String> = if &args[1][..11] == "--wallpaper" {
        custom_wallpaper_selection(&args[1])
    } else if &args[2][..11] == "--wallpaper" {
        custom_wallpaper_selection(&args[2])
    } else if &args[3][..11] == "--wallpaper" {
        custom_wallpaper_selection(&args[3])
    } else {
        println!("wallpaper not specified is required, will leave the current one");
        None
        //std::process::exit(1)
    };

    println!(
        "for custom selected\n\twall:{:?}\n\tback:{:?}",
        wallpaper_path, theme_name
    );

    //set themes here
    let gg = templating::template(Some(theme_name)).unwrap();
}

fn print_table() {
    const rows: usize = 78;
    const cols: usize = 7;
    let mut arr2d: [[&str; cols]; rows] = [[""; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            arr2d[i][j] = themes_names[i * cols + j];
        }
    }

    let mut builder = Builder::default();
    for line in themes_names.iter() {
        if line.is_empty() {
            continue;
        }

        let words: Vec<_> = line.trim().split_terminator(' ').collect();
        builder.push_record(words);
    }

    let columns = (0..builder.count_columns()).map(|i| i.to_string());
    builder.set_header(columns);

    let table = Table::new(arr2d).with(Style::extended()).to_string();

    println!("{}", table);
    println!("NOTE: seeing all themes requires fullscreen mode (cant get it to display according to term width (annoying))");
}

const themes_names: [&str; 546] = [
    "j_3024_day.json",
    "j_3024_night.json",
    "j_aci.json",
    "j_aco.json",
    "j_adventure_time.json",
    "j_afterglow.json",
    "j_alien_blood.json",
    "j_apprentice.json",
    "j_argonaut.json",
    "j_arthur.json",
    "j_atom.json",
    "j_aura.json",
    "j_ayu_dark.json",
    "j_ayu_light.json",
    "j_ayu_mirage.json",
    "j_azu.json",
    "j_belafonte_day.json",
    "j_belafonte_night.json",
    "j_bim.json",
    "j_birds_of_paradise.json",
    "j_blazer.json",
    "j_bluloco_light.json",
    "j_bluloco_zsh_light.json",
    "j_borland.json",
    "j_breath_darker.json",
    "j_breath_light.json",
    "j_breath_silverfox.json",
    "j_breath.json",
    "j_breeze.json",
    "j_broadcast.json",
    "j_brogrammer.json",
    "j_c64.json",
    "j_cai.json",
    "j_catppuccin_frappe.json",
    "j_catppuccin_latte.json",
    "j_catppuccin_macchiato.json",
    "j_catppuccin_mocha.json",
    "j_chalk.json",
    "j_chalkboard.json",
    "j_chameleon.json",
    "j_ciapre.json",
    "j_clone_of_ubuntu.json",
    "j_clrs.json",
    "j_cobalt_2.json",
    "j_cobalt_neon.json",
    "j_colorcli.json",
    "j_crayon_pony_fish.json",
    "j_dark_pastel.json",
    "j_darkside.json",
    "j_dehydration.json",
    "j_desert.json",
    "j_dimmed_monokai.json",
    "j_dissonance.json",
    "j_dracula.json",
    "j_earthsong.json",
    "j_elemental.json",
    "j_elementary.json",
    "j_elic.json",
    "j_elio.json",
    "j_espresso_libre.json",
    "j_espresso.json",
    "j_everblush.json",
    "j_everforest_dark.json",
    "j_everforest_light.json",
    "j_fairy_floss_dark.json",
    "j_fairy_floss.json",
    "j_fishtank.json",
    "j_flat_remix.json",
    "j_flat.json",
    "j_flatland.json",
    "j_foxnightly.json",
    "j_freya.json",
    "j_frontend_delight.json",
    "j_frontend_fun_forrest.json",
    "j_frontend_galaxy.json",
    "j_geohot.json",
    "j_github.json",
    "j_gogh.json",
    "j_gooey.json",
    "j_google_dark.json",
    "j_google_light.json",
    "j_gotham.json",
    "j_grape.json",
    "j_grass.json",
    "j_gruvbox_dark.json",
    "j_gruvbox_material.json",
    "j_gruvbox.json",
    "j_hardcore.json",
    "j_harper.json",
    "j_hemisu_dark.json",
    "j_hemisu_light.json",
    "j_highway.json",
    "j_hipster_green.json",
    "j_homebrew_light.json",
    "j_homebrew_ocean.json",
    "j_homebrew.json",
    "j_horizon_bright.json",
    "j_horizon_dark.json",
    "j_hurtado.json",
    "j_hybrid.json",
    "j_ibm_3270_high_contrast_.json",
    "j_ibm3270.json",
    "j_ic_green_ppl.json",
    "j_ic_orange_ppl.json",
    "j_idle_toes.json",
    "j_ir_black.json",
    "j_jackie_brown.json",
    "j_japanesque.json",
    "j_jellybeans.json",
    "j_jup.json",
    "j_kanagawa.json",
    "j_kibble.json",
    "j_kokuban.json",
    "j_laserwave.json",
    "j_later_this_evening.json",
    "j_lavandula.json",
    "j_liquid_carbon_transparent.json",
    "j_liquid_carbon.json",
    "j_lunaria_dark.json",
    "j_lunaria_eclipse.json",
    "j_lunaria_light.json",
    "j_maia.json",
    "j_man_page.json",
    "j_mar.json",
    "j_material.json",
    "j_mathias.json",
    "j_medallion.json",
    "j_misterioso.json",
    "j_molokai.json",
    "j_mona_lisa.json",
    "j_mono_amber.json",
    "j_mono_cyan.json",
    "j_mono_green.json",
    "j_mono_red.json",
    "j_mono_white.json",
    "j_mono_yellow.json",
    "j_monokai_dark.json",
    "j_monokai_pro_ristretto.json",
    "j_monokai_pro.json",
    "j_monokai_soda.json",
    "j_morada.json",
    "j_n0tch2k.json",
    "j_neon_night.json",
    "j_neopolitan.json",
    "j_nep.json",
    "j_neutron.json",
    "j_night_owl.json",
    "j_nightlion_v1.json",
    "j_nightlion_v2.json",
    "j_nighty.json",
    "j_nord_light.json",
    "j_nord.json",
    "j_novel.json",
    "j_obsidian.json",
    "j_ocean_dark.json",
    "j_oceanic_next.json",
    "j_ollie.json",
    "j_omni.json",
    "j_one_dark.json",
    "j_one_half_black.json",
    "j_one_light.json",
    "j_oxocarbon_dark.json",
    "j_palenight.json",
    "j_pali.json",
    "j_panda.json",
    "j_papercolor_dark.json",
    "j_papercolor_light.json",
    "j_paraiso_dark.json",
    "j_paul_millr.json",
    "j_pencil_dark.json",
    "j_pencil_light.json",
    "j_peppermint.json",
    "j_pixiefloss.json",
    "j_pnevma.json",
    "j_powershell.json",
    "j_predawn.json",
    "j_pro.json",
    "j_purple_people_eater.json",
    "j_red_alert.json",
    "j_red_sands.json",
    "j_relaxed.json",
    "j_rippedcasts.json",
    "j_rose_pine_dawn.json",
    "j_rose_pine_moon.json",
    "j_rose_pine.json",
    "j_royal.json",
    "j_sat.json",
    "j_sea_shells.json",
    "j_seafoam_pastel.json",
    "j_seti.json",
    "j_shaman.json",
    "j_shel.json",
    "j_slate.json",
    "j_smyck.json",
    "j_snazzy.json",
    "j_soft_server.json",
    "j_solarized_darcula.json",
    "j_solarized_dark_higher_contrast.json",
    "j_solarized_dark.json",
    "j_solarized_light.json",
    "j_sonokai.json",
    "j_spacedust.json",
    "j_spacegray_eighties_dull.json",
    "j_spacegray_eighties.json",
    "j_spacegray.json",
    "j_spring.json",
    "j_square.json",
    "j_srcery.json",
    "j_summer_pop.json",
    "j_sundried.json",
    "j_sweet_eliverlara.json",
    "j_sweet_terminal.json",
    "j_symphonic.json",
    "j_synthwave_alpha.json",
    "j_synthwave.json",
    "j_teerb.json",
    "j_tender.json",
    "j_terminal_basic.json",
    "j_terminix_dark.json",
    "j_thayer_bright.json",
    "j_tin.json",
    "j_tokyo_night_light.json",
    "j_tokyo_night_storm.json",
    "j_tokyo_night.json",
    "j_tomorrow_night_blue.json",
    "j_tomorrow_night_bright.json",
    "j_tomorrow_night_eighties.json",
    "j_tomorrow_night.json",
    "j_tomorrow.json",
    "j_toy_chest.json",
    "j_treehouse.json",
    "j_twilight.json",
    "j_ura.json",
    "j_urple.json",
    "j_vag.json",
    "j_vaughn.json",
    "j_vibrant_ink.json",
    "j_vs_code_dark_.json",
    "j_vs_code_light_.json",
    "j_warm_neon.json",
    "j_wez.json",
    "j_wild_cherry.json",
    "j_wombat.json",
    "j_wryan.json",
    "j_wzoreck.json",
    "j_zenburn.json",
    "p_3024_day.json",
    "p_3024_night.json",
    "p_abernathy.json",
    "p_adventure.json",
    "p_adventuretime.json",
    "p_afterglow.json",
    "p_alabaster.json",
    "p_alienblood.json",
    "p_andromeda.json",
    "p_apple_classic.json",
    "p_arcoiris.json",
    "p_argonaut.json",
    "p_arthur.json",
    "p_ateliersulphurpool.json",
    "p_atom.json",
    "p_atomonelight.json",
    "p_aurora.json",
    "p_ayu_light.json",
    "p_ayu_mirage.json",
    "p_ayu.json",
    "p_banana_blueberry.json",
    "p_batman.json",
    "p_belafonte_day.json",
    "p_belafonte_night.json",
    "p_birdsofparadise.json",
    "p_blazer.json",
    "p_blue_matrix.json",
    "p_blueberrypie.json",
    "p_bluedolphin.json",
    "p_blulocodark.json",
    "p_blulocolight.json",
    "p_borland.json",
    "p_breeze.json",
    "p_bright_lights.json",
    "p_broadcast.json",
    "p_brogrammer.json",
    "p_builtin_dark.json",
    "p_builtin_light.json",
    "p_builtin_pastel_dark.json",
    "p_builtin_solarized_dark.json",
    "p_builtin_solarized_light.json",
    "p_builtin_tango_dark.json",
    "p_builtin_tango_light.json",
    "p_c64.json",
    "p_calamity.json",
    "p_catppuccin-frappe.json",
    "p_catppuccin-latte.json",
    "p_catppuccin-macchiato.json",
    "p_catppuccin-mocha.json",
    "p_cga.json",
    "p_chalk.json",
    "p_chalkboard.json",
    "p_challengerdeep.json",
    "p_chester.json",
    "p_ciapre.json",
    "p_clrs.json",
    "p_cobalt_neon.json",
    "p_cobalt2.json",
    "p_coffee_theme.json",
    "p_crayonponyfish.json",
    "p_cyberdyne.json",
    "p_cyberpunk.json",
    "p_dark_pastel.json",
    "p_dark+.json",
    "p_darkermatrix.json",
    "p_darkmatrix.json",
    "p_darkside.json",
    "p_deep.json",
    "p_desert.json",
    "p_dimmedmonokai.json",
    "p_django.json",
    "p_djangorebornagain.json",
    "p_djangosmooth.json",
    "p_doom_peacock.json",
    "p_doomone.json",
    "p_dotgov.json",
    "p_dracula.json",
    "p_dracula+.json",
    "p_duckbones.json",
    "p_duotone_dark.json",
    "p_earthsong.json",
    "p_elemental.json",
    "p_elementary.json",
    "p_encom.json",
    "p_espresso_libre.json",
    "p_espresso.json",
    "p_everblush.json",
    "p_fahrenheit.json",
    "p_fairyfloss.json",
    "p_fideloper.json",
    "p_firefoxdev.json",
    "p_firewatch.json",
    "p_fishtank.json",
    "p_flat.json",
    "p_flatland.json",
    "p_floraverse.json",
    "p_forestblue.json",
    "p_framer.json",
    "p_frontenddelight.json",
    "p_funforrest.json",
    "p_galaxy.json",
    "p_galizur.json",
    "p_github_dark.json",
    "p_github.json",
    "p_glacier.json",
    "p_grape.json",
    "p_grass.json",
    "p_grey-green.json",
    "p_gruvbox_light.json",
    "p_gruvboxdark.json",
    "p_gruvboxdarkhard.json",
    "p_guezwhoz.json",
    "p_hacktober.json",
    "p_hardcore.json",
    "p_harper.json",
    "p_hax0r_blue.json",
    "p_hax0r_gr33n.json",
    "p_hax0r_r3d.json",
    "p_highway.json",
    "p_hipster_green.json",
    "p_hivacruz.json",
    "p_homebrew.json",
    "p_hopscotch.256.json",
    "p_hopscotch.json",
    "p_hurtado.json",
    "p_hybrid.json",
    "p_ic_green_ppl.json",
    "p_ic_orange_ppl.json",
    "p_iceberg-dark.json",
    "p_iceberg-light.json",
    "p_idea.json",
    "p_idletoes.json",
    "p_ir_black.json",
    "p_jackie_brown.json",
    "p_japanesque.json",
    "p_jellybeans.json",
    "p_jetbrains_darcula.json",
    "p_jubi.json",
    "p_kanagawabones.json",
    "p_kibble.json",
    "p_kolorit.json",
    "p_konsolas.json",
    "p_lab_fox.json",
    "p_laser.json",
    "p_later_this_evening.json",
    "p_lavandula.json",
    "p_liquidcarbon.json",
    "p_liquidcarbontransparent.json",
    "p_liquidcarbontransparentinverse.json",
    "p_lovelace.json",
    "p_man_page.json",
    "p_mariana.json",
    "p_material.json",
    "p_materialdark.json",
    "p_materialdarker.json",
    "p_materialdesigncolors.json",
    "p_materialocean.json",
    "p_mathias.json",
    "p_matrix.json",
    "p_medallion.json",
    "p_midnight-in-mojave.json",
    "p_mirage.json",
    "p_misterioso.json",
    "p_molokai.json",
    "p_monalisa.json",
    "p_monokai_remastered.json",
    "p_monokai_soda.json",
    "p_monokai_vivid.json",
    "p_n0tch2k.json",
    "p_neobones_dark.json",
    "p_neobones_light.json",
    "p_neon.json",
    "p_neopolitan.json",
    "p_neutron.json",
    "p_night_owlish_light.json",
    "p_nightlion_v1.json",
    "p_nightlion_v2.json",
    "p_niji.json",
    "p_nocturnal_winter.json",
    "p_nord-light.json",
    "p_nord.json",
    "p_novel.json",
    "p_obsidian.json",
    "p_ocean.json",
    "p_oceanic-next.json",
    "p_oceanicmaterial.json",
    "p_ollie.json",
    "p_onehalfdark.json",
    "p_onehalflight.json",
    "p_operator_mono_dark.json",
    "p_overnight_slumber.json",
    "p_palenighthc.json",
    "p_pandora.json",
    "p_paraiso_dark.json",
    "p_paulmillr.json",
    "p_pencildark.json",
    "p_pencillight.json",
    "p_peppermint.json",
    "p_piatto_light.json",
    "p_pnevma.json",
    "p_popping_and_locking.json",
    "p_primary.json",
    "p_pro_light.json",
    "p_pro.json",
    "p_purple_rain.json",
    "p_purplepeter.json",
    "p_rapture.json",
    "p_raycast_dark.json",
    "p_raycast_light.json",
    "p_rebecca.json",
    "p_red_alert.json",
    "p_red_planet.json",
    "p_red_sands.json",
    "p_relaxed.json",
    "p_retro.json",
    "p_rippedcasts.json",
    "p_rose-pine-dawn.json",
    "p_rose-pine-moon.json",
    "p_rose-pine.json",
    "p_rouge_2.json",
    "p_royal.json",
    "p_ryuuko.json",
    "p_sakura.json",
    "p_scarlet_protocol.json",
    "p_seafoam_pastel.json",
    "p_seashells.json",
    "p_seoulbones_dark.json",
    "p_seoulbones_light.json",
    "p_seti.json",
    "p_shades-of-purple.json",
    "p_shaman.json",
    "p_slate.json",
    "p_sleepyhollow.json",
    "p_smyck.json",
    "p_snazzy.json",
    "p_softserver.json",
    "p_solarized_darcula.json",
    "p_solarized_dark_-_patched.json",
    "p_solarized_dark_higher_contrast.json",
    "p_spacedust.json",
    "p_spacegray_eighties_dull.json",
    "p_spacegray_eighties.json",
    "p_spacegray.json",
    "p_spiderman.json",
    "p_spring.json",
    "p_square.json",
    "p_sublette.json",
    "p_subliminal.json",
    "p_sundried.json",
    "p_symfonic.json",
    "p_synthwave-everything.json",
    "p_synthwave.json",
    "p_synthwavealpha.json",
    "p_tango_adapted.json",
    "p_tango_half_adapted.json",
    "p_teerb.json",
    "p_terminal_basic.json",
    "p_thayer_bright.json",
    "p_the_hulk.json",
    "p_tinacious_design_(dark).json",
    "p_tinacious_design_(light).json",
    "p_tokyonight-day.json",
    "p_tokyonight-storm.json",
    "p_tokyonight.json",
    "p_tomorrow_night_blue.json",
    "p_tomorrow_night_bright.json",
    "p_tomorrow_night_burns.json",
    "p_tomorrow_night_eighties.json",
    "p_tomorrow_night.json",
    "p_tomorrow.json",
    "p_toychest.json",
    "p_treehouse.json",
    "p_twilight.json",
    "p_ubuntu.json",
    "p_ultradark.json",
    "p_ultraviolent.json",
    "p_underthesea.json",
    "p_unikitty.json",
    "p_urple.json",
    "p_vaughn.json",
    "p_vibrantink.json",
    "p_vimbones.json",
    "p_violet_dark.json",
    "p_violet_light.json",
    "p_warmneon.json",
    "p_wez.json",
    "p_whimsy.json",
    "p_wildcherry.json",
    "p_wilmersdorf.json",
    "p_wombat.json",
    "p_wryan.json",
    "p_zenbones_dark.json",
    "p_zenbones_light.json",
    "p_zenbones.json",
    "p_zenburn.json",
    "p_zenburned.json",
    "p_zenwritten_dark.json",
    "p_zenwritten_light.json",
    "",
    "",
];
