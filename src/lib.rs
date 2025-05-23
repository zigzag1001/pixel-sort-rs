// extern crate wasm_bindgen;
// extern crate web_sys;

use std::collections::BTreeMap;

use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element, console};


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, World!");
}

fn console_log(s: &str) {
    console::log_1(&s.into());
}


// pixel sorting

fn bresenham_line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    let mut x = x0;
    let mut y = y0;
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    loop {
        points.push((x as usize, y as usize));
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
    points
}

fn get_pixel(data: &[u8], width: usize, x: usize, y: usize) -> [u8; 4] {
    let idx = (y * width + x) * 4;
    [
        data[idx],
        data[idx + 1],
        data[idx + 2],
        data[idx + 3],
    ]
}

fn set_pixel(data: &mut [u8], width: usize, x: usize, y: usize, pixel: [u8; 4]) {
    let idx = (y * width + x) * 4;
    data[idx] = pixel[0];
    data[idx + 1] = pixel[1];
    data[idx + 2] = pixel[2];
    data[idx + 3] = pixel[3];
}

#[wasm_bindgen]
pub fn calculate_pixel_value(r: u8, g: u8, b: u8, mode: &str) -> f64 {
    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;

    match mode {
        // luma, hue, saturation, red_green_ratio, blue_emphasis, luminance, euclidean_distance, cmy_cyan, cmy_magenta, cmy_yellow, xor, modulo
        "luma" => 0.299 * r + 0.587 * g + 0.114 * b,
        "hue" => {
            let max = r.max(g).max(b);
            let min = r.min(g).min(b);
            if max == min {
                0.0
            } else if max == r {
                ((g - b) / (max - min) + if g < b { 6.0 } else { 0.0 }) / 6.0
            } else if max == g {
                ((b - r) / (max - min) + 2.0) / 6.0
            } else {
                ((r - g) / (max - min) + 4.0) / 6.0
            }
        }
        "saturation" => {
            let max = r.max(g).max(b);
            let min = r.min(g).min(b);
            if max == 0.0 { 0.0 } else { (max - min) / max }
        }
        "red_green_ratio" => {
            if g > 0.0 { (r / g).min(1.0) } else { 1.0 }
        }
        "blue_emphasis" => {
            ((b - (r + g) / 2.0) + 1.0) / 2.0
        }
        "luminance" => {
            (0.2126 * r + 0.7152 * g + 0.0722 * b).min(1.0)
        }
        "euclidean_distance" => {
            ((r - 1.0).powi(2) + g.powi(2) + b.powi(2)).sqrt() / (3.0f64).sqrt()
        }
        "cmy_cyan" => {
            1.0 - r
        }
        "cmy_magenta" => {
            1.0 - g
        }
        "cmy_yellow" => {
            1.0 - b
        }
        "xor" => {
            (((r * 255.0) as u8 ^ (g * 255.0) as u8 ^ (b * 255.0) as u8) as f64) / 255.0
        }
        "modulo" => {
            ((r * 255.0 + g * 255.0 + b * 255.0) % 10.0) as f64 / 10.0
        }
        _ => 0.0,
    }
}

// #[wasm_bindgen]
// pub fn calculate_pixel_value(r: u8, g: u8, b: u8, mode: &str) -> f64 {
//     let r = r as f64 / 255.0;
//     let g = g as f64 / 255.0;
//     let b = b as f64 / 255.0;
//
//     match mode {
//         "luma" => 0.299 * r + 0.587 * g + 0.114 * b,
//         "saturation" => {
//             let max = r.max(g).max(b);
//             let min = r.min(g).min(b);
//             if max == 0.0 {
//                 0.0
//             } else {
//                 (max - min) / max
//             }
//         }
//         "hue" => {
//             let max = r.max(g).max(b);
//             let min = r.min(g).min(b);
//             let delta = max - min;
//
//             // if delta == 0.0 {
//             //     0.0
//             // } else if max == r {
//             //     (60.0 * ((g - b) / delta) + 360.0) % 360.0
//             // } else if max == g {
//             //     60.0 * ((b - r) / delta) + 120.0
//             // } else {
//             //     60.0 * ((r - g) / delta) + 240.0
//             // }
//             if delta == 0.0 {
//                 0.0
//             } else if max == r {
//                 (60.0 * ((g - b) / delta) % 360.0) / 360.0
//             } else if max == g {
//                 (60.0 * ((b - r) / delta) + 120.0) / 360.0
//             } else {
//                 (60.0 * ((r - g) / delta) + 240.0) / 360.0
//             }
//         }
//         "chroma" => {
//             let max = r.max(g).max(b);
//             let min = r.min(g).min(b);
//             max - min
//         }
//         "colorfulness" => {
//             let mean = (r + g + b) / 3.0;
//             ((r - mean).powi(2) + (g - mean).powi(2) + (b - mean).powi(2)).sqrt()
//         }
//         &_ => 0.0,
//     }
// }

// function to sort an array of pixels given a list of coordinates
// fn sort_array(data: &mut [u8], width: usize, height: usize, coords: Vec<(usize, usize)>) {
//     if coords.len() == 0 {
//         return;
//     } else if coords.len() == 1 {
//         return;
//     }
//
//     let mut pixels = Vec::new();
//     for (x, y) in coords.iter() {
//         pixels.push(get_pixel(data, width, *x, *y));
//     }
//     pixels.sort_by_key(|pixel| pixel[0]);
//
//     for ((x, y), pixel) in coords.iter().zip(pixels.iter()) {
//         set_pixel(data, width, *x, *y, *pixel);
//     }
// }
fn sort_array(data: &mut [u8], width: usize, height: usize, coords: Vec<(usize, usize)>, mode: &str) {
    if coords.len() == 0 {
        return;
    } else if coords.len() == 1 {
        return;
    }
    let mut pixels = Vec::new();
    for (x, y) in coords.iter() {
        let pixel = get_pixel(data, width, *x, *y);
        let value = calculate_pixel_value(pixel[0], pixel[1], pixel[2], mode);
        pixels.push((value, pixel));
    }
    pixels.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for ((x, y), (_, pixel)) in coords.iter().zip(pixels.iter()) {
        set_pixel(data, width, *x, *y, *pixel);
    }
}

// break up array based on brightness
// keep adding pixels to the current array until the brightness is less than the threshold
// then start a new array
fn break_array(data: &[u8], width: usize, height: usize, coords: Vec<(usize, usize)>, threshold: u16, invert: bool, mode: &str) -> Vec<Vec<(usize, usize)>> {
    let mut arrays = Vec::new();
    let mut current_array = Vec::new();
    if invert {
        for (x, y) in coords.iter() {
            let pixel = get_pixel(data, width, *x, *y);
            // let brightness = (pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16) / 3;
            let pixel_value = (calculate_pixel_value(pixel[0], pixel[1], pixel[2], mode) * 255.0) as u16;
            if pixel_value >= threshold {
                if current_array.len() > 0 {
                    arrays.push(current_array);
                    current_array = Vec::new();
                }
            } else {
                current_array.push((*x, *y));
            }
        }
    } else {
        for (x, y) in coords.iter() {
            let pixel = get_pixel(data, width, *x, *y);
            // let brightness = (pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16) / 3;
            let pixel_value = (calculate_pixel_value(pixel[0], pixel[1], pixel[2], mode) * 255.0) as u16;
            if pixel_value < threshold {
                if current_array.len() > 0 {
                    arrays.push(current_array);
                    current_array = Vec::new();
                }
            } else {
                current_array.push((*x, *y));
            }
        }
    }
    arrays.push(current_array);
    arrays
}

// Magic (i didnt do this)
// Rotates a grid of coordinates created from width and height by a given angle (in degrees).
// Returns a 2D vector of (x, y) tuples representing the rotated coordinates.
fn rotate_grid(width: usize, height: usize,angle: u16) -> Vec<Vec<(usize, usize)>> {
    if width == 0 || height == 0 {
        return Vec::new();
    }
    let mut arr = Vec::new();
    for y in 0..height {
        let mut row = Vec::new();
        for x in 0..width {
            row.push((x as usize, y as usize));
        }
        arr.push(row);
    }
    let angle = angle as f64;

    // Get dimensions.
    let height = arr.len();
    let width = arr.iter().map(|row| row.len()).max().unwrap_or(0);
    // Compute center of the array.
    let cx = (width as f64 - 1.0) / 2.0;
    let cy = (height as f64 - 1.0) / 2.0;

    // Convert angle to radians and compute cosine and sine.
    let rad = angle.to_radians();
    let cos_a = rad.cos();
    let sin_a = rad.sin();

    // Vertical scaling to form a diamond shape.
    // let scale_y = if sin_a.abs() > 1e-6 { 1.0 / sin_a.abs() } else { 1.0 };
    let scale_y = 1.0;

    // Collect rotated points as (x, y, value).
    let mut points = Vec::new();
    for (y, row) in arr.iter().enumerate() {
        for (x, &(a,b)) in row.iter().enumerate() {
            let dx = x as f64 - cx;
            let dy = y as f64 - cy;
            // Apply clockwise rotation.
            let rx = dx * cos_a + dy * sin_a;
            let mut ry = -dx * sin_a + dy * cos_a;
            ry *= scale_y;
            points.push((rx, ry, (a, b)));
        }
    }

    // Find the minimum x and y to shift points into non-negative space.
    let min_x = points.iter().map(|(x, _, _)| *x).fold(f64::INFINITY, f64::min);
    let min_y = points.iter().map(|(_, y, _)| *y).fold(f64::INFINITY, f64::min);

    // Shift points and round coordinates to the nearest integer.
    let shifted_points: Vec<(i32, i32, (usize, usize))> = points
        .into_iter()
        .map(|(x, y, value)| {
            let new_x = (x - min_x).round() as i32;
            let new_y = (y - min_y).round() as i32;
            (new_x, new_y, value)
        })
        .collect();

    // Group points by their y-coordinate.
    let mut rows: BTreeMap<i32, Vec<(i32, (usize, usize))>> = BTreeMap::new();
    for (x, y, value) in shifted_points {
        rows.entry(y).or_default().push((x, value));
    }

    // Sort each row by x and collect the diamond shape.
    let mut diamond = Vec::new();
    for (_y, mut pts) in rows {
        pts.sort_by_key(|&(x, _)| x);
        let row: Vec<(usize, usize)> = pts.into_iter().map(|(_x, value)| value).collect();
        diamond.push(row);
    }

    diamond
}

#[wasm_bindgen]
pub struct SortConfig {
    width: usize,
    height: usize,
    threshold: u16,
    angle: u16,
    invert: bool,
    mode: String,
    StackOutput: bool,
}

#[wasm_bindgen]
impl SortConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize, threshold: u16, angle: u16, invert: bool, mode: String, StackOutput: bool) -> Self {
        Self {
            width,
            height,
            threshold,
            angle,
            invert,
            mode,
            StackOutput
        }
    }
}

#[wasm_bindgen]
pub fn sort(data: &[u8], config: &SortConfig) -> Vec<u8> {
    let width = config.width;
    let height = config.height;
    let threshold = config.threshold;
    let angle = config.angle; // 0 - 360
    let invert = config.invert;
    let mode = &config.mode;
    let StackOutput = config.StackOutput;

    let mut finished = vec![0; data.len()]; // Create a new buffer

    finished.copy_from_slice(data); // Copy the original data into the new buffer

    let bresenham = rotate_grid(width, height, angle);
    for array in bresenham.iter() {
        let broken = break_array(&finished, width, height, array.clone(), threshold, invert, mode);
        for broken_array in broken.iter() {
            sort_array(&mut finished, width, height, broken_array.clone(), mode);
        }
    }

    finished
}
