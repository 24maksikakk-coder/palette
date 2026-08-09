// palette.rs
use std::collections::HashMap;
use clap::{Arg, Command};
use std::str::FromStr;
use serde_json::{json, Value};

fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), String> {
    let s = hex.trim_start_matches('#');
    let s = if s.len() == 3 {
        s.chars().flat_map(|c| vec![c, c]).collect::<String>()
    } else {
        s.to_string()
    };
    if s.len() != 6 {
        return Err("Неверный HEX-формат".to_string());
    }
    let r = u8::from_str_radix(&s[0..2], 16).map_err(|_| "Ошибка парсинга R")?;
    let g = u8::from_str_radix(&s[2..4], 16).map_err(|_| "Ошибка парсинга G")?;
    let b = u8::from_str_radix(&s[4..6], 16).map_err(|_| "Ошибка парсинга B")?;
    Ok((r, g, b))
}

fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let (rf, gf, bf) = (r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0);
    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let l = (max + min) / 2.0;
    let (h, s) = if max == min {
        (0.0, 0.0)
    } else {
        let diff = max - min;
        let s = if l < 0.5 { diff / (max + min) } else { diff / (2.0 - max - min) };
        let h = if max == rf {
            (gf - bf) / diff + if gf < bf { 6.0 } else { 0.0 }
        } else if max == gf {
            (bf - rf) / diff + 2.0
        } else {
            (rf - gf) / diff + 4.0
        };
        (h / 6.0, s)
    };
    (h, s, l)
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let (r, g, b) = if s == 0.0 {
        (l, l, l)
    } else {
        let v2 = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
        let v1 = 2.0 * l - v2;
        let hue_to_rgb = |v1: f64, v2: f64, h: f64| {
            let h = if h < 0.0 { h + 1.0 } else if h > 1.0 { h - 1.0 } else { h };
            if 6.0 * h < 1.0 { v1 + (v2 - v1) * 6.0 * h }
            else if 2.0 * h < 1.0 { v2 }
            else if 3.0 * h < 2.0 { v1 + (v2 - v1) * (2.0 / 3.0 - h) * 6.0 }
            else { v1 }
        };
        (hue_to_rgb(v1, v2, h + 1.0 / 3.0),
         hue_to_rgb(v1, v2, h),
         hue_to_rgb(v1, v2, h - 1.0 / 3.0))
    };
    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

fn generate_palette(hex: &str) -> Result<(HashMap<String, String>, HashMap<String, String>, HashMap<String, String>), String> {
    let (r, g, b) = hex_to_rgb(hex)?;
    let (h, s, l) = rgb_to_hsl(r, g, b);

    let mut shades = HashMap::new();
    let lightness_levels = vec![0.90, 0.82, 0.74, 0.66, 0.58, 0.50, 0.42, 0.34, 0.26, 0.18];
    for (i, &lvl) in lightness_levels.iter().enumerate() {
        let name = 50 + i * 100;
        let (rr, gg, bb) = hsl_to_rgb(h, s, lvl);
        shades.insert(name.to_string(), rgb_to_hex(rr, gg, bb));
    }

    let mut accents = HashMap::new();
    let accent_lightness = vec![0.60, 0.70, 0.75, 0.80];
    let accent_saturation = vec![0.95, 0.90, 0.85, 0.80];
    let accent_names = vec!["A100", "A200", "A400", "A700"];
    for (i, &name) in accent_names.iter().enumerate() {
        let (rr, gg, bb) = hsl_to_rgb(h, accent_saturation[i], accent_lightness[i]);
        accents.insert(name.to_string(), rgb_to_hex(rr, gg, bb));
    }

    let mut contrast = HashMap::new();
    for (name, hex_c) in &shades {
        let (rr, gg, bb) = hex_to_rgb(hex_c)?;
        let lum = (0.299 * rr as f64 + 0.587 * gg as f64 + 0.114 * bb as f64) / 255.0;
        contrast.insert(name.clone(), if lum < 0.5 { "#ffffff".to_string() } else { "#000000".to_string() });
    }
    Ok((shades, accents, contrast))
}

fn print_palette(shades: &HashMap<String, String>, accents: &HashMap<String, String>, contrast: &HashMap<String, String>) {
    println!("\n--- Material Palette ---");
    for (name, hex_c) in shades {
        let (r, g, b) = hex_to_rgb(hex_c).unwrap();
        let txt = contrast.get(name).unwrap();
        let (tr, tg, tb) = hex_to_rgb(txt).unwrap();
        println!("\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m {:4} {} \x1b[0m", r, g, b, tr, tg, tb, name, hex_c);
    }
    println!("\n--- Accents ---");
    for (name, hex_c) in accents {
        let (r, g, b) = hex_to_rgb(hex_c).unwrap();
        println!("\x1b[48;2;{};{};{}m {:4} {} \x1b[0m", r, g, b, name, hex_c);
    }
}

fn main() {
    let matches = Command::new("palette")
        .version("1.0")
        .about("Material Palette Generator")
        .arg(Arg::new("color").help("Основной цвет (HEX)").required(true).index(1))
        .arg(Arg::new("json").long("json").help("Вывод в JSON"))
        .arg(Arg::new("css").long("css").help("Вывод CSS-переменных"))
        .arg(Arg::new("scss").long("scss").help("Вывод SCSS-переменных"))
        .get_matches();

    let color = matches.get_one::<String>("color").unwrap();
    let (shades, accents, contrast) = match generate_palette(color) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Ошибка: {}", e);
            std::process::exit(1);
        }
    };

    if matches.contains_id("json") {
        let palette = json!({
            "shades": shades,
            "accents": accents,
            "contrast": contrast
        });
        println!("{}", serde_json::to_string_pretty(&palette).unwrap());
    } else if matches.contains_id("css") {
        println!(":root {{");
        for (name, hex_c) in &shades {
            println!("  --md-{}: {};", name, hex_c);
        }
        for (name, hex_c) in &accents {
            println!("  --md-{}: {};", name, hex_c);
        }
        println!("}}");
    } else if matches.contains_id("scss") {
        for (name, hex_c) in &shades {
            println!("$md-{}: {};", name, hex_c);
        }
        for (name, hex_c) in &accents {
            println!("$md-{}: {};", name, hex_c);
        }
    } else {
        print_palette(&shades, &accents, &contrast);
    }
}
