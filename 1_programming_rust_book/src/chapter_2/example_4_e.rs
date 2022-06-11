/// Writing Image Files
extern crate image;

use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file and `filename`
fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let encode = PNGEncoder::new(output);
    encode.encode(
        &pixels, 
        bounds.0 as u32, 
        bounds.1 as u32, 
        ColorType::Gray(8)
    )?;

    // Returning unit type () which is the same as `void` in C++
    Ok(())
}

fn main() -> i32 {
    1
}
