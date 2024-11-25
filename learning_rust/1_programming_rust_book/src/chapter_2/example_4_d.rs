/// Plotting the Set

/// Given the row and column of a pixel in the outpu image, return the 
/// correspoinding point on the complex plane. 
/// 
/// `bounds` is a pair giving the width and height of the image in pixels
/// `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex
/// plane designating the are our image covers. 
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>
    lower_right: Complex<f64>
) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, 
                            upper_left.im - lower_right.im);

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
        // Why subtraction here? pixel.1 increases as we go down,
        // but the imaginary component increases as we go up.
    }
}

/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide.
/// 
/// If `c` is not a member, returns `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius two centered on the 
/// origin. If `c` seems to be a member (more precisely, if we reached the 
/// iteration limit without being able to prove that `c` is not a member),
/// returns `None`.
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    
    /// This `for` loop simply iterates over the range of integers starting with 
    /// 0 and up to (but not including) limit.
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// argument specify points on the complex plane correspoinding to the upper-
/// left and lower-right of the pixel buffer.
fn render(
    pixels: &mut [u8],
    bonds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: COmplex<f64>,
) {
    assert_eq!(pixels.len() == bonds.0 * bonds.1);

    for row in 0..bounds.1 {
        for column in 0 .. bounds.o {
            let point = pixel_to_point(bounds, (column, row), upper_left. lower_right);

            pixels[row * bounds.0 + column] = 
                match escape_time(point, 255) {
                    None => 0, 
                    Some(count) => 255 - count as u8
                };
        }
    }
}

fn main() -> i32 {
    1
}
