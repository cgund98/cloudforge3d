use std::{fs::File, path::PathBuf};

use gif::{Encoder, Frame, Repeat};
use regex::Regex;
use tauri::{AppHandle, Manager};

use crate::errors::AppError;

fn extract_frame_number(filename: &str) -> Option<i32> {
    let re = Regex::new(r"frame(\d+)\.preview\.webp").unwrap();
    if let Some(captures) = re.captures(filename) {
        if let Some(frame_str) = captures.get(1) {
            return frame_str.as_str().parse::<i32>().ok();
        }
    }
    None
}

pub fn get_job_thumbnails_path(handle: &AppHandle, job_id: &str) -> PathBuf {
    let data_dir = handle.path().app_data_dir().unwrap();
    data_dir.join("thumbnails/jobs").join(&job_id)
}

pub async fn generate_job_thumbnail_gif(
    job_path: PathBuf,
    frame_rate: i32,
) -> Result<bool, AppError> {
    // Generate list of valid frame files
    let mut frame_paths: Vec<(i32, PathBuf)> = Vec::new();

    let entries = job_path.read_dir()?;
    for entry in entries {
        let file = entry?;
        let file_path = file.path();
        let file_name = file.file_name().into_string().unwrap_or_default();

        let frame_number = extract_frame_number(&file_name);

        // Ensure this is a WEBP file
        if !file_path.is_file() || frame_number.is_none() {
            continue;
        }

        frame_paths.push((frame_number.unwrap(), file_path));
    }

    // Sort by frame number
    frame_paths.sort_by_key(|&(num, _)| num);

    if frame_paths.len() == 0 {
        log::warn!("Unable to generate thumbnail because no valid preview frames were found.");
        return Ok(false);
    }

    // Create an output file for the GIF
    let output_path = job_path.join("thumbnail.gif");
    let mut image_file = File::create(&output_path)?;

    // Read the first frame to get its dimensions
    let (_, first_frame) = &frame_paths[0];
    let first_img = image::open(first_frame)?.to_rgba8();
    let (width, height) = first_img.dimensions();

    // Calculate delay
    let frame_delay = (100.0 / frame_rate as f32) as u16;

    // Create a GIF encoder with repeat set to infinity (loops indefinitely)
    let mut encoder = Encoder::new(&mut image_file, width as u16, height as u16, &[])?;
    encoder.set_repeat(Repeat::Infinite)?;

    // Load each image, resize if needed, and add it to the GIF
    for (_frame_number, path) in frame_paths {
        // Open the image and ensure it is in RGBA format
        log::info!("Loading preview image...");
        let img = image::open(path)?.to_rgba8();
        log::info!("Encoding thumbnail frame...");

        let (width, height) = first_img.dimensions();

        // Create a GIF frame from the raw image data
        let mut frame =
            Frame::from_rgba_speed(width as u16, height as u16, &mut img.into_raw(), 30);
        frame.delay = frame_delay;
        encoder.write_frame(&frame)?;
    }

    log::info!("GIF created at {output_path:?}");

    Ok(true)
}
