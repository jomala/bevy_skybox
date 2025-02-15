//! Process an image into a skybox
//!
//! This makes significant assumptions about the image.
//! * The skybox is a cube.
//! * The y-axis is up.
//! * The image provides a net for a cube in the same format as
//!   `assets/sky1.png`, ie. with the vertical faces in a strip
//!   in the middle and the top and bottom above and below the
//!   third square from the left in the strip.
//! * The image doesn't have a specific "front" direction.
//! * It has an exact background colour outside the net, and that
//!   exact colour does not appear around the edge the net.
//! * The net is well-aligned with the image border.
//!
//! The image is searched by
//! * sampling 8 points, assuming that 6 at least will be the background colour
//! * searching the "equator" for the extremes of the net
//! * searching the "tropics" for the width of the top and bottom squares
//! * sampling "longitudinally" for the extremes.
//!
//! Many skybox images are available on the internet, but only
//! approximately meet the above criteria. This is usually sufficient
//! for demo quality however.
//!
//! Note that flipping an image (e.g. flipping a PNG image in "Paint" on
//! Windows) may not actually flip the underlying data read by this
//! module. Instead, you may need to copy the flipped image (in "Paint")
//! and then paste it into a new file.

use bevy::prelude::*;
use image::{
    DynamicImage, GenericImage, GenericImageView, ImageBuffer, ImageReader, Rgba, RgbaImage,
};
use itertools::Itertools;
use std::collections::HashMap;
use std::path::Path;

/// `image` module errors.
#[derive(Debug, Clone, Copy)]
pub enum ImageError {
    BadEnv,
    FileNotFound,
    DecodeFailed,
    BackgroundNotDetermined,
    NetNotFound,
    NotAligned,
    CopyError,
}

/// Get the skybox mesh, including the uv values for the given texture
/// image. The box has unit edges is centred on the origin.
pub fn get_skybox(image_name: &str) -> Result<Image, ImageError> {
    // Load the image for processing.
    let root_path = std::env::var_os("CARGO_MANIFEST_DIR").ok_or(ImageError::BadEnv)?;
    let path = Path::new(&root_path).join("assets").join(image_name);
    println!("Skybox path: {:?}", path);
    let reader = ImageReader::open(path).map_err(|e| {
        println!("Skybox load error: {:?}", e);
        ImageError::FileNotFound
    })?;
    let orig_image = reader.decode().map_err(|e| {
        println!("Skybox decode error: {:?}", e);
        ImageError::DecodeFailed
    })?;
    let orig_rgba = DynamicImage::ImageRgba8(orig_image.to_rgba8());
    let meas = ImageMeasurements::find_measurements(&orig_rgba)?;
    let shaped_image = meas.new_image(&orig_rgba)?;
    Ok(shaped_image)
}

/// `image` module measurements of positions in pixels.
///
/// See docs for the explanation of the indices.
pub struct ImageMeasurements {
    vec_x: Vec<u32>,
    vec_y: Vec<u32>,
}

impl ImageMeasurements {
    pub fn new_image(&self, old_image: &DynamicImage) -> Result<Image, ImageError> {
        let side = self.measure_side_length();
        let mut new_image = RgbaImage::new(side, side * 6);

        // +X
        self.copy_face(old_image, &mut new_image, side, 3, 1, 0)?;
        // -X
        self.copy_face(old_image, &mut new_image, side, 1, 1, 1)?;
        // +Y
        self.copy_face(old_image, &mut new_image, side, 2, 0, 2)?;
        // -Y
        self.copy_face(old_image, &mut new_image, side, 2, 2, 3)?;
        // +Z
        self.copy_face(old_image, &mut new_image, side, 2, 1, 4)?;
        // -Z
        self.copy_face(old_image, &mut new_image, side, 0, 1, 5)?;

        let image = Image::from_dynamic(
            image::DynamicImage::from(new_image),
            true,
            bevy::asset::RenderAssetUsages::all(),
        );
        Ok(image)
    }

    /// Copy a face as part of the new_image creation
    fn copy_face(
        &self,
        old_image: &DynamicImage,
        new_image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        side: u32,
        x_idx: usize,
        y_idx: usize,
        out_idx: usize,
    ) -> Result<(), ImageError> {
        let offset_x = (self.vec_x[x_idx + 1] - self.vec_x[x_idx] - side) / 2;
        let offset_y = (self.vec_y[y_idx + 1] - self.vec_y[y_idx] - side) / 2;
        new_image
            .copy_from(
                &old_image
                    .view(
                        self.vec_x[x_idx] + offset_x,
                        self.vec_y[y_idx] + offset_y,
                        side,
                        side,
                    )
                    .to_image(),
                0,
                side * (out_idx as u32),
            )
            .map_err(|_| ImageError::CopyError)
    }

    /// Find the dimensions of the skybox net in the image.
    pub fn find_measurements(rgb: &DynamicImage) -> Result<Self, ImageError> {
        // Find the background colour.
        let background = find_background(&rgb)?;
        // Measure the x values of the vertical edges of the net.
        let dy = rgb.height() / 6;
        let mid_x_min = search_from_left(&rgb, background, dy * 3)?;
        let mid_x_max = search_from_right(&rgb, background, dy * 3)?;
        let top_x_min = search_from_left(&rgb, background, dy * 1)?;
        let top_x_max = search_from_right(&rgb, background, dy * 1)?;
        let bot_x_min = search_from_left(&rgb, background, dy * 5)?;
        let bot_x_max = search_from_right(&rgb, background, dy * 5)?;
        if (top_x_min as i32 - bot_x_min as i32).abs() > 8 {
            return Err(ImageError::NotAligned);
        }
        if (top_x_max as i32 - bot_x_max as i32).abs() > 8 {
            return Err(ImageError::NotAligned);
        }
        let short_x_min = (top_x_min + bot_x_min) / 2;
        let short_x_max = (top_x_max + bot_x_max) / 2;
        // Assuming the shape, calculate the x values of the vertices and check them.
        let vec_x = vec![
            mid_x_min,
            (short_x_min + mid_x_min) / 2,
            short_x_min,
            short_x_max,
            mid_x_max,
        ];
        let mut diff_x = vec_x
            .as_slice()
            .windows(2)
            .map(|w| w[1] as i32 - w[0] as i32)
            .collect::<Vec<i32>>();
        diff_x.sort_unstable();
        if diff_x[3] - diff_x[0] > 16 {
            return Err(ImageError::NotAligned);
        }

        // Measure the y values of the horizontal edges of the net.
        let mid_y_min = search_from_top(&rgb, background, (vec_x[2] + vec_x[3]) / 2)?;
        let mid_y_max = search_from_bottom(&rgb, background, (vec_x[2] + vec_x[3]) / 2)?;
        let left_y_min = search_from_top(&rgb, background, vec_x[1])?;
        let left_y_max = search_from_bottom(&rgb, background, vec_x[1])?;
        let right_y_min = search_from_top(&rgb, background, (vec_x[3] + vec_x[4]) / 2)?;
        let right_y_max = search_from_bottom(&rgb, background, (vec_x[3] + vec_x[4]) / 2)?;
        if (left_y_min as i32 - right_y_min as i32).abs() > 8 {
            return Err(ImageError::NotAligned);
        }
        if (left_y_max as i32 - right_y_max as i32).abs() > 8 {
            return Err(ImageError::NotAligned);
        }
        let short_y_min = (left_y_min + right_y_min) / 2;
        let short_y_max = (left_y_max + right_y_max) / 2;

        // Assuming the shape, calculate the y values to return and check them.
        let vec_y = vec![mid_y_min, short_y_min, short_y_max, mid_y_max];
        let mut diff_y = vec_y
            .as_slice()
            .windows(2)
            .map(|w| w[1] as i32 - w[0] as i32)
            .collect::<Vec<i32>>();
        diff_y.sort_unstable();
        if diff_y[2] - diff_y[0] > 16 {
            return Err(ImageError::NotAligned);
        }

        // Pull in the borders. The matches won't be as good but maybe we can avoid some bad edges.
        // let adj = 2;
        // let vec_x = [vec_x[0] + adj, vec_x[1] + adj, vec_x[2] + adj, vec_x[3] - adj, vec_x[4] - adj];
        // let vec_y = [vec_y[0] + adj, vec_y[1] + adj, vec_y[2] - adj, vec_y[3] - adj];

        Ok(ImageMeasurements { vec_x, vec_y })
    }

    /// Determine the size of each image in the net, assuming that they all have to be the same
    /// so that we can copy pixel for pixel into the output without needing to scale.
    fn measure_side_length(&self) -> u32 {
        let min_x = self
            .vec_x
            .windows(2)
            .map(|x| x[1] - x[0])
            .min()
            .expect("Four x intervals");
        let min_y = self
            .vec_y
            .windows(2)
            .map(|y| y[1] - y[0])
            .min()
            .expect("Three y intervals");
        let side = min_x.min(min_y);
        println!("Skybox side length: {}", side);
        side
    }
}

/// Search 8 points in the top and bottom sectors where we expect the background
/// in most points.
///
/// This is more complicated that is currently required, but might survive the loosening of the
/// image requirements in the future.
pub fn find_background(rgb: &DynamicImage) -> Result<Rgba<u8>, ImageError> {
    // Sample select points in the image likely to be background.
    let samples = (0..4)
        .cartesian_product(0..2)
        .map(|(x, y)| {
            rgb.get_pixel(
                (x * 2 + 1) * rgb.width() / 8,
                (y * 4 + 1) * rgb.height() / 6,
            )
        })
        .collect::<Vec<Rgba<u8>>>();

    // Find the most common background colour.
    let mut sample_freq = HashMap::<Rgba<u8>, usize>::new();
    for s in samples {
        *sample_freq.entry(s).or_insert(0) += 1;
    }
    let mut sample_hist = sample_freq.drain().collect::<Vec<(Rgba<u8>, usize)>>();
    sample_hist.sort_by(|a, b| (a.1).cmp(&b.1));
    let background = sample_hist.iter().last().expect("Histogram");

    // At least half should be the background colour.
    if background.1 > 4 {
        Ok(background.0)
    } else {
        Err(ImageError::BackgroundNotDetermined)
    }
}

/// Search horizontally from the left to find the first non-background pixel.
pub fn search_from_left(rgb: &DynamicImage, bg: Rgba<u8>, y: u32) -> Result<u32, ImageError> {
    for x in 0..rgb.width() {
        if rgb.get_pixel(x, y) != bg {
            return Ok(x);
        }
    }
    Err(ImageError::NetNotFound)
}

/// Search horizontally from the right to find the last background pixel.
pub fn search_from_right(rgb: &DynamicImage, bg: Rgba<u8>, y: u32) -> Result<u32, ImageError> {
    for x in (0..rgb.width()).rev() {
        if rgb.get_pixel(x, y) != bg {
            return Ok(x + 1);
        }
    }
    Err(ImageError::NetNotFound)
}

/// Search vertically from the top to find the first non-background pixel.
pub fn search_from_top(rgb: &DynamicImage, bg: Rgba<u8>, x: u32) -> Result<u32, ImageError> {
    for y in 0..rgb.height() {
        if rgb.get_pixel(x, y) != bg {
            return Ok(y);
        }
    }
    Err(ImageError::NetNotFound)
}

/// Search vertically from the bottom to find the last background pixel.
pub fn search_from_bottom(rgb: &DynamicImage, bg: Rgba<u8>, x: u32) -> Result<u32, ImageError> {
    for y in (0..rgb.height()).rev() {
        if rgb.get_pixel(x, y) != bg {
            return Ok(y +  1);
        }
    }
    Err(ImageError::NetNotFound)
}
