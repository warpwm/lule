// Importing necessary crates and modules
use crate::scheme::*;
use rand::prelude::*;
use super::hex::color_from_hex;

// Helper function to generate the main six colors based on provided colors
pub fn gen_main_six(col: &[pastel::Color]) -> Vec<pastel::Color> {
    // Filtering colors based on lightness criteria
    let mut colors = col.to_owned();
    colors.retain(|x| x.to_lab().l > 20.0);
    colors.retain(|x| x.to_lab().l < 80.0);

    // Sorting and reversing colors based on lightness
    colors.sort_by_key(|c| (c.to_lch().l) as i32);
    colors.reverse();

    // Ensuring there are six colors by adding complementary colors
    let mut i = 0;
    while colors.len() < 6 {
        colors.push(pastel::Color::complementary(&colors[i]));
        i += 1;
    }

    // Selecting the top six colors and sorting them based on chroma
    let mut main_colors: Vec<pastel::Color> = Vec::new();
    for color in colors.iter().take(6) {
        main_colors.push(color.clone())
    }
    main_colors.sort_by_key(|c| (c.to_lch().c) as i32);
    main_colors.reverse();
    main_colors
}

// Helper function to get black and white colors mixed with a given color
pub fn get_black_white(
    ac: &pastel::Color,
    black_mix: f64,
    white_mix: f64,
    theme: bool,
) -> (pastel::Color, pastel::Color) {
    let black = pastel::Color::from_rgb(0, 0, 0);
    let white = pastel::Color::from_rgb(255, 255, 255);
    let dark = black.mix::<pastel::RGBA<f64>>(ac, pastel::Fraction::from(black_mix));
    let light = white.mix::<pastel::RGBA<f64>>(ac, pastel::Fraction::from(white_mix));
    // Choosing the appropriate order based on the theme
    if theme {
        (dark, light)
    } else {
        (light, dark)
    }
}

// Helper function to get two gray colors mixed with a given color
pub fn get_two_grays(ac: &pastel::Color, mix: f64, theme: bool) -> (pastel::Color, pastel::Color) {
    let darker = pastel::Color::from_rgb(100, 100, 100);
    let lighter = pastel::Color::from_rgb(170, 170, 170);
    let dark = darker.mix::<pastel::RGBA<f64>>(ac, pastel::Fraction::from(mix));
    let light = lighter.mix::<pastel::RGBA<f64>>(ac, pastel::Fraction::from(mix));
    // Choosing the appropriate order based on the theme
    if theme {
        (dark, light)
    } else {
        (light, dark)
    }
}

// Helper function to generate a set of colors based on a mix factor and theme
pub fn gen_prime_six(colors: Vec<pastel::Color>, mix: f64, theme: bool) -> Vec<pastel::Color> {
    let mut second_colors: Vec<pastel::Color> = Vec::new();

    for col in colors.iter() {
        // Lightening or darkening each color based on the theme
        let new_col = if theme {
            col.lighten(mix)
        } else {
            col.darken(mix)
        };
        second_colors.push(new_col)
    }
    second_colors
}

// Helper function similar to gen_prime_six but with a different condition
pub fn gen_second_six(colors: Vec<pastel::Color>, mix: f64, theme: bool) -> Vec<pastel::Color> {
    let mut second_colors: Vec<pastel::Color> = Vec::new();

    for col in colors.iter() {
        // Lightening or darkening each color based on the theme
        let new_col = if !theme {
            col.lighten(mix)
        } else {
            col.darken(mix)
        };
        second_colors.push(new_col)
    }
    second_colors
}

// Helper function to generate shades between given colors
pub fn gen_shades(colors: Vec<&pastel::Color>, number: u8) -> Vec<pastel::Color> {
    let mut color_scale = pastel::ColorScale::empty();
    let mut gradients: Vec<pastel::Color> = Vec::new();

    // Creating a color scale based on provided colors
    for (i, color) in colors.iter().enumerate() {
        let position = pastel::Fraction::from(i as f64 / (colors.len() as f64 - 1.0));
        color_scale.add_stop((*color).clone(), position);
    }

    // Generating shades between colors using the color scale
    let mix = Box::new(
        |c1: &pastel::Color, c2: &pastel::Color, f: pastel::Fraction| c1.mix::<pastel::Lab>(c2, f),
    );
    let count = number + 2;
    for i in 0..count {
        let position = pastel::Fraction::from(i as f64 / (count as f64 - 1.0));
        let color = color_scale.sample(position, &mix).expect("gradient color");
        // Skipping the first and last colors to avoid duplicates
        if i == 0 || i == count - 1 {
            continue;
        }
        gradients.push(color)
    }
    gradients
}

// Helper function to generate gradients using a set of colors
pub fn gen_gradients(
    ac: pastel::Color,
    col0: pastel::Color,
    col15: pastel::Color,
    black: pastel::Color,
    white: pastel::Color,
) -> Vec<pastel::Color> {
    let mut gradients: Vec<pastel::Color> = Vec::new();

    // Adding black as the starting color
    gradients.push(black.clone());
    // Generating shades between black and col0
    let blacks = gen_shades(vec![&black, &col0], 3);
    gradients.extend(blacks);
    // Generating shades between col0, ac, and col15
    let middle = gen_shades(vec![&col0, &ac, &col15], 16);
    gradients.extend(middle);
    // Generating shades between col15 and white
    let whites = gen_shades(vec![&col15, &white], 3);
    gradients.extend(whites);
    // Adding white as the ending color
    gradients.push(white.clone());

    gradients
}

// Function to get all colors based on the provided color scheme
pub fn get_all_colors(scheme: &mut Scheme) -> Vec<pastel::Color> {
    // Checking the theme of the color scheme
    let theme = scheme.theme().as_ref().unwrap_or(&"dark".to_string()) != "light";
    let mut palette: Vec<pastel::Color> = Vec::new();

    // Populating palette with colors from the scheme
    if let Some(ref cols) = scheme.pigments() {
        for c in cols.iter() {
            palette.push(color_from_hex(c));
        }
    }

    // Generating the main six colors
    let main = gen_main_six(&palette);

    // Initializing black and white colors based on the theme
    let mut black = pastel::Color::from_rgb(0, 0, 0);
    let mut white = pastel::Color::from_rgb(255, 255, 255);
    if !theme {
        white = pastel::Color::from_rgb(0, 0, 0);
        black = pastel::Color::from_rgb(255, 255, 255);
    }

    // Generating the prime colors based on main, with adjustments
    let prime = gen_prime_six(main.clone(), 0.1, theme);
    let acc = prime.get(0).unwrap().clone();

    // Generating two additional colors for shades
    let (col0, col15) = get_black_white(&acc, 0.08, 0.12, theme);
    let (col7, col8) = get_two_grays(&acc, 0.2, theme);

    // Generating the second set of six colors based on main, with adjustments
    let second = gen_second_six(main.clone(), 0.1, theme);

    // Generating gradients using various colors
    let gradients = gen_gradients(acc.clone(), col0.clone(), col15.clone(), black, white);

    // Combining all colors into a final vector
    let mut colors: Vec<pastel::Color> = Vec::new();
    colors.push(col0.clone());
    colors.extend(prime);
    colors.push(col7);
    colors.push(col8);
    colors.extend(second);
    colors.push(col15.clone());

    // Adding randomly generated colors to the mix
    for _ in 0..15 {
        let rng: &mut dyn RngCore = &mut thread_rng();
        let hue = rng.gen::<f64>() * 360.0;
        let saturation = 0.2 + 0.6 * rng.gen::<f64>();
        let lightness = 0.3 + 0.4 * rng.gen::<f64>();
        colors.extend(gen_shades(
            vec![
                &col0,
                &pastel::Color::from_hsl(hue, saturation, lightness),
                &col15,
            ],
            12,
        ));
    }
    let lightish = gradients[3].clone();
    let darkish = gradients[22].clone();

    colors.extend(gen_shades(
        vec![
            &lightish,
            &pastel::Color::from_rgb(255, 0, 0),
            &darkish,
        ],
        12,
    ));
    colors.extend(gen_shades(
        vec![
            &lightish,
            &pastel::Color::from_rgb(0, 255, 0),
            &darkish,
        ],
        12,
    ));
    colors.extend(gen_shades(
        vec![
            &lightish,
            &pastel::Color::from_rgb(0, 0, 255),
            &darkish,
        ],
        12,
    ));


    // Adding the generated gradients to the final color vector
    colors.extend(gradients);
    colors
}
