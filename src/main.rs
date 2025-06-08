// Copyright (c) 2025 MALSHAN BANDARA [https://github.com/malshanb]
// SPDX-License-Identifier: MIT

use clap::{ArgGroup, Parser};
use image::{DynamicImage, GenericImageView, RgbaImage};
use std::fs;
use std::path::{PathBuf};
// use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// <=== Bulk Tagger ===> By Malshan Bandara
#[derive(Parser, Debug)]
#[command(author, version, about = "<=== Bulk Tagger ===> By Malshan Bandara.", long_about = None)]
#[command(group(ArgGroup::new("wm_rotate").args(["q", "e"]).multiple(false)))]
#[command(group(ArgGroup::new("final_rotate").args(["w", "r"]).multiple(false)))]
struct Args {
    /// Folder with original images
    input_folder: PathBuf,

    /// Folder to save watermarked images
    output_folder: PathBuf,

    /// Watermark image file path
    watermark: PathBuf,

    /// Rotate watermark 90° left
    #[arg(short = 'q')]
    q: bool,

    /// Rotate watermark 90° right
    #[arg(short = 'e')]
    e: bool,

    /// Rotate final image 90° left
    #[arg(short = 'w')]
    w: bool,

    /// Rotate final image 90° right
    #[arg(short = 'r')]
    r: bool,
}

fn rotate(image: DynamicImage, clockwise: bool) -> DynamicImage {
    if clockwise {
        image.rotate270()
    } else {
        image.rotate90()
    }
}

fn apply_watermark(
    base: DynamicImage,
    watermark: DynamicImage,
    wm_rotate: Option<bool>,
) -> RgbaImage {
    let (width, height) = base.dimensions();
    let mut wm = watermark.resize_exact(width, height, image::imageops::Lanczos3);

    if let Some(clockwise) = wm_rotate {
        wm = rotate(wm, clockwise);
        wm = wm.resize_exact(width, height, image::imageops::Lanczos3);
    }

    let mut base_rgba = base.to_rgba8();
    let wm_rgba = wm.to_rgba8();

    for (x, y, pixel) in base_rgba.enumerate_pixels_mut() {
        let wm_px = wm_rgba.get_pixel(x, y);
        let alpha = wm_px.0[3] as f32 / 255.0;

        for i in 0..3 {
            pixel.0[i] = ((1.0 - alpha) * pixel.0[i] as f32 + alpha * wm_px.0[i] as f32) as u8;
        }
    }

    base_rgba
}

fn main() {
    let args = Args::parse();

    fs::create_dir_all(&args.output_folder).expect("Failed to create output folder");

    let watermark_image =
        image::open(&args.watermark).expect("Failed to open watermark image");

    let wm_rotate = if args.q {
        Some(false)
    } else if args.e {
        Some(true)
    } else {
        None
    };

    let final_rotate = if args.w {
        Some(false)
    } else if args.r {
        Some(true)
    } else {
        None
    };

    for entry in WalkDir::new(&args.input_folder)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
        if !["jpg", "jpeg", "png"].contains(&ext.as_str()) {
            continue;
        }

        let base_img = image::open(path).expect("Failed to open input image");

        let watermarked = apply_watermark(base_img.clone(), watermark_image.clone(), wm_rotate);

        let final_image = if let Some(clockwise) = final_rotate {
            rotate(DynamicImage::ImageRgba8(watermarked), clockwise).to_rgb8()
        } else {
            DynamicImage::ImageRgba8(watermarked).to_rgb8()
        };

        let output_name = format!(
            "tagged-{}",
            path.file_name().unwrap().to_string_lossy()
        );

        let output_path = args.output_folder.join(output_name);
        final_image
            .save(output_path)
            .expect("Failed to save image");
        println!("===> Successfully Changed: {:?}", entry.file_name());
    }
}
