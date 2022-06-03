extern crate num;

use num::Complex;

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
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

fn main() -> i32 {

}