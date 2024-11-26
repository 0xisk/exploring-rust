use regex::Regex;

fn split_string(input: &str, start: char, end: char) -> Vec<String> {
    let pattern = format!(r"{}.*?{}", start, end); // Create a regex like "a.*?z"
    let re = Regex::new(&pattern).unwrap();

    re.find_iter(input)
        .map(|m| m.as_str().to_string()) // Extract matches as strings
        .collect()
}

fn solution(n: i32) -> i8 {
    let total_binary_gap = 0;
    let number_of_bits = (n as f32 + 1.).log2().ceil();
    println!("number_of_bits = {:?}",number_of_bits);

    let n_binary = format!("{n:b}");
    println!("Number = {:?} has binary = {:?}", n, n_binary);

    let substrings = split_string(&n_binary, '1', '1');
    println!("{:?}", substrings);

    // let n_binary: Vec<&str> = n_binary
    // println!("Number = {:?} has binary = {:?}", n, n_binary);

    // for char in n_binary.chars() {
    //     if (char == '1') {
    //         println!("char: {:?}", char)
    //     } else if (char == '0') {

    //     }
    // }

    total_binary_gap
}

fn main() {
    solution(9);
    println!("____________________________________ = 2");

    solution(529);
    println!("____________________________________ = 4");

    solution(20);
    println!("____________________________________ = 1");

    solution(15);
    println!("____________________________________ = 0");

    solution(32);
    println!("____________________________________ = 0");

    solution(1041);
    println!("____________________________________ = 5");
}
