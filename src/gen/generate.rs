use rand::prelude::*;
use crate::scheme::*;

pub fn gen_main_six(col: &Vec<pastel::Color>) -> Vec<pastel::Color> {
    let mut colors = col.clone();
    colors.retain(|x| x.to_lab().l > 20.0);
    colors.retain(|x| x.to_lab().l < 80.0);

    colors.sort_by_key(|c| (c.to_lch().l) as i32);
    colors.reverse();

    let mut i = 0;
    while colors.len() < 6 {
        colors.push(pastel::Color::complementary(&colors[i]));
        i = i +1;
    }

    let mut main_colors: Vec<pastel::Color> = Vec::new();
    for i in 0..6 {
        main_colors.push(colors[i].clone())
    }

    main_colors.sort_by_key(|c| (c.to_lch().c) as i32);
    main_colors.reverse();
    main_colors
}

pub fn get_black_white(ac: &pastel::Color, black_mix: f64, white_mix: f64, theme: bool) -> (pastel::Color, pastel::Color) {
    let black = pastel::Color::from_rgb(0,0,0);
    let white = pastel::Color::from_rgb(255,255,255);
    let dark = black.mix::<pastel::RGBA<f64>>(&ac, pastel::Fraction::from(black_mix));
    let light = white.mix::<pastel::RGBA<f64>>(&ac, pastel::Fraction::from(white_mix));
    if theme { (dark, light) } else { (light, dark) }
}

pub fn get_two_grays(ac: &pastel::Color, mix: f64, theme: bool) -> (pastel::Color, pastel::Color) {
    let darker = pastel::Color::from_rgb(100,100,100);
    let lighter = pastel::Color::from_rgb(170,170,170);
    let dark = darker.mix::<pastel::RGBA<f64>>(&ac, pastel::Fraction::from(mix));
    let light = lighter.mix::<pastel::RGBA<f64>>(&ac, pastel::Fraction::from(mix));
    if theme { (dark, light) } else { (light, dark) }
}

pub fn gen_prime_six(colors: Vec<pastel::Color>, mix: f64, theme: bool) -> Vec<pastel::Color> {
    let mut second_colors: Vec<pastel::Color> = Vec::new();

    for col in colors.iter() {
        let new_col = if theme { col.lighten(mix) } else { col.darken(mix) };
        second_colors.push(new_col)
    }
    second_colors
}

pub fn gen_second_six(colors: Vec<pastel::Color>, mix: f64, theme: bool) -> Vec<pastel::Color> {
    let mut second_colors: Vec<pastel::Color> = Vec::new();

    for col in colors.iter() {
        let new_col = if ! theme { col.lighten(mix) } else { col.darken(mix) };
        second_colors.push(new_col)
    }
    second_colors
}

pub fn gen_shades(colors: Vec<&pastel::Color>, number: u8) -> Vec<pastel::Color>{
    let mut color_scale = pastel::ColorScale::empty();
    let mut gradients: Vec<pastel::Color> = Vec::new();

    for (i, color) in colors.iter().enumerate() {
        let position = pastel::Fraction::from(i as f64 / (colors.len() as f64 - 1.0));
        color_scale.add_stop(color.clone().clone(), position);
    }

    let mix = Box::new(|c1: &pastel::Color, c2: &pastel::Color, f: pastel::Fraction| c1.mix::<pastel::Lab>(c2, f));
    let count = number + 2;
    for i in 0..count {
        let position = pastel::Fraction::from(i as f64 / (count as f64 - 1.0));
        let color = color_scale.sample(position, &mix).expect("gradient color");
        if i == 0 || i == count-1 { continue; }
        gradients.push(color) 
    }
    gradients
}

pub fn gen_gradients(ac: pastel::Color, col0: pastel::Color, col15: pastel::Color, black: pastel::Color, white: pastel::Color) -> Vec<pastel::Color> {
    let mut gradients: Vec<pastel::Color> = Vec::new();

    gradients.push(black.clone());
    let blacks = gen_shades(vec![&black, &col0], 3);
    gradients.extend(blacks);
    let middle = gen_shades(vec![&col0, &ac, &col15], 16);
    gradients.extend(middle);
    let whites = gen_shades(vec![&col15, &white], 3);
    gradients.extend(whites);
    gradients.push(white.clone());

    gradients
}

pub fn get_all_colors(scheme: &mut SCHEME) -> Vec<pastel::Color> {
    let theme = if scheme.theme().as_ref().unwrap_or(&"dark".to_string()) == "light" { false } else { true };
    let mut palette: Vec<pastel::Color> = Vec::new();
    if let Some(ref cols) = scheme.pigments() {
        for c in cols.iter() {
            palette.push(pastel::Color::from_hex(c));
        }
    }
    let main = gen_main_six(&palette);

    let mut black = pastel::Color::from_rgb(0,0,0);
    let mut white = pastel::Color::from_rgb(255,255,255);
    if !theme {
        white = pastel::Color::from_rgb(0,0,0);
        black = pastel::Color::from_rgb(255,255,255);
    }

    let prime = gen_prime_six(main.clone(), 0.1, theme);
    let acc = prime.get(0).unwrap().clone();
    let (col0, col15) = get_black_white(&acc, 0.08, 0.12, theme);
    let (col7, col8) = get_two_grays(&acc, 0.2, theme);
    let second = gen_second_six(main.clone(), 0.1, theme);

    let gradients = gen_gradients(acc.clone(), col0.clone(), col15.clone(), black, white);

    let mut colors: Vec<pastel::Color> = Vec::new();
    colors.push(col0.clone());
    colors.extend(prime);
    colors.push(col7);
    colors.push(col8);
    colors.extend(second);
    colors.push(col15.clone());


    for _ in 0..18 {
        let rng: &mut dyn RngCore = &mut thread_rng();
        let hue = rng.gen::<f64>() * 360.0;
        let saturation = 0.2 + 0.6 * rng.gen::<f64>();
        let lightness = 0.3 + 0.4 * rng.gen::<f64>();
        colors.extend(
            gen_shades(vec![&col0, &pastel::Color::from_hsl(hue, saturation, lightness), &col15], 12)
        );
    }

    colors.extend(gradients);
    colors
}
