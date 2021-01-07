//! Process an image into a skybox
//!
//! This makes significant assumptions about the image.
//! * The skybox is a cube.
//! * The y-axis is up.
//! * The image provides a net for a cube in the same format as
//!   `assets/sky1.png`, ie. with the vertical sides in a strip
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
use bevy::render::mesh::VertexAttributeValues;
use image::{open, Rgb, RgbImage};
use itertools::Itertools;
use std::collections::HashMap;

/// `image` module errors.
#[derive(Debug, Clone, Copy)]
pub enum ImageError {
    FileNotFound,
    BackgroundNotDetermined,
    NetNotFound,
    NotAligned,
}

/// Create the `SkyboxBox` using settings from the `SkyboxPlugin`.
pub fn create_skybox(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    plugin: Res<crate::SkyboxPlugin>,
) {
    // Get the mesh for the image given.
    let mesh = get_mesh(&plugin.image).expect("Good image");
    // Load image as a texture asset.
    let texture_handle = asset_server.load(plugin.image.as_str());
    // Even before the texture is loaded we can updated the material.
    let mat_handle: Handle<StandardMaterial> = materials.add(texture_handle.into());
    let mat = materials.get_mut(mat_handle.clone()).expect("Material");
    mat.shaded = false;
    // Create the PbrBundle tagged as a skybox.
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: mat_handle,
            ..Default::default()
        })
        .with(crate::SkyboxBox);
}

/// Get the skybox mesh, including the uv values for the given texture
/// image. The box has unit edges is centred on the origin.
fn get_mesh(image: &str) -> Result<Mesh, ImageError> {
    let (fx, fy) = find_uv(image)?;
    // This relies on the particular face and vertex order of the
    // `shape::cube`.
    let mut mesh = Mesh::from(shape::Cube { size: -1.0 });
    let uv = VertexAttributeValues::Float2(vec![
        [fx[1], fy[1]],
        [fx[0], fy[1]],
        [fx[0], fy[2]],
        [fx[1], fy[2]],
        [fx[2], fy[2]],
        [fx[3], fy[2]],
        [fx[3], fy[1]],
        [fx[2], fy[1]],
        [fx[3], fy[1]],
        [fx[3], fy[2]],
        [fx[4], fy[2]],
        [fx[4], fy[1]],
        [fx[1], fy[1]],
        [fx[1], fy[2]],
        [fx[2], fy[2]],
        [fx[2], fy[1]],
        [fx[3], fy[2]],
        [fx[2], fy[2]],
        [fx[2], fy[3]],
        [fx[3], fy[3]],
        [fx[3], fy[0]],
        [fx[2], fy[0]],
        [fx[2], fy[1]],
        [fx[3], fy[1]],
    ]);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uv);
    Ok(mesh)
}

/// Find the dimensions of the skybox net in the image.
fn find_uv(image: &str) -> Result<(Vec<f32>, Vec<f32>), ImageError> {
    // Load the image for processing.
    let rgb: RgbImage = open(&format!("assets/{}", image))
        .map_err(|_| ImageError::FileNotFound)?
        .into_rgb8();
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
    if diff_x[3] - diff_x[0] > 8 {
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
    if diff_y[2] - diff_y[0] > 8 {
        return Err(ImageError::NotAligned);
    }

    // Return as fractions of whole image.
    let f_x = vec_x
        .iter()
        .map(|x| (*x as f32) / ((rgb.width() - 1) as f32))
        .collect::<Vec<f32>>();
    let f_y = vec_y
        .iter()
        .map(|y| (*y as f32) / ((rgb.height() - 1) as f32))
        .collect::<Vec<f32>>();
    Ok((f_x, f_y))
}

/// Search 8 points in the top and bottom sectors where we expect the background
/// in most points.
///
/// This is more complicated that is currently required, but might survive the losing of the
/// image requirements in the future.
fn find_background(rgb: &RgbImage) -> Result<Rgb<u8>, ImageError> {
    // Sample select points in the image likely to be background.
    let samples = (0..4)
        .cartesian_product(0..2)
        .map(|(x, y)| {
            rgb.get_pixel(
                (x * 2 + 1) * rgb.width() / 8,
                (y * 4 + 1) * rgb.height() / 6,
            )
        })
        .copied()
        .collect::<Vec<Rgb<u8>>>();

    // Find the most common background colour.
    let mut sample_freq = HashMap::<Rgb<u8>, usize>::new();
    for s in samples {
        *sample_freq.entry(s).or_insert(0) += 1;
    }
    let mut sample_hist = sample_freq.drain().collect::<Vec<(Rgb<u8>, usize)>>();
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
fn search_from_left(rgb: &RgbImage, bg: Rgb<u8>, y: u32) -> Result<u32, ImageError> {
    for x in 0..rgb.width() {
        if *rgb.get_pixel(x, y) != bg {
            return Ok(x);
        }
    }
    Err(ImageError::NetNotFound)
}

/// Search horizontally from the right to find the first non-background pixel.
fn search_from_right(rgb: &RgbImage, bg: Rgb<u8>, y: u32) -> Result<u32, ImageError> {
    for x in (0..rgb.width()).rev() {
        if *rgb.get_pixel(x, y) != bg {
            return Ok(x);
        }
    }
    Err(ImageError::NetNotFound)
}

/// Search vertically from the top to find the first non-background pixel.
fn search_from_top(rgb: &RgbImage, bg: Rgb<u8>, x: u32) -> Result<u32, ImageError> {
    for y in 0..rgb.height() {
        if *rgb.get_pixel(x, y) != bg {
            return Ok(y);
        }
    }
    Err(ImageError::NetNotFound)
}

/// Search vertically from the bottom to find the first non-background pixel.
fn search_from_bottom(rgb: &RgbImage, bg: Rgb<u8>, x: u32) -> Result<u32, ImageError> {
    for y in (0..rgb.height()).rev() {
        if *rgb.get_pixel(x, y) != bg {
            return Ok(y);
        }
    }
    Err(ImageError::NetNotFound)
}
