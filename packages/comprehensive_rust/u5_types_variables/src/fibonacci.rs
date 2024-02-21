fn fib(n: u32) -> u32 {
    if n <= 2 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fib(23);

        println!("result: {}", result);
    }
}
