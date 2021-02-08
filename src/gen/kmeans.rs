extern crate rand;
extern crate image;

use rayon::prelude::*;
use rand::{distributions::WeightedIndex, prelude::*};
use image::GenericImageView;

pub fn nearest(color: &pastel::Lab, colors: &Vec<pastel::Lab>) -> (usize, f64) {
    return colors
        .iter()
        .map(|c| pastel::delta_e::cie76(color, c))
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(&b).expect("NaN encountered"))
        .unwrap();
}

// * Calculates Delta E(1994) between two colors
pub fn _cie94(color0: &pastel::Lab, color: &pastel::Lab) -> f64 {
    // for more info see: https://opentextbc.ca/graphicdesign/chapter/4-4-lab-colour-space-and-delta-e-measurements
    let xc1 = (color0.a.powi(2) + color0.b.powi(2)).sqrt();
    let xc2 = (color.a.powi(2) + color.b.powi(2)).sqrt();
    let xdl = color.l - color0.l;
    let mut xdc = xc2 - xc1;
    let xde = ( (color0.l - color.l).powi(2) + (color0.a - color.a).powi(2) + (color0.b - color.b).powi(2) ).sqrt();
    let mut xdh = xde.powi(2) - xdl.powi(2) - xdc.powi(2);
    xdh = if xdh > 0.0 { xdh.sqrt() } else { 0.0 };
    let xsc = 1.0 + 0.045 * xc1;
    let xsh = 1.0 + 0.015 * xc1;
    xdc = xdc / xsc;
    xdh = xdh / xsh;

    return ( xdl.powi(2) + xdc.powi(2) + xdh.powi(2) ).sqrt();
}


fn recalculate(colors: &Vec<&pastel::Lab>) -> pastel::Lab {
    let mut w_sum = 0.0;
    let (mut l, mut a, mut b) = (0.0, 0.0, 0.0);
    for col in colors.iter() {
        w_sum += 1.0;
        l += 1.0 * col.l;
        a += 1.0 * col.a;
        b += 1.0 * col.b;
    }

    pastel::Lab {
        l: l/w_sum,
        a: a/w_sum,
        b: b/w_sum,
        alpha: 1.0
    }
}

// * K-means++ clustering
pub fn palette(pixels: &Vec<pastel::Lab>, k: u8, max_iter: Option<u16>) -> Vec<(pastel::Lab, f32)> {
    const TOLERANCE: f64 = 1e-4;
    let mut rng = rand::thread_rng();

    // Randomly pick the starting cluster center
    let i: usize = rng.gen_range(0..pixels.len());
    let mut means: Vec<pastel::Lab> = vec![pixels[i].clone()];

    // Pick the remaining (k-1) means
    for _ in 0..(k - 1) {
        // Calculate the (nearest_distance)^2 for every color in the image
        let distances: Vec<f64> = pixels
            .par_iter()
            .map(|color| (nearest(&color, &means).1).powi(2))
            .collect();

        // Create a weighted distribution based on distance^2 -> if error, return the means
        let dist = match WeightedIndex::new(&distances) {
            Ok(t) => t,
            Err(_) => {
                // Calculate the dominance of each color
                let mut palette: Vec<(pastel::Lab, f32)> = means.iter().map(|c| (c.clone(), 0.0)).collect();
                let len = pixels.len() as f32;
                for color in pixels.iter() {
                    let near = nearest(&color, &means).0;
                    palette[near].1 += 1.0 / len;
                }
                return palette;
            }
        };

        // Pick a color and use it as a cluster center
        means.push(pixels[dist.sample(&mut rng)].clone());
    }

    let mut clusters: Vec<Vec<&pastel::Lab>>;
    let mut iters_left = max_iter.unwrap_or(300);
    loop {
        clusters = vec![Vec::new(); k as usize];
        for color in pixels.iter() {
            clusters[nearest(&color, &means).0].push(color);
        }
        let mut changed: bool = false;
        for i in 0..clusters.len() {
            let new_mean = recalculate(&clusters[i]);
            if pastel::delta_e::cie76(&means[i], &new_mean) > TOLERANCE {
                changed = true;
            }
            means[i] = new_mean;
        }
        iters_left -= 1;
        if !changed || iters_left <= 0 {
            break;
        }
    }

    // Length of each cluster divided by total pixels -> dominance of each mean
    return clusters
        .iter()
        .enumerate()
        .map(|(i, cluster)| {
            (means[i].clone(), cluster.len() as f32 / pixels.len() as f32)
        }).collect();
}



pub fn pigments(image_path: &str, count: u8, iters: Option<u16>) -> Result<Vec<(pastel::Lab, f32)>, Box<dyn std::error::Error>> {
    let mut img;
    img = image::open(image_path)?;
    img = img.resize(512, 512, image::imageops::FilterType::CatmullRom);

    let pixels: Vec<pastel::Lab> = img
        .pixels()
        .map(|(_, _, pix)| pastel::Color::from_rgba(pix[0], pix[1], pix[2], 1.0).to_lab())
        .collect();

    let mut output = palette(&pixels, count, iters);
    output.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    return Ok(output);
}
