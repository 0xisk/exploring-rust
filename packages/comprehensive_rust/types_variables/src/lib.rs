mod string;
mod fibonacci;

pub fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = interproduct(120, 100, 248);

        println!("result: {}", result);
    }
}
